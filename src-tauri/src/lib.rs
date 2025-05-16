mod convert;
mod epub;
mod file;
mod model;
mod tray;
mod menu;

use epub::{get_epub_html_with_images, get_epub_to_html_file};
use menu::get_all_local_files;
use model::{MenuItem, HtmlWithImages};
use tauri::AppHandle;

// 获取HTML文件路径
#[tauri::command]
async fn get_epub_to_html_file_command(
    app_handle: AppHandle,
    path: &str,
) -> Result<String, String> {
    get_epub_to_html_file(app_handle, path).await
}

// 获取HTML的相关图片
#[tauri::command]
async fn get_epub_html_with_images_command(
    app_handle: AppHandle,
    path: &str,
) -> Result<HtmlWithImages, String> {
    get_epub_html_with_images(app_handle, path).await
}

// 获取菜单页面的全部电子书文件 带base64编码的封面和文件路径
#[tauri::command]
async fn get_all_local_files_command(
    app_handle: AppHandle,
) -> Result<Vec<MenuItem>, String> {
    get_all_local_files(app_handle).await
}

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
            get_epub_to_html_file_command,
            get_epub_html_with_images_command,
            get_all_local_files_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
