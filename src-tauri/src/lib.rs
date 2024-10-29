// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let _ = app.handle().plugin(tauri_plugin_positioner::init());

            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            let icon = app.default_window_icon().unwrap().clone();

            TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray_handle, event| {
                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
                    match event {
                        TrayIconEvent::Click {
                            id,
                            position,
                            rect,
                            button,
                            button_state,
                        } => {
                            println!("left click pressed and released");
                            let app = tray_handle.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.as_ref().window().move_window(Position::TrayCenter);
                            }
                        }
                        TrayIconEvent::DoubleClick {
                            id,
                            position,
                            rect,
                            button,
                        } => {}
                        TrayIconEvent::Enter { id, position, rect } => {}
                        TrayIconEvent::Move { id, position, rect } => {}
                        TrayIconEvent::Leave { id, position, rect } => {}
                        _ => todo!(),
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
