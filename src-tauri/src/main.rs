#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod chess;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

  tauri::Builder::default()
      .menu(menu)
      .invoke_handler(tauri::generate_handler![my_custom_command,chess::check_legal_moves])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() -> String {
    return "Hello, World!".into()
}
