mod epub;
mod tray;
mod file;
mod model;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // setup the tray icon
            tray::setup_tray(app).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            epub::read_epub_file,
            epub::get_epub_page,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
