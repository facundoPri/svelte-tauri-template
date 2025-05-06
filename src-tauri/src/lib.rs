use panel::WebviewWindowExt;
use tauri::Manager;
use tauri_nspanel::ManagerExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

mod command;
mod panel;

pub const SPOTLIGHT_LABEL: &str = "main";
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            command::show,
            command::hide
        ])
        .plugin(tauri_nspanel::init())
        .setup(move |app| {
            let handle = app.app_handle();

            let window = handle.get_webview_window(SPOTLIGHT_LABEL).unwrap();

            let _ = window.to_spotlight_panel()?;

            Ok(())
        })
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(Shortcut::new(Some(Modifiers::ALT), Code::Space))
                .unwrap()
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Pressed
                        && shortcut.matches(Modifiers::ALT, Code::Space)
                    {
                        let window = app.get_webview_window(SPOTLIGHT_LABEL).unwrap();

                        let panel = app.get_webview_panel(SPOTLIGHT_LABEL).unwrap();

                        if panel.is_visible() {
                            panel.order_out(None);
                        } else {
                            let center = false;

                            if center {
                                window.center_at_cursor_monitor().unwrap();
                            }

                            panel.show();
                        }
                    }
                })
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
