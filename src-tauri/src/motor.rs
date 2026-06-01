//! Motor de reproducción M1 en un único hilo (audio rodio no es `Send`).

use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use diffplayerqc::analisis_scopes::{self, ScopesFrame};
use diffplayerqc::decoder;
use diffplayerqc::analisis_loudness::DatosEbuR128;
use diffplayerqc::forma_onda::{self, FormaOnda, PICOS_POR_SEGUNDO_DEFECTO};
use diffplayerqc::metricas_video::{
    self, MUESTRAS_POR_SEGUNDO_DEFECTO, PuntoMetrica, SerieMetricasVideo,
};
use diffplayerqc::types::{AudioFrame, Channel, ColorMetadata, DecoderCommand, VideoFrame};
use diffplayerqc_core::{next_frame_repaint_delay, PlaybackState, REPINT_AUDIO_MAX_MS, REPINT_IDLE_MAX_MS};
use rodio::{OutputStream, Sink};
use serde::Serialize;
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter};

use crate::hilo_render::HiloRender;
use crate::puente_viewport::PuenteViewport;
use crate::vista_previa;
use crate::viewport::{EstadoViewport, VistaCompare};

/// Canal lógico expuesto al frontend.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CanalUi {
    A,
    B,
}

impl From<CanalUi> for Channel {
    fn from(c: CanalUi) -> Self {
        match c {
            CanalUi::A => Channel::A,
            CanalUi::B => Channel::B,
        }
    }
}

/// Estado serializable emitido en cada `playback-tick`.
#[derive(Debug, Clone, Serialize)]
pub struct SnapshotReproduccion {
    pub pts_actual: f64,
    pub reproduciendo: bool,
    pub duracion_a: f64,
    pub duracion_b: f64,
    pub ruta_a: Option<String>,
    pub ruta_b: Option<String>,
    pub fps: f64,
    pub nivel_audio_a: f32,
    pub nivel_audio_b: f32,
    /// `true` = canal silenciado (rodio volumen 0).
    pub mute_a: bool,
    pub mute_b: bool,
    /// JPEG canal A (compare en webview).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vista_b64_a: Option<String>,
    /// JPEG canal B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vista_b64_b: Option<String>,
    /// Respaldo: A si existe, si no B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vista_b64: Option<String>,
    #[serde(default)]
    pub vista_ancho: u32,
    #[serde(default)]
    pub vista_alto: u32,
    /// Incrementa solo cuando hay frame JPEG nuevo (evita repintar el canvas en vano).
    #[serde(default)]
    pub vista_seq: u64,
    /// SSIM del fotograma actual A↔B (si ambos cargados y mismo tamaño tras escala).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssim_actual: Option<f32>,
    /// Escaneo offline de métricas en curso.
    #[serde(default)]
    pub escaneando_metricas: bool,
    /// Ruta de decode canal A (`hw:videotoolbox`, `software`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decode_a: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decode_b: Option<String>,
}

/// Payload emitido al frontend cuando termina el escaneo de forma de onda.
#[derive(Debug, Clone, Serialize)]
pub struct FormaOndaEvento {
    pub canal: String,
    pub picos: Vec<f32>,
    pub duracion_secs: f64,
    pub lufs_integrado: f64,
    pub picos_por_segundo: u32,
    #[serde(default)]
    pub lufs_buckets: Vec<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebu: Option<DatosEbuR128>,
}

/// Payload cuando termina (o falla) el escaneo de métricas A↔B.
#[derive(Debug, Clone, Serialize)]
pub struct MetricasVideoEvento {
    pub serie: SerieMetricasVideo,
}

/// Progreso del escaneo offline (0..1).
#[derive(Debug, Clone, Serialize)]
pub struct MetricasProgresoEvento {
    pub fraccion: f32,
}

impl From<(Channel, &FormaOnda)> for FormaOndaEvento {
    fn from((canal, f): (Channel, &FormaOnda)) -> Self {
        Self {
            canal: match canal {
                Channel::A => "a".into(),
                Channel::B => "b".into(),
            },
            picos: f.picos.clone(),
            duracion_secs: f.duracion_secs,
            lufs_integrado: f.lufs_integrado,
            picos_por_segundo: f.picos_por_segundo,
            lufs_buckets: f.lufs_buckets.clone(),
            ebu: f.ebu.clone(),
        }
    }
}

/// Frame JPEG asíncrono hacia el canvas (canal separado del tick ligero).
#[derive(Debug, Clone, Serialize)]
pub struct VistaFrameEvent {
    pub canal: String,
    pub seq: u64,
    pub ancho: u32,
    pub alto: u32,
    pub b64: String,
}

struct TrabajoVistaJpeg {
    canal: &'static str,
    rgba: Arc<Vec<u8>>,
    width: u32,
    height: u32,
    seq: u64,
}

struct DecoderHandle {
    cmd_tx: Sender<DecoderCommand>,
    frame_rx: Receiver<VideoFrame>,
    audio_rx: Receiver<AudioFrame>,
    meta: ColorMetadata,
    path: String,
    last_frame_pts: Option<f64>,
    ultimo_frame: Option<VideoFrame>,
    /// Frame futuro retenido (evita descartarlo del canal bounded).
    frame_siguiente: Option<VideoFrame>,
    /// PTS ya enviado al viewport (evita clonar RGBA en cada tick).
    ultimo_pts_viewport: Option<f64>,
}

