mod file;
mod model;
mod style;
mod tray;

use file::{load_all_local_epub_files, read_epub_file_content, save_file_and_return_local_path};
use model::{EpubFile, ReaderStyle};
use style::{load_style_from_local_storage, save_style_to_local_storage};
use tauri::AppHandle;
use tray::setup_tray;

// 保存文件并返回本地路径
#[tauri::command]
async fn save_file_and_return_local_path_command(
    app_handle: AppHandle,
    origin_path: String,
) -> Result<EpubFile, String> {
    save_file_and_return_local_path(&app_handle, &origin_path).await
}

// 加载本地epub文件
#[tauri::command]
async fn load_all_local_epub_files_command(app_handle: AppHandle) -> Result<Vec<EpubFile>, String> {
    load_all_local_epub_files(&app_handle).await
}

// 读取epub文件内容
#[tauri::command]
async fn read_epub_file_content_command(file_path: String) -> Result<Vec<u8>, String> {
    read_epub_file_content(&file_path).await
}

// 保存阅读器样式
#[tauri::command]
async fn save_reader_style_command(
    app_handle: AppHandle,
    font_family: String,
    font_size: u32,
    line_height: f32,
) -> Result<String, String> {
    let style = ReaderStyle {
        font_family,
        font_size,
        line_height,
    };

    save_style_to_local_storage(&app_handle, &style).await
}

// 获取阅读器样式
#[tauri::command]
async fn get_reader_style_command(app_handle: AppHandle) -> Result<ReaderStyle, String> {
    load_style_from_local_storage(&app_handle).await
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
            save_reader_style_command,
            get_reader_style_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
