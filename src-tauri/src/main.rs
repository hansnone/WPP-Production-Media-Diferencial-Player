// No permite dead_code en el binario de desarrollo Tauri.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    diffplayerqc_lib::run();
}
