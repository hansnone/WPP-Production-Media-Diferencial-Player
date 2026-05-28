//! Backend Tauri v2: motor de reproducción M1 e IPC.

mod motor;

use motor::{enviar_y_esperar, iniciar_motor, CanalUi, OrdenMotor, SnapshotReproduccion};
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let tx = iniciar_motor(app.handle().clone());
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
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}
