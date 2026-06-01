//! Backend Tauri v2: motor de reproducción, viewport wgpu e IPC.

mod eventos_qc_servicio;
mod hilo_render;
mod motor;
mod puente_viewport;
mod vista_previa;
mod viewport;

use std::sync::{Arc, Mutex};

use diffplayerqc_core::{EventoQc, RegistroEventosQc};
use eventos_qc_servicio::{directorio_eventos, parsear_filtro_tipo, ServicioEventosQc};
use hilo_render::HiloRender;
use motor::{enviar_y_esperar, iniciar_motor, CanalUi, OrdenMotor, SnapshotReproduccion};
use diffplayerqc::analisis_scopes::ScopesFrame;
use diffplayerqc::forma_onda::FormaOnda;
use diffplayerqc::metricas_video::SerieMetricasVideo;
use viewport::{EstadoViewport, RectViewport, VistaCompare};
use tauri::Manager;

pub struct EstadoApp {
    pub tx_motor: crossbeam_channel::Sender<OrdenMotor>,
}

#[tauri::command]
fn obtener_estado(estado: tauri::State<'_, EstadoApp>) -> Result<SnapshotReproduccion, String> {
    let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
    estado
        .tx_motor
        .send(OrdenMotor::Snapshot { resp: resp_tx })
        .map_err(|e| e.to_string())?;
    resp_rx.recv().map_err(|e| e.to_string())
}

#[tauri::command]
fn abrir_video(
    estado: tauri::State<'_, EstadoApp>,
    canal: CanalUi,
    ruta: String,
) -> Result<SnapshotReproduccion, String> {
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::Abrir {
        canal: canal.into(),
        ruta,
        resp,
    })
}

/// Respaldo si algo invoca `abrir_dialogo` por IPC (el flujo habitual usa `open` en el frontend).
#[tauri::command]
fn abrir_dialogo(
    app: tauri::AppHandle,
    estado: tauri::State<'_, EstadoApp>,
    canal: CanalUi,
) -> Result<Option<SnapshotReproduccion>, String> {
    use tauri_plugin_dialog::DialogExt;

    let _ = viewport::ocultar_overlay(&app);
    if let Some(main) = app.get_webview_window("main") {
        let _ = main.set_focus();
    }

    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    let app_dialogo = app.clone();
    app.run_on_main_thread(move || {
        app_dialogo
            .dialog()
            .file()
            .add_filter("Vídeo", &["mp4", "mov", "mkv", "mxf", "avi", "webm"])
            .pick_file(move |path| {
                let _ = tx.send(path);
            });
    })
    .map_err(|e| e.to_string())?;

    let ruta = rx
        .recv()
        .map_err(|_| "no se pudo completar el diálogo de archivos".to_string())?;

    let Some(path) = ruta else {
        return Ok(None);
    };

    let ruta_str = path
        .into_path()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .into_owned();

    let snap = enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::Abrir {
        canal: canal.into(),
        ruta: ruta_str,
        resp,
    })?;
    Ok(Some(snap))
}

#[tauri::command]
fn alternar_play(estado: tauri::State<'_, EstadoApp>) -> Result<SnapshotReproduccion, String> {
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::AlternarPlay { resp })
}

#[tauri::command]
fn seek(estado: tauri::State<'_, EstadoApp>, pts: f64) -> Result<SnapshotReproduccion, String> {
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::Seek { pts, resp })
}

#[tauri::command]
fn step_adelante(estado: tauri::State<'_, EstadoApp>) -> Result<SnapshotReproduccion, String> {
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::StepAdelante { resp })
}

#[tauri::command]
fn step_atras(estado: tauri::State<'_, EstadoApp>) -> Result<SnapshotReproduccion, String> {
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::StepAtras { resp })
}

#[tauri::command]
fn alternar_mute_audio(
    estado: tauri::State<'_, EstadoApp>,
    canal: CanalUi,
) -> Result<SnapshotReproduccion, String> {
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::AlternarMute {
        canal: canal.into(),
        resp,
    })
}

#[tauri::command]
fn ocultar_viewport(
    app: tauri::AppHandle,
) -> Result<(), String> {
    viewport::ocultar_overlay(&app).map_err(|e: tauri::Error| e.to_string())
}

#[tauri::command]
fn sincronizar_viewport(
    app: tauri::AppHandle,
    estado: tauri::State<'_, EstadoApp>,
    viewport: tauri::State<'_, Arc<Mutex<EstadoViewport>>>,
    hilo: tauri::State<'_, Arc<HiloRender>>,
    rect: RectViewport,
) -> Result<(), String> {
    let vp = viewport.inner().clone();
    let hilo_ref = hilo.inner().clone();
    let app_main = app.clone();
    let tx_motor = estado.tx_motor.clone();
    viewport::enviar_en_main(&app, move || {
        let republicar = match vp.lock() {
            Ok(mut guard) => match guard.sincronizar_recto(&app_main, &hilo_ref, rect) {
                Ok(()) => true,
                Err(e) => {
                    log::error!("sincronizar_viewport: {e}");
                    false
                }
            },
            Err(e) => {
                log::error!("sincronizar_viewport: lock: {e}");
                false
            }
        };
        // Republicar frames tras soltar el lock (evita deadlock con el motor).
        if republicar {
            let _ = tx_motor.send(OrdenMotor::RepublicarViewport);
        }
    });
    Ok(())
}

