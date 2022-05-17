#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod chess;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![my_custom_command,chess::check_legal_moves])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() -> String {
    return "Hello, World!".into()
}
