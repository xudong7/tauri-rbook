mod file;
mod tray;
use file::{
    load_all_local_epub_files, read_epub_file_content, save_file_and_return_local_path, EpubFile,
};
use tauri::AppHandle;
use tray::setup_tray;

#[tauri::command]
async fn save_file_and_return_local_path_command(
    app_handle: AppHandle,
    origin_path: String,
) -> Result<EpubFile, String> {
    save_file_and_return_local_path(&app_handle, &origin_path).await
}

#[tauri::command]
async fn load_all_local_epub_files_command(app_handle: AppHandle) -> Result<Vec<EpubFile>, String> {
    load_all_local_epub_files(&app_handle).await
}

#[tauri::command]
async fn read_epub_file_content_command(file_path: String) -> Result<Vec<u8>, String> {
    read_epub_file_content(&file_path).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // setup the tray icon
            setup_tray(app).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_file_and_return_local_path_command,
            load_all_local_epub_files_command,
            read_epub_file_content_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