#[tauri::command]
fn establecer_vista_compare(
    estado: tauri::State<'_, EstadoApp>,
    vista: VistaCompare,
) -> Result<(), String> {
    estado
        .tx_motor
        .send(OrdenMotor::EstablecerVista { vista })
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn obtener_scopes(estado: tauri::State<'_, EstadoApp>) -> Result<Option<ScopesFrame>, String> {
    let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
    estado
        .tx_motor
        .send(OrdenMotor::ObtenerScopes { resp: resp_tx })
        .map_err(|e| e.to_string())?;
    resp_rx.recv().map_err(|e| e.to_string())
}

#[tauri::command]
fn obtener_forma_onda(
    estado: tauri::State<'_, EstadoApp>,
    canal: CanalUi,
) -> Result<Option<FormaOnda>, String> {
    let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
    estado
        .tx_motor
        .send(OrdenMotor::ObtenerFormaOnda {
            canal: canal.into(),
            resp: resp_tx,
        })
        .map_err(|e| e.to_string())?;
    resp_rx.recv().map_err(|e| e.to_string())
}

#[tauri::command]
fn obtener_metricas_video(
    estado: tauri::State<'_, EstadoApp>,
) -> Result<Option<SerieMetricasVideo>, String> {
    let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
    estado
        .tx_motor
        .send(OrdenMotor::ObtenerMetricas { resp: resp_tx })
        .map_err(|e| e.to_string())?;
    resp_rx.recv().map_err(|e| e.to_string())
}

#[tauri::command]
fn actualizar_proyecto_eventos(
    app: tauri::AppHandle,
    servicio: tauri::State<'_, ServicioEventosQc>,
    ruta_a: Option<String>,
    ruta_b: Option<String>,
) -> Result<RegistroEventosQc, String> {
    servicio.establecer_proyecto(
        &app,
        ruta_a.as_deref(),
        ruta_b.as_deref(),
    )
}

#[tauri::command]
fn listar_eventos(
    servicio: tauri::State<'_, ServicioEventosQc>,
    filtro_tipo: Option<String>,
) -> Result<Vec<EventoQc>, String> {
    let filtro = parsear_filtro_tipo(filtro_tipo)?;
    servicio.listar(filtro)
}

#[tauri::command]
fn crear_evento(
    app: tauri::AppHandle,
    servicio: tauri::State<'_, ServicioEventosQc>,
    tipo: String,
    pts_secs: f64,
    titulo: String,
    descripcion: Option<String>,
) -> Result<EventoQc, String> {
    let tipo = parsear_filtro_tipo(Some(tipo))?
        .ok_or_else(|| "tipo de evento obligatorio".to_string())?;
    servicio.crear_evento(&app, tipo, pts_secs, titulo, descripcion)
}

#[tauri::command]
fn crear_nota(
    app: tauri::AppHandle,
    servicio: tauri::State<'_, ServicioEventosQc>,
    evento_id: u64,
    texto: String,
    pts_secs: f64,
) -> Result<EventoQc, String> {
    servicio.crear_nota(&app, evento_id, texto, pts_secs)
}

#[tauri::command]
fn eliminar_evento(
    app: tauri::AppHandle,
    servicio: tauri::State<'_, ServicioEventosQc>,
    id: u64,
) -> Result<bool, String> {
    servicio.eliminar_evento(&app, id)
}

#[tauri::command]
fn seek_a_evento(
    estado: tauri::State<'_, EstadoApp>,
    servicio: tauri::State<'_, ServicioEventosQc>,
    id: u64,
) -> Result<SnapshotReproduccion, String> {
    let pts = servicio
        .pts_de_evento(id)?
        .ok_or_else(|| format!("evento {id} no encontrado"))?;
    enviar_y_esperar(&estado.tx_motor, |resp| OrdenMotor::Seek { pts, resp })
}

#[tauri::command]
fn exportar_metricas_csv(estado: tauri::State<'_, EstadoApp>) -> Result<String, String> {
    let (resp_tx, resp_rx) = crossbeam_channel::bounded(1);
    estado
        .tx_motor
        .send(OrdenMotor::ObtenerMetricas { resp: resp_tx })
        .map_err(|e| e.to_string())?;
    let serie = resp_rx.recv().map_err(|e| e.to_string())?;
    match serie {
        Some(s) => Ok(diffplayerqc::metricas_video::exportar_csv(&s)),
        None => Err("No hay métricas escaneadas (carga A y B)".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let _ = env_logger::Builder::from_env(
                env_logger::Env::default().default_filter_or("diffplayerqc_tauri=info,warn"),
            )
            .try_init();
            // Recrear overlay si quedó una ventana hija de una sesión anterior.
            if let Some(vieja) = app.get_webview_window("viewport") {
                let _ = vieja.close();
            }
            if let Some(vieja) = app.get_window("viewport") {
                let _ = vieja.close();
            }
            let viewport = Arc::new(Mutex::new(EstadoViewport::nuevo()));
            app.manage(viewport.clone());
            let hilo_render = HiloRender::iniciar(app.handle().clone());
            app.manage(Arc::clone(&hilo_render));
            let tx = iniciar_motor(app.handle().clone(), viewport, hilo_render);
            app.manage(EstadoApp { tx_motor: tx });
            let dir_eventos = directorio_eventos(app.handle())?;
            app.manage(ServicioEventosQc::nuevo(dir_eventos));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            obtener_estado,
            abrir_video,
            abrir_dialogo,
            alternar_play,
            seek,
            step_adelante,
            step_atras,
            alternar_mute_audio,
            sincronizar_viewport,
            ocultar_viewport,
            establecer_vista_compare,
            obtener_forma_onda,
            obtener_scopes,
            obtener_metricas_video,
            exportar_metricas_csv,
            actualizar_proyecto_eventos,
            listar_eventos,
            crear_evento,
            crear_nota,
            eliminar_evento,
            seek_a_evento,
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}