struct MotorReproduccion {
    decoder_a: Option<DecoderHandle>,
    decoder_b: Option<DecoderHandle>,
    playback: PlaybackState,
    _audio_stream: &'static OutputStream,
    sink_a: Option<Sink>,
    sink_b: Option<Sink>,
    nivel_audio_a: f32,
    nivel_audio_b: f32,
    vol_a: f32,
    vol_b: f32,
    mute_a: bool,
    mute_b: bool,
    vista_cache_a: Option<VistaCacheCanal>,
    vista_cache_b: Option<VistaCacheCanal>,
    ultimo_pts_vista_a: Option<f64>,
    ultimo_pts_vista_b: Option<f64>,
    ultimo_jpeg: Instant,
    vista_seq: u64,
    ultimo_pts_emitido: f64,
    ultimo_nivel_emit_a: f32,
    ultimo_nivel_emit_b: f32,
    inicializado_emit: bool,
    forma_onda_a: Option<FormaOnda>,
    forma_onda_b: Option<FormaOnda>,
    scopes_cache: Option<ScopesFrame>,
    ultimo_pts_scopes_emitido: Option<f64>,
    metricas_serie: Option<SerieMetricasVideo>,
    metrica_actual: Option<PuntoMetrica>,
    escaneando_metricas: bool,
}

/// Caché JPEG por canal (clave = PTS del frame).
struct VistaCacheCanal {
    pts: f64,
    b64: String,
    ancho: u32,
    alto: u32,
}

/// Ancho máximo RGBA en reproducción Tauri (1080p→1280: ~56 % menos píxeles/upload).
const ANCHO_MAX_REPRODUCCION: u32 = 1280;

impl MotorReproduccion {
    fn nuevo() -> anyhow::Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        let stream = Box::leak(Box::new(stream));
        let sink_a = Sink::try_new(&handle).ok();
        let sink_b = Sink::try_new(&handle).ok();
        if let (Some(sa), Some(sb)) = (&sink_a, &sink_b) {
            sa.set_volume(0.0);
            sb.set_volume(0.0);
        }

