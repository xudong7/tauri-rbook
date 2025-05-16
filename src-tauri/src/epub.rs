use crate::convert::{download_converted_file, upload_epub_to_fileformat_api};
use crate::file::{calculate_md5_hash, find_html_file};
use crate::model::{HtmlWithImages, ImageItem};
use base64::{engine::general_purpose, Engine as _};
use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;
use std::path::Path;
use tauri::AppHandle;
use tauri::Manager;

// 保存epub文件的封面图片到本地
fn save_epub_cover(epub_path: &str, save_path: &str) -> Result<String, String> {
    // 创建保存目录
    if let Some(parent) = Path::new(save_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create cover directory: {}", e))?;
        }
    }

    // 从epub中提取封面
    let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    let cover_data = doc.get_cover().unwrap();

    let (image_data, _mime_type) = cover_data;

    // 保存图片到文件
    let mut f =
        fs::File::create(save_path).map_err(|e| format!("Failed to create cover file: {}", e))?;

    f.write_all(&image_data)
        .map_err(|e| format!("Failed to write cover file: {}", e))?;

    // 同时返回base64编码，以便在前端显示
    let base64_image = general_purpose::STANDARD.encode(image_data);

    Ok(base64_image)
}

// 调用函数，传入epub文件路径，得到转换后的html文件路径
pub async fn get_epub_to_html_file(app_handle: AppHandle, path: &str) -> Result<String, String> {
    // 获取epub文件名和哈希值
    let epub_file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    // 计算文件的MD5哈希值作为目录名，确保不同文件使用不同目录
    let file_hash = match calculate_md5_hash(path) {
        Ok(hash) => hash,
        Err(e) => return Err(format!("Failed to calculate file hash: {}", e)),
    };

    // 获取应用数据目录，如 C:\Users\XU Dong\AppData\Roaming\com.rbook.app
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("unable to get dir: {}", e))?;
    let book_dir = app_dir.join("books").join(&file_hash);
    let epub_copy_path = book_dir.join(epub_file_name).to_string_lossy().to_string();
    let zip_output_path = book_dir
        .join(format!("{}.zip", epub_file_name))
        .to_string_lossy()
        .to_string();
    let extract_dir = book_dir.join("extracted").to_string_lossy().to_string();

    println!("Using app data directory: {}", book_dir.display()); // 确保目录存在
    if let Err(e) = fs::create_dir_all(&book_dir) {
        return Err(format!("Failed to create books directory: {}", e));
    }

    // 保存文件哈希值，便于以后查询
    let hash_file_path = book_dir.join("file_hash.txt");
    if !hash_file_path.exists() {
        if let Err(e) = fs::write(&hash_file_path, &file_hash) {
            println!("Warning: Failed to write hash file: {}", e);
        }
    }

    // 保存原始EPUB文件
    if !Path::new(&epub_copy_path).exists() {
        if let Err(e) = fs::copy(path, &epub_copy_path) {
            println!("Warning: Failed to copy EPUB file: {}", e);
        }
    }

    // 保存封面图片
    let cover_image_path = book_dir.join("cover.jpg").to_string_lossy().to_string();
    if !Path::new(&cover_image_path).exists() {
        if let Err(e) = save_epub_cover(path, &cover_image_path) {
            println!("Warning: Failed to save cover image: {}", e);
        }
    }

    // 检查是否已经转换过，如果已存在HTML文件就直接返回
    if Path::new(&extract_dir).exists() {
        if let Ok(Some(file)) = find_html_file(&extract_dir) {
            // 检查文件是否存在
            if Path::new(&file).exists() {
                println!("Using existing HTML file: {}", file);
                return Ok(file);
            }
        }
    }

    // 上传 EPUB 文件到 API
    let response = match upload_epub_to_fileformat_api(path).await {
        Ok(response) => response,
        Err(e) => return Err(format!("Failed to upload EPUB file: {}", e)),
    };

    // 下载并解压转换后的文件
    if let Err(e) = download_converted_file(&response.id, &zip_output_path, &extract_dir).await {
        return Err(format!("Failed to download converted file: {}", e));
    }

    // 查找解压后的 HTML 文件
    let html_file = match find_html_file(&extract_dir) {
        Ok(Some(file)) => file,
        Ok(None) => return Err("No HTML file found in the extracted directory".to_string()),
        Err(e) => return Err(format!("Error finding HTML file: {}", e)),
    };

    // 返回 HTML 文件路径
    Ok(html_file)
}

// 获取HTML内容和相关图片
pub async fn get_epub_html_with_images(
    app_handle: AppHandle,
    path: &str,
) -> Result<HtmlWithImages, String> {
    // 首先获取HTML文件路径
    let html_file_path = get_epub_to_html_file(app_handle, path).await?;

    // 读取HTML内容
    let html_content = match std::fs::read_to_string(&html_file_path) {
        Ok(content) => content,
        Err(e) => return Err(format!("Failed to read HTML file: {}", e)),
    };

    // 查找images文件夹
    let html_dir = Path::new(&html_file_path).parent().unwrap_or(Path::new(""));
    let images_dir = html_dir.join("images");

    let mut images = Vec::new();

    // 如果images文件夹存在，读取其中的所有图片
    if images_dir.exists() && images_dir.is_dir() {
        let read_dir = match std::fs::read_dir(&images_dir) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(format!("Failed to read images directory: {}", e)),
        };

        for entry in read_dir {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                if file_path.is_file() {
                    // 图片的相对路径，相对于HTML文件
                    let relative_path = format!(
                        "images/{}",
                        file_path.file_name().unwrap_or_default().to_string_lossy()
                    );

                    // 读取图片内容
                    if let Ok(file_content) = std::fs::read(&file_path) {
                        // 获取MIME类型
                        let mime_type = match file_path.extension().and_then(|ext| ext.to_str()) {
                            Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
                            Some("png") => "image/png".to_string(),
                            Some("gif") => "image/gif".to_string(),
                            Some("svg") => "image/svg+xml".to_string(),
                            Some("webp") => "image/webp".to_string(),
                            _ => "application/octet-stream".to_string(),
                        };

                        // Base64编码图片内容
                        let content = general_purpose::STANDARD.encode(&file_content);

                        // 添加到图片列表
                        images.push(ImageItem {
                            path: relative_path,
                            content,
                            mime_type,
                        });
                    }
                }
            }
        }
    }

    // 返回HTML内容和图片列表
    Ok(HtmlWithImages {
        html_content,
        images,
    })
}
