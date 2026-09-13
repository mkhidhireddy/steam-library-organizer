pub mod commands;
pub mod domain;
pub mod steam;
pub mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::sync_library,
            commands::refresh_metadata,
            commands::preview_export,
            commands::apply_export
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
