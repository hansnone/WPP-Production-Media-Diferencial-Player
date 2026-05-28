//! Backend Tauri v2 (M0): IPC mínimo y enlace al crate `core`.

use diffplayerqc_core::WorkspaceLayout;

#[tauri::command]
fn workspace_por_defecto() -> String {
    WorkspaceLayout::default().id().to_string()
}

#[tauri::command]
fn version_core() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![workspace_por_defecto, version_core])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}
