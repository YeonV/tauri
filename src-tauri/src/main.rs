#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let quit_tray = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit_tray)
    .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(hide);
  let tray = SystemTray::new().with_menu(tray_menu);

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));

  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);
    tauri::Builder::default()
      .system_tray(tray)
      .menu(menu)
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
