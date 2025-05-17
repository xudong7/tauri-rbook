use crate::model::MenuItem;
use tauri::AppHandle;
use epub::doc::EpubDoc;
use base64::{engine::general_purpose, Engine as _};
use tauri::Manager;
use std::fs;
use std::io::Write;
use std::path::Path;

// 读取封面图片文件，返回base64编码的图片
fn read_cover_file(cover_path: &str) -> Result<String, String> {
    // 读取图片内容
    let file_content = fs::read(cover_path)
        .map_err(|e| format!("Failed to read cover file: {}", e))?;
    
    // Base64编码图片内容
    let base64_image = general_purpose::STANDARD.encode(&file_content);
    
    Ok(base64_image)
}

// 读取epub文件的封面图片 返回base64编码的图片
// 如果保存的封面图片存在，则读取保存的图片
// 否则从epub中提取封面图片
async fn read_epub_cover(epub_path: &str) -> Result<String, String> {
    // 首先检查同目录下是否有保存的cover.jpg
    let parent_dir = Path::new(epub_path).parent().unwrap_or(Path::new(""));
    let cover_path = parent_dir.join("cover.jpg");
    
    // 如果封面图片文件存在，直接读取
    if cover_path.exists() {
        return read_cover_file(cover_path.to_str().unwrap_or(""));
    }
    
    // 否则从epub中提取封面图片
    // let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    // let cover_data = doc.get_cover().unwrap();
    // 否则从epub中提取封面图片
    let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    let cover_data = match doc.get_cover() {
        Some(data) => data,
        None => return Err("读取EPUB文件封面失败".to_string()),
    };
    
    let (image_data, _mime_type) = cover_data;
    // 保存图片到文件
    let mut f = fs::File::create(&cover_path)
        .map_err(|e| format!("Failed to create cover file: {}", e))?;
    f.write_all(&image_data)
        .map_err(|e| format!("Failed to write cover file: {}", e))?;
    
    let base64_image = general_purpose::STANDARD.encode(image_data);
    
    Ok(base64_image)
}

// 获得本地已经有的文件
pub async fn get_all_local_files(app_handle: AppHandle) -> Result<Vec<MenuItem>, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("unable to get dir: {}", e))?;
    let book_dir = app_dir.join("books");
    let mut menu_items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(book_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // 读取目录下的文件
                if let Ok(files) = std::fs::read_dir(path.clone()) {
                    for file_entry in files.flatten() {
                        let file_path = file_entry.path();
                        if file_path.extension().map_or(false, |ext| ext == "epub") {
                            // 读取封面图片
                            if let Ok(cover_image) = read_epub_cover(file_path.to_str().unwrap()).await {
                                menu_items.push(MenuItem {
                                    cover: cover_image,
                                    file_path: file_path.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(menu_items)
}
