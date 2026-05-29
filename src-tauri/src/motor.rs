//! Motor de reproducción M1 en un único hilo (audio rodio no es `Send`).

use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use diffplayerqc::decoder;
use diffplayerqc::types::{AudioFrame, Channel, ColorMetadata, DecoderCommand, VideoFrame};
use diffplayerqc_core::PlaybackState;
use rodio::{OutputStream, Sink};
use serde::Serialize;
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter};

use crate::puente_viewport::PuenteViewport;
use crate::viewport::{enviar_en_main, EstadoViewport, VistaCompare};

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
}

struct DecoderHandle {
    cmd_tx: Sender<DecoderCommand>,
    frame_rx: Receiver<VideoFrame>,
    audio_rx: Receiver<AudioFrame>,
    meta: ColorMetadata,
    path: String,
    last_frame_pts: Option<f64>,
    ultimo_frame: Option<VideoFrame>,
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
}

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
        })
    }

    fn abrir_archivo(&mut self, canal: Channel, ruta: String) -> anyhow::Result<()> {
        let (cmd_tx, frame_rx, audio_rx, meta) = decoder::spawn_decoder(&ruta)?;
        let handle = DecoderHandle {
            cmd_tx,
            frame_rx,
            audio_rx,
            meta,
            path: ruta,
            last_frame_pts: None,
            ultimo_frame: None,
            ultimo_pts_viewport: None,
        };

        match canal {
            Channel::A => {
                if let Some(old) = self.decoder_a.take() {
                    let _ = old.cmd_tx.send(DecoderCommand::Stop);
                }
                self.playback.duration_a = handle.meta.duration_secs;
                self.decoder_a = Some(handle);
            }
            Channel::B => {
                if let Some(old) = self.decoder_b.take() {
                    let _ = old.cmd_tx.send(DecoderCommand::Stop);
                }
                self.playback.duration_b = handle.meta.duration_secs;
                self.decoder_b = Some(handle);
            }
        }
        self.seek_interno(0.0);
        self.invalidar_cache_viewport();
        Ok(())
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

    fn tick(
        &mut self,
        app: &AppHandle,
        puente: &Arc<PuenteViewport>,
    ) -> SnapshotReproduccion {
        let ahora = Instant::now();
        if self.playback.is_playing {
            let fin = self.playback.tick_clock(ahora);
            if fin {
                self.seek_interno(0.0);
                self.pausar(ahora);
            }
        }
        self.drenar_frames();
        self.publicar_frames_viewport(puente);
        self.drenar_audio();
        self.aplicar_volumenes();
        let snap = self.snapshot();
        let _ = app.emit("playback-tick", &snap);
        snap
    }

    fn drenar_frames(&mut self) {
        let pts = self.playback.current_pts;
        let playing = self.playback.is_playing;
        const TOL: f64 = 0.005;

        // Solo conservamos el último frame del lote (el decoder puede enviar varios por tick).
        let drenar = |dec: &mut DecoderHandle| {
            let mut candidato: Option<VideoFrame> = None;
            while let Ok(frame) = dec.frame_rx.try_recv() {
                let mostrar = if playing {
                    frame.pts <= pts + TOL
                } else {
                    true
                };
                if mostrar {
                    candidato = Some(frame);
                } else {
                    break;
                }
            }
            if let Some(frame) = candidato {
                dec.last_frame_pts = Some(frame.pts);
                dec.ultimo_frame = Some(frame);
            }
        };

        if let Some(d) = &mut self.decoder_a {
            drenar(d);
        }
        if let Some(d) = &mut self.decoder_b {
            drenar(d);
        }

        if !playing {
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
        }
    }

    fn publicar_frames_viewport(&mut self, puente: &Arc<PuenteViewport>) {
        if let Some(d) = &mut self.decoder_a {
            if let Some(f) = d.ultimo_frame.as_ref() {
                if !f.rgba_data.is_empty() && d.ultimo_pts_viewport != Some(f.pts) {
                    d.ultimo_pts_viewport = Some(f.pts);
                    puente.encolar_a(Arc::new(f.rgba_data.clone()), f.width, f.height);
                }
            }
        }
        if let Some(d) = &mut self.decoder_b {
            if let Some(f) = d.ultimo_frame.as_ref() {
                if !f.rgba_data.is_empty() && d.ultimo_pts_viewport != Some(f.pts) {
                    d.ultimo_pts_viewport = Some(f.pts);
                    puente.encolar_b(Arc::new(f.rgba_data.clone()), f.width, f.height);
                }
            }
        }
    }

    fn invalidar_cache_viewport(&mut self) {
        if let Some(d) = &mut self.decoder_a {
            d.ultimo_pts_viewport = None;
        }
        if let Some(d) = &mut self.decoder_b {
            d.ultimo_pts_viewport = None;
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
        }
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
    Snapshot {
        resp: Sender<SnapshotReproduccion>,
    },
    EstablecerVista {
        vista: VistaCompare,
    },
}

fn responder(motor: &MotorReproduccion, resp: Sender<Result<SnapshotReproduccion, String>>) {
    let _ = resp.send(Ok(motor.snapshot()));
}

/// Arranca el hilo del motor y el de ticks; devuelve el canal de órdenes (prioridad sobre ticks).
pub fn iniciar_motor(
    app: AppHandle,
    viewport: Arc<Mutex<EstadoViewport>>,
) -> Sender<OrdenMotor> {
    let (tx_cmd, rx_cmd) = crossbeam_channel::unbounded::<OrdenMotor>();
    let (tx_tick, rx_tick) = crossbeam_channel::unbounded::<()>();
    let puente = PuenteViewport::nuevo(app.clone(), Arc::clone(&viewport));

    let puente_motor = Arc::clone(&puente);
    let app_tick = app.clone();
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
                crossbeam_channel::select_biased! {
                    recv(rx_cmd) -> msg => {
                        let Ok(orden) = msg else { break };
                        match orden {
                            OrdenMotor::Abrir { canal, ruta, resp } => {
                                let out = motor
                                    .abrir_archivo(canal, ruta)
                                    .map(|_| motor.snapshot())
                                    .map_err(|e| e.to_string());
                                let _ = resp.send(out);
                            }
                            OrdenMotor::AlternarPlay { resp } => {
                                motor.alternar_play_pausa();
                                responder(&motor, resp);
                            }
                            OrdenMotor::Seek { pts, resp } => {
                                motor.seek(pts);
                                responder(&motor, resp);
                            }
                            OrdenMotor::StepAdelante { resp } => {
                                motor.step_adelante();
                                responder(&motor, resp);
                            }
                            OrdenMotor::StepAtras { resp } => {
                                motor.step_atras();
                                responder(&motor, resp);
                            }
                            OrdenMotor::Snapshot { resp } => {
                                let _ = resp.send(motor.snapshot());
                            }
                            OrdenMotor::EstablecerVista { vista } => {
                                let vp = Arc::clone(&viewport);
                                let app_clone = app_tick.clone();
                                enviar_en_main(&app_clone, move || {
                                    vp.lock().expect("viewport").establecer_vista(vista);
                                });
                            }
                        }
                    }
                    recv(rx_tick) -> _ => {
                        motor.tick(&app_tick, &puente_motor);
                    }
                }
            }
        })
        .expect("hilo motor");

    thread::Builder::new()
        .name("playback-tick".into())
        .spawn(move || loop {
            thread::sleep(Duration::from_millis(16));
            let _ = tx_tick.send(());
        })
        .expect("hilo tick");

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
