#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 1. Setup the System Tray Context Menu (so users can actually fully quit)
            let quit_item = MenuItem::with_id(app, "quit", "Quit Teams", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&quit_item])?;

            // 2. Create the System Tray Icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone()) // Uses your app icon
                .menu(&tray_menu)
                .on_menu_event(|app, event| {
                    if event.id == "quit" {
                        app.exit(0); // Fully terminates the app
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // Left-clicking the tray icon restores the window
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("uniquelabel") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // 3. Build the Teams WebView Window
            let data_dir = app.path().app_data_dir().unwrap();
            let webview_window = WebviewWindowBuilder::new(
                app,
                "uniquelabel",
                WebviewUrl::External("https://teams.microsoft.com".parse().unwrap()),
            )
            .title("Microsoft Teams")
            .inner_size(800.0, 600.0)
            .data_directory(data_dir)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36")
            .build()?;

            // 4. Intercept the Window Close Event ("X" click)
            let window_clone = webview_window.clone();
            webview_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    // Stop the window from being destroyed
                    api.prevent_close(); 
                    // Hide it to the background instead
                    let _ = window_clone.hide(); 
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
