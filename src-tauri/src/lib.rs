mod cover;
mod file;
mod mark;
mod model;
mod style;
mod tray;

use cover::init_default_cover;
use file::{
    load_all_local_epub_files, read_epub_file_content, save_file_and_return_local_path,
    update_last_opened,
};
use mark::{load_bookmark_from_local_storage, save_bookmark_to_local_storage};
use model::{BookMark, EpubFile, ReaderStyle};
use style::{load_style_from_local_storage, save_style_to_local_storage};
use tauri::path::BaseDirectory;
use tauri::AppHandle;
use tauri::Manager;
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
    theme: String,
) -> Result<String, String> {
    let style = ReaderStyle {
        font_family,
        font_size,
        line_height,
        theme,
    };

    save_style_to_local_storage(&app_handle, &style).await
}

// 获取阅读器样式
#[tauri::command]
async fn get_reader_style_command(app_handle: AppHandle) -> Result<ReaderStyle, String> {
    load_style_from_local_storage(&app_handle).await
}

// 保存书签，action=0表示添加，action=1表示移除
#[tauri::command]
async fn save_bookmark_command(
    book_path: &str,
    page: u32,
    content: String,
    width: u32,
    height: u32,
    cfi: Option<String>,
    action: Option<u32>,
) -> Result<String, String> {
    // 尝试加载已有的书签，如果不存在则创建新的
    let mut bookmark = match load_bookmark_from_local_storage(book_path).await {
        Ok(bm) => bm,
        Err(_) => BookMark::new(book_path.to_string()),
    };

    match action {
        Some(1) => {
            // 移除书签
            bookmark.remove_mark(page);
        }
        _ => {
            // 默认行为是添加或更新书签
            let cfi_str = cfi.unwrap_or_default();
            bookmark.add_mark(page, content, width, height, cfi_str);
        }
    }

    // 保存到本地
    save_bookmark_to_local_storage(&bookmark).await
}

// 获取书签
#[tauri::command]
async fn get_bookmark_command(book_path: &str) -> Result<BookMark, String> {
    load_bookmark_from_local_storage(book_path).await
}

// 更新最后打开时间
#[tauri::command]
async fn update_last_opened_command(file_path: String) -> Result<(), String> {
    update_last_opened(&file_path).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
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
            save_file_and_return_local_path_command,
            load_all_local_epub_files_command,
            read_epub_file_content_command,
            save_reader_style_command,
            get_reader_style_command,
            save_bookmark_command,
            get_bookmark_command,
            update_last_opened_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