        Ok(Self {
            decoder_a: None,
            decoder_b: None,
            playback: PlaybackState::default(),
            _audio_stream: stream,
            sink_a,
            sink_b,
            nivel_audio_a: 0.0,
            nivel_audio_b: 0.0,
            vol_a: 1.0,
            vol_b: 1.0,
            mute_a: true,
            mute_b: true,
            vista_cache_a: None,
            vista_cache_b: None,
            ultimo_pts_vista_a: None,
            ultimo_pts_vista_b: None,
            ultimo_jpeg: Instant::now(),
            vista_seq: 0,
            ultimo_pts_emitido: f64::NAN,
            ultimo_nivel_emit_a: f32::NAN,
            ultimo_nivel_emit_b: f32::NAN,
            inicializado_emit: false,
            forma_onda_a: None,
            forma_onda_b: None,
            scopes_cache: None,
            ultimo_pts_scopes_emitido: None,
            metricas_serie: None,
            metrica_actual: None,
            escaneando_metricas: false,
        })
    }

    fn invalidar_metricas(&mut self) {
        self.metricas_serie = None;
        self.metrica_actual = None;
        self.escaneando_metricas = false;
    }

    fn rutas_par_metricas(&self) -> Option<(String, String)> {
        let a = self.decoder_a.as_ref()?.path.clone();
        let b = self.decoder_b.as_ref()?.path.clone();
        Some((a, b))
    }

    fn actualizar_metrica_instantanea(&mut self) {
        let (da, db) = match (&self.decoder_a, &self.decoder_b) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                self.metrica_actual = None;
                return;
            }
        };
        let (fa, fb) = match (da.ultimo_frame.as_ref(), db.ultimo_frame.as_ref()) {
            (Some(a), Some(b)) if !a.rgba_data.is_empty() && !b.rgba_data.is_empty() => (a, b),
            _ => {
                self.metrica_actual = None;
                return;
            }
        };
        self.metrica_actual = Some(metricas_video::comparar_fotogramas(fa, fb));
    }

    fn invalidar_scopes(&mut self) {
        self.scopes_cache = None;
        self.ultimo_pts_scopes_emitido = None;
    }

    /// Frame RGBA preferido para scopes: canal A si existe, si no B.
    fn frame_para_scopes(&self) -> Option<(&VideoFrame, &'static str)> {
        if let Some(d) = &self.decoder_a {
            if let Some(f) = d.ultimo_frame.as_ref() {
                if !f.rgba_data.is_empty() {
                    return Some((f, "a"));
                }
            }
        }
        if let Some(d) = &self.decoder_b {
            if let Some(f) = d.ultimo_frame.as_ref() {
                if !f.rgba_data.is_empty() {
                    return Some((f, "b"));
                }
            }
        }
        None
    }

    fn actualizar_y_emitir_scopes(&mut self, app: &AppHandle) {
        let Some((frame, canal)) = self.frame_para_scopes() else {
            return;
        };
        let pts = self.playback.current_pts;
        if self
            .ultimo_pts_scopes_emitido
            .map(|p| (p - pts).abs() < 1e-9)
            .unwrap_or(false)
        {
            return;
        }
        let scopes = analisis_scopes::calcular_desde_rgba(
            &frame.rgba_data,
            frame.width,
            frame.height,
            pts,
            canal,
        );
        self.scopes_cache = Some(scopes.clone());
        self.ultimo_pts_scopes_emitido = Some(pts);
        if let Err(e) = app.emit("scopes-actualizados", scopes) {
            log::warn!("scopes-actualizados emit: {e}");
        }
    }

    fn scopes_actuales(&self) -> Option<ScopesFrame> {
        self.scopes_cache.clone()
    }

    fn forma_onda_de_canal(&self, canal: Channel) -> Option<FormaOnda> {
        match canal {
            Channel::A => self.forma_onda_a.clone(),
            Channel::B => self.forma_onda_b.clone(),
        }
    }

    fn guardar_forma_onda(&mut self, canal: Channel, forma: FormaOnda) {
        match canal {
            Channel::A => self.forma_onda_a = Some(forma),
            Channel::B => self.forma_onda_b = Some(forma),
        }
    }

    fn abrir_archivo(&mut self, canal: Channel, ruta: String) -> anyhow::Result<()> {
        let (cmd_tx, frame_rx, audio_rx, mut meta, hw_rx) =
            decoder::spawn_decoder(&ruta, true, Some(ANCHO_MAX_REPRODUCCION))?;
        meta.decode_ruta = decoder::esperar_etiqueta_decode(hw_rx);
        let handle = DecoderHandle {
            cmd_tx,
            frame_rx,
            audio_rx,
            meta,
            path: ruta,
            last_frame_pts: None,
            ultimo_frame: None,
            ultimo_pts_viewport: None,
            frame_siguiente: None,
        };

        match canal {
            Channel::A => {
                if let Some(old) = self.decoder_a.take() {
                    let _ = old.cmd_tx.send(DecoderCommand::Stop);
                }
                self.forma_onda_a = None;
                self.playback.duration_a = handle.meta.duration_secs;
                self.decoder_a = Some(handle);
            }
            Channel::B => {
                if let Some(old) = self.decoder_b.take() {
                    let _ = old.cmd_tx.send(DecoderCommand::Stop);
                }
                self.forma_onda_b = None;
                self.playback.duration_b = handle.meta.duration_secs;
                self.decoder_b = Some(handle);
            }
        }
        self.seek_interno(0.0);
        self.invalidar_cache_viewport();
        self.invalidar_vista_jpeg();
        self.invalidar_scopes();
        self.invalidar_metricas();
        Ok(())
    }

    fn guardar_metricas(&mut self, serie: SerieMetricasVideo) {
        self.metricas_serie = Some(serie);
        self.escaneando_metricas = false;
    }

    fn metricas_actuales(&self) -> Option<SerieMetricasVideo> {
        self.metricas_serie.clone()
    }

    /// Fuerza re-codificar JPEG en el próximo tick (seek, step, abrir).
    fn invalidar_vista_jpeg(&mut self) {
        self.vista_cache_a = None;
        self.vista_cache_b = None;
        self.ultimo_pts_vista_a = None;
        self.ultimo_pts_vista_b = None;
        self.invalidar_scopes();
    }

    fn alternar_play_pausa(&mut self) {
        let ahora = Instant::now();
        if self.playback.is_playing {
            self.pausar(ahora);
        } else {
            self.reproducir(ahora);
        }
    }

    fn reproducir(&mut self, ahora: Instant) {
        self.playback.start_playback(ahora);
        if let Some(s) = &self.sink_a {
            s.play();
        }
        if let Some(s) = &self.sink_b {
            s.play();
        }
        self.enviar_decoders(DecoderCommand::Play);
    }

    fn pausar(&mut self, ahora: Instant) {
        self.playback.pause(ahora);
        if let Some(s) = &self.sink_a {
            s.pause();
        }
        if let Some(s) = &self.sink_b {
            s.pause();
        }
        self.enviar_decoders(DecoderCommand::Pause);
        // Con ritmo_externo el decoder llena el canal antes de procesar Pause;
        // sin vaciar, cada tick mostraría un frame más hasta agotar la cola.
        self.descartar_cola_frames_pendientes();
    }

    fn alternar_mute(&mut self, canal: Channel) {
        match canal {
            Channel::A => self.mute_a = !self.mute_a,
            Channel::B => self.mute_b = !self.mute_b,
        }
        self.aplicar_volumenes();
    }

    fn seek(&mut self, pts: f64) {
        self.seek_interno(pts);
    }

    fn seek_interno(&mut self, pts: f64) {
        let ahora = Instant::now();
        self.playback.seek(pts, ahora);
        self.enviar_decoders(DecoderCommand::Seek(pts));
        self.invalidar_cache_viewport();
    }

    fn step_adelante(&mut self) {
        let fps = self.fps_efectivo();
        let pts = self.playback.step_forward_pts(fps);
        self.seek_interno(pts);
        self.enviar_decoders(DecoderCommand::StepForward);
    }

    fn step_atras(&mut self) {
        let fps = self.fps_efectivo();
        let pts = self.playback.step_back_pts(fps);
        self.seek_interno(pts);
    }

    fn enviar_decoders(&self, cmd: DecoderCommand) {
        if let Some(d) = &self.decoder_a {
            let _ = d.cmd_tx.send(cmd.clone());
        }
        if let Some(d) = &self.decoder_b {
            let _ = d.cmd_tx.send(cmd);
        }
    }

    fn fps_efectivo(&self) -> f64 {
        let fa = self.decoder_a.as_ref().map(|d| d.meta.fps).unwrap_or(25.0);
        let fb = self.decoder_b.as_ref().map(|d| d.meta.fps).unwrap_or(25.0);
        fa.max(fb).max(1.0)
    }

    /// Espera hasta el boundary del siguiente frame (v1 / VLC / mpv).
    fn intervalo_proximo_tick(&self) -> Duration {
        if !self.playback.is_playing {
            return Duration::from_millis(REPINT_IDLE_MAX_MS);
        }
        let fps = self.fps_efectivo();
        let pts = self.playback.pts_at(Instant::now());
        let max_ms = if self.audio_audible() {
            REPINT_AUDIO_MAX_MS
        } else {
            REPINT_IDLE_MAX_MS
        };
        next_frame_repaint_delay(fps, pts, max_ms)
    }

    fn tick(
        &mut self,
        app: &AppHandle,
        puente: &Arc<PuenteViewport>,
        hilo: &HiloRender,
        enc_jpeg: &Sender<TrabajoVistaJpeg>,
    ) -> SnapshotReproduccion {
        if self.playback.is_playing {
            self.drenar_audio();
            let ahora = Instant::now();
            let fin = self.playback.tick_clock(ahora);
            self.drenar_frames_sincronizados();
            self.publicar_frames_viewport(puente);
            self.aplicar_volumenes();
            if fin {
                self.finalizar_reproduccion(Instant::now(), hilo);
            }
        } else {
            self.drenar_frames();
            self.publicar_frames_viewport(puente);
        }

        self.actualizar_metrica_instantanea();
        self.actualizar_y_emitir_scopes(app);

        let mut snap = self.snapshot();
        self.rellenar_vista_en_snapshot_si_toca(&mut snap, hilo, enc_jpeg);
        if self.debe_emitir_playback_tick(&snap) {
            let _ = app.emit("playback-tick", &snap);
        }
        snap
    }

    fn finalizar_reproduccion(&mut self, ahora: Instant, hilo: &HiloRender) {
        self.seek_interno(0.0);
        self.pausar(ahora);
        hilo.establecer_reproduciendo(false);
    }

    fn audio_audible(&self) -> bool {
        (self.sink_a.is_some() && !self.mute_a && self.vol_a > 0.0)
            || (self.sink_b.is_some() && !self.mute_b && self.vol_b > 0.0)
    }

    /// Selección por PTS + descarte de frames obsoletos (mpv/VLC cuando CPU va justa).
    fn drenar_frames_sincronizados(&mut self) {
        let pts = self.playback.current_pts;
        let frame_dur = PlaybackState::duracion_frame(self.fps_efectivo());
        const TOL: f64 = 0.005;

        let drenar = |dec: &mut DecoderHandle| {
            loop {
                let candidato = dec
                    .frame_siguiente
                    .take()
                    .or_else(|| dec.frame_rx.try_recv().ok());

                match candidato {
                    None => break,
                    Some(f) if f.pts + frame_dur < pts - TOL => continue,
                    Some(f) if f.pts <= pts + TOL => {
                        dec.last_frame_pts = Some(f.pts);
                        dec.ultimo_frame = Some(f);
                        dec.frame_siguiente = dec.frame_rx.try_recv().ok();
                        break;
                    }
                    Some(f) => {
                        dec.frame_siguiente = Some(f);
                        break;
                    }
                }
            }
        };

        if let Some(d) = &mut self.decoder_a {
            drenar(d);
        }
        if let Some(d) = &mut self.decoder_b {
            drenar(d);
        }
    }

    /// Limita eventos `playback-tick` al webview (timeline + VU); no afecta al viewport GPU.
    fn debe_emitir_playback_tick(&mut self, snap: &SnapshotReproduccion) -> bool {
        const UMBRAL_PTS: f64 = 0.001;
        const UMBRAL_AUDIO: f32 = 0.02;
        let cambio_pts =
            (snap.pts_actual - self.ultimo_pts_emitido).abs() > UMBRAL_PTS;
        let cambio_audio = (snap.nivel_audio_a - self.ultimo_nivel_emit_a).abs() > UMBRAL_AUDIO
            || (snap.nivel_audio_b - self.ultimo_nivel_emit_b).abs() > UMBRAL_AUDIO;
        if cambio_pts || cambio_audio || !self.inicializado_emit {
            self.ultimo_pts_emitido = snap.pts_actual;
            self.ultimo_nivel_emit_a = snap.nivel_audio_a;
            self.ultimo_nivel_emit_b = snap.nivel_audio_b;
            self.inicializado_emit = true;
            true
        } else {
            false
        }
    }

    /// Vacía el canal del decoder hasta dejar el frame más reciente en `ultimo_frame`.
    /// Espera hasta ~300 ms a que llegue al menos un frame tras seek/step.
    fn esperar_frame_decoder(&mut self, intentos: u32) {
        for _ in 0..intentos {
            self.drenar_frames_agresivo();
            let tiene = self
                .decoder_a
                .as_ref()
                .map(|d| d.ultimo_frame.is_some())
                .unwrap_or(false)
                || self
                    .decoder_b
                    .as_ref()
                    .map(|d| d.ultimo_frame.is_some())
                    .unwrap_or(false);
            if tiene {
                return;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn drenar_frames_agresivo(&mut self) {
        let vaciar = |dec: &mut DecoderHandle| {
            while let Ok(frame) = dec.frame_rx.try_recv() {
                dec.last_frame_pts = Some(frame.pts);
                dec.ultimo_frame = Some(frame);
            }
        };
        if let Some(d) = &mut self.decoder_a {
            vaciar(d);
        }
        if let Some(d) = &mut self.decoder_b {
            vaciar(d);
        }
        if self.playback.is_playing {
            self.drenar_frames_sincronizados();
        }
        // Pausado: `vaciar` ya dejó el frame más reciente; no llamar a `drenar_frames`
        // (descartaría la cola sin necesidad tras step/seek).
    }

    /// Vacía frames en cola sin cambiar `ultimo_frame` (freeze real al pausar).
    fn descartar_cola_frames_pendientes(&mut self) {
        let vaciar = |dec: &mut DecoderHandle| {
            dec.frame_siguiente = None;
            while dec.frame_rx.try_recv().is_ok() {}
        };
        if let Some(d) = &mut self.decoder_a {
            vaciar(d);
        }
        if let Some(d) = &mut self.decoder_b {
            vaciar(d);
        }
    }

    fn drenar_frames(&mut self) {
        let playing = self.playback.is_playing;

        if !playing {
            // Pausado: no avanzar imagen por backlog del decoder (Tauri / ritmo_externo).
            self.descartar_cola_frames_pendientes();
            if let Some(d) = &self.decoder_a {
                if let Some(p) = d.last_frame_pts {
                    self.playback.current_pts = p;
                }
            }
            if let Some(d) = &self.decoder_b {
                if let Some(p) = d.last_frame_pts {
                    self.playback.current_pts = self.playback.current_pts.max(p);
                }
            }
            return;
        }

        let pts = self.playback.current_pts;
        const TOL: f64 = 0.005;

        // Igual que v1: como mucho un frame por decoder y por tick (no saltar frames intermedios).
        let drenar = |dec: &mut DecoderHandle| {
            let mut candidato = dec.frame_siguiente.take();
            if candidato.is_none() {
                candidato = dec.frame_rx.try_recv().ok();
            }

            if let Some(frame) = candidato {
                if frame.pts <= pts + TOL {
                    dec.last_frame_pts = Some(frame.pts);
                    dec.ultimo_frame = Some(frame);
                    dec.frame_siguiente = dec.frame_rx.try_recv().ok();
                } else {
                    dec.frame_siguiente = Some(frame);
                }
            }
        };

        if let Some(d) = &mut self.decoder_a {
            drenar(d);
        }
        if let Some(d) = &mut self.decoder_b {
            drenar(d);
        }
    }

    fn publicar_frames_viewport(&mut self, puente: &Arc<PuenteViewport>) {
        let mut frame_a = None;
        let mut frame_b = None;

        if let Some(d) = &mut self.decoder_a {
            if let Some(f) = d.ultimo_frame.as_ref() {
                if !f.rgba_data.is_empty() && d.ultimo_pts_viewport != Some(f.pts) {
                    d.ultimo_pts_viewport = Some(f.pts);
                    frame_a = Some((Arc::clone(&f.rgba_data), f.width, f.height));
                }
            }
        }
        if let Some(d) = &mut self.decoder_b {
            if let Some(f) = d.ultimo_frame.as_ref() {
                if !f.rgba_data.is_empty() && d.ultimo_pts_viewport != Some(f.pts) {
                    d.ultimo_pts_viewport = Some(f.pts);
                    frame_b = Some((Arc::clone(&f.rgba_data), f.width, f.height));
                }
            }
        }

        puente.subir_y_presentar(frame_a, frame_b);
    }

    fn invalidar_cache_viewport(&mut self) {
        if let Some(d) = &mut self.decoder_a {
            d.ultimo_pts_viewport = None;
            d.frame_siguiente = None;
        }
        if let Some(d) = &mut self.decoder_b {
            d.ultimo_pts_viewport = None;
            d.frame_siguiente = None;
        }
    }

    fn republicar_frames_viewport(
        &mut self,
        puente: &Arc<PuenteViewport>,
        viewport: &Arc<Mutex<EstadoViewport>>,
        hilo: &HiloRender,
    ) {
        self.drenar_frames_agresivo();
        self.actualizar_meta_viewport(viewport, hilo);
        self.invalidar_cache_viewport();
        self.publicar_frames_viewport(puente);
    }

    /// Sincroniza dimensiones de vídeo al hilo render (letterbox como v1 `sync_uniforms`).
    fn actualizar_meta_viewport(
        &self,
        viewport: &Arc<Mutex<EstadoViewport>>,
        hilo: &HiloRender,
    ) {
        let mut ancho = 0u32;
        let mut alto = 0u32;
        if let Some(d) = &self.decoder_a {
            ancho = ancho.max(d.meta.width);
            alto = alto.max(d.meta.height);
        }
        if let Some(d) = &self.decoder_b {
            ancho = ancho.max(d.meta.width);
            alto = alto.max(d.meta.height);
        }
        if ancho < 2 || alto < 2 {
            return;
        }
        if let Ok(mut guard) = viewport.lock() {
            guard.establecer_dimensiones_video(ancho, alto, hilo);
        }
    }

    fn drenar_audio(&mut self) {
        if !self.playback.is_playing {
            return;
        }
        const DECAY: f32 = 0.92;

        const MAX_AUDIO_POR_TICK: usize = 8;
        if let Some(dec) = &mut self.decoder_a {
            if let Some(sink) = &self.sink_a {
                for _ in 0..MAX_AUDIO_POR_TICK {
                    let Ok(audio) = dec.audio_rx.try_recv() else {
                        break;
                    };
                    let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
                    self.nivel_audio_a = (self.nivel_audio_a * DECAY + peak).max(peak).min(1.0);
                    let buf = rodio::buffer::SamplesBuffer::new(
                        audio.channels,
                        audio.sample_rate,
                        audio.samples,
                    );
                    sink.append(buf);
                }
            }
        }
        if let Some(dec) = &mut self.decoder_b {
            if let Some(sink) = &self.sink_b {
                for _ in 0..MAX_AUDIO_POR_TICK {
                    let Ok(audio) = dec.audio_rx.try_recv() else {
                        break;
                    };
                    let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
                    self.nivel_audio_b = (self.nivel_audio_b * DECAY + peak).max(peak).min(1.0);
                    let buf = rodio::buffer::SamplesBuffer::new(
                        audio.channels,
                        audio.sample_rate,
                        audio.samples,
                    );
                    sink.append(buf);
                }
            }
        }
    }

    fn aplicar_volumenes(&self) {
        if let Some(s) = &self.sink_a {
            s.set_volume(if self.mute_a { 0.0 } else { self.vol_a });
        }
        if let Some(s) = &self.sink_b {
            s.set_volume(if self.mute_b { 0.0 } else { self.vol_b });
        }
    }

    fn snapshot(&self) -> SnapshotReproduccion {
        SnapshotReproduccion {
            pts_actual: self.playback.current_pts,
            reproduciendo: self.playback.is_playing,
            duracion_a: self.playback.duration_a,
            duracion_b: self.playback.duration_b,
            ruta_a: self.decoder_a.as_ref().map(|d| d.path.clone()),
            ruta_b: self.decoder_b.as_ref().map(|d| d.path.clone()),
            fps: self.fps_efectivo(),
            nivel_audio_a: self.nivel_audio_a,
            nivel_audio_b: self.nivel_audio_b,
            mute_a: self.mute_a,
            mute_b: self.mute_b,
            ssim_actual: self.metrica_actual.map(|m| m.ssim),
            escaneando_metricas: self.escaneando_metricas,
            decode_a: self.decoder_a.as_ref().map(|d| d.meta.decode_ruta.clone()),
            decode_b: self.decoder_b.as_ref().map(|d| d.meta.decode_ruta.clone()),
            vista_b64_a: None,
            vista_b64_b: None,
            vista_b64: None,
            vista_ancho: 0,
            vista_alto: 0,
            vista_seq: self.vista_seq,
        }
    }

    /// JPEG al webview solo cuando cambia el PTS del frame (async si canvas; omitido si GPU).
    fn rellenar_vista_en_snapshot_si_toca(
        &mut self,
        snap: &mut SnapshotReproduccion,
        hilo: &HiloRender,
        enc_jpeg: &Sender<TrabajoVistaJpeg>,
    ) {
        let pts_a = self
            .decoder_a
            .as_ref()
            .and_then(|d| d.ultimo_frame.as_ref().map(|f| f.pts));
        let pts_b = self
            .decoder_b
            .as_ref()
            .and_then(|d| d.ultimo_frame.as_ref().map(|f| f.pts));
        let cambio_pts =
            pts_a != self.ultimo_pts_vista_a || pts_b != self.ultimo_pts_vista_b;
        snap.vista_seq = self.vista_seq;
        if !cambio_pts {
            return;
        }
        self.ultimo_pts_vista_a = pts_a;
        self.ultimo_pts_vista_b = pts_b;
        self.ultimo_jpeg = Instant::now();
        self.vista_seq = self.vista_seq.saturating_add(1);
        snap.vista_seq = self.vista_seq;

        if let Some(d) = &self.decoder_a {
            snap.vista_ancho = snap.vista_ancho.max(d.meta.width);
            snap.vista_alto = snap.vista_alto.max(d.meta.height);
        }
        if let Some(d) = &self.decoder_b {
            snap.vista_ancho = snap.vista_ancho.max(d.meta.width);
            snap.vista_alto = snap.vista_alto.max(d.meta.height);
        }

        if hilo.gpu_operativo() {
            return;
        }

        self.encolar_jpeg_canal(enc_jpeg, "a", &self.decoder_a);
        self.encolar_jpeg_canal(enc_jpeg, "b", &self.decoder_b);
    }

    fn encolar_jpeg_canal(
        &self,
        enc_jpeg: &Sender<TrabajoVistaJpeg>,
        canal: &'static str,
        decoder: &Option<DecoderHandle>,
    ) {
        let Some(d) = decoder else { return };
        let Some(f) = d.ultimo_frame.as_ref() else { return };
        if f.rgba_data.is_empty() {
            return;
        }
        let trabajo = TrabajoVistaJpeg {
            canal,
            rgba: Arc::clone(&f.rgba_data),
            width: f.width,
            height: f.height,
            seq: self.vista_seq,
        };
        if enc_jpeg.send(trabajo).is_err() {
            log::warn!("cola JPEG vista caída ({canal})");
        }
    }

    fn codificar_vista_canal(
        dec: &DecoderHandle,
        cache: &mut Option<VistaCacheCanal>,
    ) -> Option<(String, u32, u32)> {
        let f = dec.ultimo_frame.as_ref()?;
        if f.rgba_data.is_empty() {
            return None;
        }
        if let Some(c) = cache {
            if (c.pts - f.pts).abs() < 1e-6 {
                return Some((c.b64.clone(), c.ancho, c.alto));
            }
        }
        let (b64, w, h) = vista_previa::codificar_base64_jpeg(&f.rgba_data, f.width, f.height)?;
        *cache = Some(VistaCacheCanal {
            pts: f.pts,
            b64: b64.clone(),
            ancho: w,
            alto: h,
        });
        Some((b64, w, h))
    }

    /// Añade JPEG base64 por canal al snapshot (compare A/B en webview).
    fn rellenar_vista_en_snapshot(&mut self, snap: &mut SnapshotReproduccion) {
        snap.vista_ancho = 0;
        snap.vista_alto = 0;

        if let Some(d) = &self.decoder_a {
            if let Some((b64, w, h)) =
                Self::codificar_vista_canal(d, &mut self.vista_cache_a)
            {
                snap.vista_b64_a = Some(b64);
                snap.vista_ancho = snap.vista_ancho.max(w);
                snap.vista_alto = snap.vista_alto.max(h);
            }
        }
        if let Some(d) = &self.decoder_b {
            if let Some((b64, w, h)) =
                Self::codificar_vista_canal(d, &mut self.vista_cache_b)
            {
                snap.vista_b64_b = Some(b64);
                snap.vista_ancho = snap.vista_ancho.max(w);
                snap.vista_alto = snap.vista_alto.max(h);
            }
        }

        snap.vista_b64 = snap
            .vista_b64_a
            .clone()
            .or_else(|| snap.vista_b64_b.clone());
    }

}

/// Órdenes al hilo del motor (desde comandos Tauri o tick).
pub enum OrdenMotor {
    Abrir {
        canal: Channel,
        ruta: String,
        resp: Sender<Result<SnapshotReproduccion, String>>,
    },
    AlternarPlay {
        resp: Sender<Result<SnapshotReproduccion, String>>,
    },
    Seek {
        pts: f64,
        resp: Sender<Result<SnapshotReproduccion, String>>,
    },
    StepAdelante {
        resp: Sender<Result<SnapshotReproduccion, String>>,
    },
    StepAtras {
        resp: Sender<Result<SnapshotReproduccion, String>>,
    },
    AlternarMute {
        canal: Channel,
        resp: Sender<Result<SnapshotReproduccion, String>>,
    },
    Snapshot {
        resp: Sender<SnapshotReproduccion>,
    },
    EstablecerVista {
        vista: VistaCompare,
    },
    /// Vuelve a enviar el último frame tras crear o mover la overlay (mismo PTS).
    RepublicarViewport,
    /// Resultado del escaneo async de forma de onda (hilo auxiliar).
    FormaOndaLista {
        canal: Channel,
        forma: FormaOnda,
    },
    /// Consulta la forma de onda ya escaneada (puede ser `None` si aún no terminó).
    ObtenerFormaOnda {
        canal: Channel,
        resp: Sender<Option<FormaOnda>>,
    },
    ObtenerScopes {
        resp: Sender<Option<ScopesFrame>>,
    },
    MetricasLista {
        serie: SerieMetricasVideo,
    },
    ObtenerMetricas {
        resp: Sender<Option<SerieMetricasVideo>>,
    },
}

fn iniciar_hilo_jpeg_vista(app: AppHandle) -> Sender<TrabajoVistaJpeg> {
    let (tx, rx) = crossbeam_channel::unbounded::<TrabajoVistaJpeg>();
    thread::Builder::new()
        .name("jpeg-vista".into())
        .spawn(move || {
            while let Ok(primero) = rx.recv() {
                let mut ultimo_a: Option<TrabajoVistaJpeg> = None;
                let mut ultimo_b: Option<TrabajoVistaJpeg> = None;
                let mut encolar = |t: TrabajoVistaJpeg| {
                    if t.canal == "a" {
                        ultimo_a = Some(t);
                    } else {
                        ultimo_b = Some(t);
                    }
                };
                encolar(primero);
                while let Ok(mas) = rx.try_recv() {
                    encolar(mas);
                }
                for trabajo in [ultimo_a, ultimo_b].into_iter().flatten() {
                    if let Some((b64, w, h)) = vista_previa::codificar_base64_jpeg_reproduccion(
                        &trabajo.rgba,
                        trabajo.width,
                        trabajo.height,
                    ) {
                        let payload = VistaFrameEvent {
                            canal: trabajo.canal.to_string(),
                            seq: trabajo.seq,
                            ancho: w,
                            alto: h,
                            b64,
                        };
                        if let Err(e) = app.emit("vista-frame", payload) {
                            log::warn!("vista-frame emit: {e}");
                        }
                    }
                }
            }
        })
        .expect("hilo jpeg-vista");
    tx
}

/// Escaneo offline de audio en hilo auxiliar (no bloquea el motor).
fn iniciar_escaneo_forma_onda(
    canal: Channel,
    ruta: String,
    tx: Sender<OrdenMotor>,
    app: AppHandle,
) {
    thread::Builder::new()
        .name(format!("forma-onda-{}", if canal == Channel::A { "a" } else { "b" }))
        .spawn(move || {
            match forma_onda::escanear(&ruta, PICOS_POR_SEGUNDO_DEFECTO) {
                Ok(forma) => {
                    let evento = FormaOndaEvento::from((canal, &forma));
                    let _ = tx.send(OrdenMotor::FormaOndaLista {
                        canal,
                        forma: forma.clone(),
                    });
                    if let Err(e) = app.emit("forma-onda-lista", evento) {
                        log::warn!("forma-onda-lista emit: {e}");
                    }
                }
                Err(e) => {
                    log::warn!("escaneo forma onda {:?}: {e}", canal);
                    // Avisar al frontend para no dejar "Escaneando…" colgado.
                    let vacia = FormaOnda {
                        picos: Vec::new(),
                        duracion_secs: 0.0,
                        lufs_integrado: f64::NEG_INFINITY,
                        picos_por_segundo: PICOS_POR_SEGUNDO_DEFECTO,
                        lufs_buckets: Vec::new(),
                        ebu: None,
                    };
                    let evento = FormaOndaEvento::from((canal, &vacia));
                    let _ = tx.send(OrdenMotor::FormaOndaLista {
                        canal,
                        forma: vacia,
                    });
                    let _ = app.emit("forma-onda-lista", evento);
                }
            }
        })
        .ok();
}

/// Escaneo SSIM/PSNR offline cuando A y B están cargados.
fn iniciar_escaneo_metricas(
    ruta_a: String,
    ruta_b: String,
    tx: Sender<OrdenMotor>,
    app: AppHandle,
) {
    thread::Builder::new()
        .name("metricas-video".into())
        .spawn(move || {
            let app_prog = app.clone();
            let mut al_progreso = move |fraccion: f32| {
                let payload = MetricasProgresoEvento { fraccion };
                let _ = app_prog.emit("metricas-progreso", payload);
            };
            let resultado = metricas_video::escanear_par(
                &ruta_a,
                &ruta_b,
                MUESTRAS_POR_SEGUNDO_DEFECTO,
                Some(&mut al_progreso),
            );
            match resultado {
                Ok(serie) => {
                    let evento = MetricasVideoEvento {
                        serie: serie.clone(),
                    };
                    let _ = tx.send(OrdenMotor::MetricasLista { serie });
                    let _ = app.emit("metricas-lista", evento);
                }
                Err(e) => {
                    log::warn!("escaneo métricas: {e:#}");
                    let vacia = SerieMetricasVideo::vacia();
                    let _ = tx.send(OrdenMotor::MetricasLista {
                        serie: vacia.clone(),
                    });
                    let _ = app.emit(
                        "metricas-lista",
                        MetricasVideoEvento { serie: vacia },
                    );
                }
            }
        })
        .ok();
}

fn intentar_escaneo_metricas(motor: &mut MotorReproduccion, tx: &Sender<OrdenMotor>, app: &AppHandle) {
    if motor.escaneando_metricas {
        return;
    }
    let Some((ruta_a, ruta_b)) = motor.rutas_par_metricas() else {
        return;
    };
    motor.metricas_serie = None;
    motor.metrica_actual = None;
    motor.escaneando_metricas = true;
    iniciar_escaneo_metricas(ruta_a, ruta_b, tx.clone(), app.clone());
}

fn responder(
    motor: &mut MotorReproduccion,
    resp: Sender<Result<SnapshotReproduccion, String>>,
    hilo: &HiloRender,
    enc_jpeg: &Sender<TrabajoVistaJpeg>,
    app: &AppHandle,
) {
    let mut snap = motor.snapshot();
    motor.ultimo_pts_vista_a = None;
    motor.ultimo_pts_vista_b = None;
    motor.rellenar_vista_en_snapshot_si_toca(&mut snap, hilo, enc_jpeg);
    motor.actualizar_y_emitir_scopes(app);
    let _ = resp.send(Ok(snap));
}

/// Arranca el hilo del motor; devuelve el canal de órdenes.
pub fn iniciar_motor(
    app: AppHandle,
    viewport: Arc<Mutex<EstadoViewport>>,
    hilo_render: Arc<HiloRender>,
) -> Sender<OrdenMotor> {
    let (tx_cmd, rx_cmd) = crossbeam_channel::unbounded::<OrdenMotor>();
    let puente = PuenteViewport::nuevo(Arc::clone(&hilo_render));
    let enc_jpeg = iniciar_hilo_jpeg_vista(app.clone());

    let puente_motor = Arc::clone(&puente);
    let app_tick = app.clone();
    let viewport_motor = Arc::clone(&viewport);
    let hilo_motor = Arc::clone(&hilo_render);
    let enc_motor = enc_jpeg.clone();
    let app_abrir = app.clone();
    let tx_motor_interno = tx_cmd.clone();
    thread::Builder::new()
        .name("motor-reproduccion".into())
        .spawn(move || {
            let mut motor = match MotorReproduccion::nuevo() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("Motor: {e}");
                    return;
                }
            };

            loop {
                let timeout = motor.intervalo_proximo_tick();
                let msg = rx_cmd.recv_timeout(timeout);
                match msg {
                    Ok(orden) => match orden {
                            OrdenMotor::Abrir { canal, ruta, resp } => {
                                let ruta_escaneo = ruta.clone();
                                let out = motor
                                    .abrir_archivo(canal, ruta)
                                    .map(|_| {
                                        for _ in 0..50 {
                                            motor.drenar_frames_agresivo();
                                            let tiene = motor.decoder_a.as_ref().map(|d| d.ultimo_frame.is_some()).unwrap_or(false)
                                                || motor.decoder_b.as_ref().map(|d| d.ultimo_frame.is_some()).unwrap_or(false);
                                            if tiene {
                                                break;
                                            }
                                            thread::sleep(Duration::from_millis(10));
                                        }
                                        motor.actualizar_meta_viewport(&viewport_motor, &hilo_motor);
                                        motor.publicar_frames_viewport(&puente_motor);
                                        if !motor
                                            .decoder_a
                                            .as_ref()
                                            .map(|d| d.ultimo_frame.is_some())
                                            .unwrap_or(false)
                                            && !motor
                                                .decoder_b
                                                .as_ref()
                                                .map(|d| d.ultimo_frame.is_some())
                                                .unwrap_or(false)
                                        {
                                            log::warn!(
                                                "motor: sin frame tras abrir (¿FFmpeg/decodificador?)"
                                            );
                                        }
                                        motor.ultimo_pts_vista_a = None;
                                        motor.ultimo_pts_vista_b = None;
                                        motor.actualizar_y_emitir_scopes(&app_abrir);
                                        let mut snap = motor.snapshot();
                                        motor.rellenar_vista_en_snapshot_si_toca(
                                            &mut snap,
                                            &hilo_motor,
                                            &enc_motor,
                                        );
                                        snap
                                    })
                                    .map_err(|e| e.to_string());
                                if out.is_ok() {
                                    iniciar_escaneo_forma_onda(
                                        canal,
                                        ruta_escaneo,
                                        tx_motor_interno.clone(),
                                        app_abrir.clone(),
                                    );
                                    intentar_escaneo_metricas(
                                        &mut motor,
                                        &tx_motor_interno,
                                        &app_abrir,
                                    );
                                }
                                let _ = resp.send(out);
                            }
                            OrdenMotor::AlternarPlay { resp } => {
                                motor.alternar_play_pausa();
                                hilo_motor.establecer_reproduciendo(motor.playback.is_playing);
                                responder(&mut motor, resp, &hilo_motor, &enc_motor, &app_tick);
                            }
                            OrdenMotor::Seek { pts, resp } => {
                                motor.seek(pts);
                                motor.invalidar_vista_jpeg();
                                motor.esperar_frame_decoder(30);
                                responder(&mut motor, resp, &hilo_motor, &enc_motor, &app_tick);
                            }
                            OrdenMotor::StepAdelante { resp } => {
                                motor.step_adelante();
                                motor.invalidar_vista_jpeg();
                                motor.esperar_frame_decoder(30);
                                responder(&mut motor, resp, &hilo_motor, &enc_motor, &app_tick);
                            }
                            OrdenMotor::StepAtras { resp } => {
                                motor.step_atras();
                                motor.invalidar_vista_jpeg();
                                motor.esperar_frame_decoder(30);
                                responder(&mut motor, resp, &hilo_motor, &enc_motor, &app_tick);
                            }
                            OrdenMotor::AlternarMute { canal, resp } => {
                                motor.alternar_mute(canal);
                                responder(&mut motor, resp, &hilo_motor, &enc_motor, &app_tick);
                            }
                            OrdenMotor::Snapshot { resp } => {
                                let mut snap = motor.snapshot();
                                motor.rellenar_vista_en_snapshot_si_toca(
                                    &mut snap,
                                    &hilo_motor,
                                    &enc_motor,
                                );
                                let _ = resp.send(snap);
                            }
                            OrdenMotor::EstablecerVista { vista } => {
                                if let Ok(mut guard) = viewport_motor.lock() {
                                    guard.establecer_vista(vista, &hilo_motor);
                                }
                            }
                            OrdenMotor::RepublicarViewport => {
                                motor.republicar_frames_viewport(
                                    &puente_motor,
                                    &viewport_motor,
                                    &hilo_motor,
                                );
                            }
                            OrdenMotor::FormaOndaLista { canal, forma } => {
                                motor.guardar_forma_onda(canal, forma);
                            }
                            OrdenMotor::ObtenerFormaOnda { canal, resp } => {
                                let _ = resp.send(motor.forma_onda_de_canal(canal));
                            }
                            OrdenMotor::ObtenerScopes { resp } => {
                                let _ = resp.send(motor.scopes_actuales());
                            }
                            OrdenMotor::MetricasLista { serie } => {
                                motor.guardar_metricas(serie);
                            }
                            OrdenMotor::ObtenerMetricas { resp } => {
                                let _ = resp.send(motor.metricas_actuales());
                            }
                        },
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => {
                        motor.tick(
                            &app_tick,
                            &puente_motor,
                            &hilo_motor,
                            &enc_motor,
                        );
                    }
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                }
            }
        })
        .expect("hilo motor");

    tx_cmd
}

pub fn enviar_y_esperar(
    tx: &Sender<OrdenMotor>,
    orden: impl FnOnce(Sender<Result<SnapshotReproduccion, String>>) -> OrdenMotor,
) -> Result<SnapshotReproduccion, String> {
    let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
    tx.send(orden(resp_tx)).map_err(|e| e.to_string())?;
    resp_rx.recv().map_err(|e| e.to_string())?
}
