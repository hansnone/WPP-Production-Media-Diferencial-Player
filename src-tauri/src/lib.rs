//! Backend Tauri v2: motor de reproducción, viewport wgpu e IPC.

mod motor;
mod puente_viewport;
mod viewport;

use std::sync::{Arc, Mutex};

use motor::{enviar_y_esperar, iniciar_motor, CanalUi, OrdenMotor, SnapshotReproduccion};
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

#[tauri::command]
async fn abrir_dialogo(
    app: tauri::AppHandle,
    estado: tauri::State<'_, EstadoApp>,
    canal: CanalUi,
) -> Result<Option<SnapshotReproduccion>, String> {
    use tauri_plugin_dialog::DialogExt;

    let ruta = app
        .dialog()
        .file()
        .add_filter("Vídeo", &["mp4", "mov", "mkv", "mxf", "avi", "webm"])
        .blocking_pick_file();

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
fn ocultar_viewport(
    app: tauri::AppHandle,
) -> Result<(), String> {
    viewport::ocultar_overlay(&app).map_err(|e: tauri::Error| e.to_string())
}

#[tauri::command]
fn sincronizar_viewport(
    app: tauri::AppHandle,
    viewport: tauri::State<'_, Arc<Mutex<EstadoViewport>>>,
    rect: RectViewport,
) -> Result<(), String> {
    let vp = viewport.inner().clone();
    let app_main = app.clone();
    viewport::enviar_en_main(&app, move || {
        if let Ok(mut guard) = vp.lock() {
            let _ = guard.sincronizar_recto(&app_main, rect);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let viewport = Arc::new(Mutex::new(EstadoViewport::nuevo()));
            app.manage(viewport.clone());
            let tx = iniciar_motor(app.handle().clone(), viewport);
            app.manage(EstadoApp { tx_motor: tx });
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
            sincronizar_viewport,
            ocultar_viewport,
            establecer_vista_compare,
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}
