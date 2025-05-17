mod convert;
mod epub;
mod file;
mod menu;
mod model;
mod search;
mod tray;

use epub::{get_epub_html_with_images, get_epub_to_html_file};
use file::init_default_cover;
use menu::get_all_local_files;
use model::{HtmlWithImages, MenuItem};
use search::{download_certain_online_book, search_online_books_by_keyword, BookSearchResult};
use tauri::path::BaseDirectory;
use tauri::AppHandle;
use tauri::Manager;
use tray::setup_tray;

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
async fn get_all_local_files_command(app_handle: AppHandle) -> Result<Vec<MenuItem>, String> {
    get_all_local_files(app_handle).await
}

// 搜索在线书籍
#[tauri::command]
async fn search_online_books_command(
    keyword: &str,
    page: u32,
) -> Result<Vec<BookSearchResult>, String> {
    search_online_books_by_keyword(keyword, page).await
}

#[tauri::command]
async fn download_online_book_command(
    app_handle: AppHandle,
    book: BookSearchResult,
) -> Result<String, String> {
    download_certain_online_book(app_handle, &book).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 将默认封面图片复制到应用程序目录
            let resource_path = app
                .path()
                .resolve("resources/default_cover.png", BaseDirectory::Resource)?;
            let app_handle = app.handle();
            init_default_cover(&app_handle, &resource_path.to_string_lossy())?;

            // setup the tray icon
            setup_tray(app).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_epub_to_html_file_command,
            get_epub_html_with_images_command,
            get_all_local_files_command,
            search_online_books_command,
            download_online_book_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
