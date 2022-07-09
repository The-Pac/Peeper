#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cards,set_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_cards() {}

#[tauri::command]
fn set_card() {}
