mod ai;
mod capture;
mod commands;
mod scheduler;
mod storage;

use commands::capture::AppState;
use capture::detector::CaptureDetector;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Arc::new(
        storage::database::Database::new().expect("Failed to initialize database"),
    );

    let detector = CaptureDetector::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut("CmdOrCtrl+Shift+Y")
                .expect("Failed to parse shortcut")
                .with_handler(|app, _shortcut, event| {
                    if let tauri_plugin_global_shortcut::ShortcutState::Pressed = event.state {
                        show_main_window(app, None);
                    }
                })
                .build(),
        )
        .manage(AppState { db })
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // --- System Tray ---
            setup_tray(app)?;

            // --- Start capture detector (auto-saves to database) ---
            detector.start(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::capture::save_captured_content,
            commands::storage::get_all_content,
            commands::storage::delete_content,
            commands::report::generate_report,
            commands::report::get_report,
            commands::report::get_all_reports,
            commands::report::submit_feedback,
            commands::preferences::get_settings,
            commands::preferences::update_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::TrayIconBuilder;

    let show = MenuItem::with_id(app, "show", "打开小云", true, None::<&str>)?;
    let report = MenuItem::with_id(app, "report", "生成周报", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &report, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("小云 — 智能信息助手")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "show" => {
                    show_main_window(app, None);
                }
                "report" => {
                    show_main_window(app, Some("report"));
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

fn show_main_window(app: &tauri::AppHandle, tab: Option<&str>) {
    use tauri::Emitter;

    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.set_focus();
        if let Some(tab) = tab {
            let _ = app.emit("navigate-tab", tab);
        }
    }
}
