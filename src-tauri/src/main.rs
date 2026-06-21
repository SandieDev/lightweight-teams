// 1. Suppress the terminal window on Windows in release mode (ignored by macOS)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 2. Establish localized persistent data directories for cache/cookies
            let data_dir = app.path().app_data_dir().unwrap();

            // 3. Build the Teams WebView Window with persistent storage and spoofed User-Agent
            let _webview_window = WebviewWindowBuilder::new(
                app,
                "uniquelabel",
                WebviewUrl::External("https://teams.microsoft.com/v2".parse().unwrap()),
            )
            .title("Microsoft Teams")
            .inner_size(1024.0, 768.0)
            .data_directory(data_dir)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36")
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
