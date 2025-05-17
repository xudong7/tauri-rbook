use base64::{engine::general_purpose, Engine as _};
use epub::doc::EpubDoc;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use tauri::AppHandle;
use tauri::Manager;

// 遍历目录，查找 HTML 文件
// 如果找到多个 HTML 文件，优先选择 index.html
fn scan_directory(dir: &Path, html_files: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path, html_files)?;
            } else if let Some(ext) = path.extension() {
                if ext == "html" || ext == "htm" {
                    html_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

// 查找解压目录中的 HTML 文件
pub fn find_html_file(extract_dir: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut html_files = Vec::new();

    // 遍历解压目录，查找 HTML 文件
    scan_directory(Path::new(extract_dir), &mut html_files)?;

    // 如果找到多个 HTML 文件，优先选择 index.html
    if !html_files.is_empty() {
        // 优先返回 index.html
        if let Some(index_html) = html_files.iter().find(|path| {
            let file_name = Path::new(path).file_name().and_then(|name| name.to_str());
            file_name
                .map(|name| name.to_lowercase() == "index.html")
                .unwrap_or(false)
        }) {
            return Ok(Some(index_html.clone()));
        }

        // 否则返回第一个 HTML 文件
        return Ok(Some(html_files[0].clone()));
    }

    Ok(None)
}

// 保存epub文件的封面图片到本地
pub fn save_epub_cover(epub_path: &str, save_path: &str) -> Result<String, String> {
    // 创建保存目录
    if let Some(parent) = Path::new(save_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create cover directory: {}", e))?;
        }
    }

    // 从epub中提取封面 没有则从资源文件夹中读取
    let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    let cover_data = match doc.get_cover() {
        Some(data) => {
            // 检查封面图片大小，如果小于1KB则认为已损坏，使用默认封面
            if data.0.len() < 1024 {
                (
                    use_default_cover().map_err(|e| format!("Failed to read default cover file: {}", e))?,
                    "image/png".to_string(),
                )
            } else {
                data
            }
        }
        None => (
            use_default_cover().map_err(|e| format!("Failed to read default cover file: {}", e))?,
            "image/png".to_string(),
        ),
    };

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

// 读取封面图片文件，返回base64编码的图片
pub fn read_cover_file(cover_path: &str) -> Result<String, String> {
    // 读取图片内容
    let file_content =
        fs::read(cover_path).map_err(|e| format!("Failed to read cover file: {}", e))?;

    // Base64编码图片内容
    let base64_image = general_purpose::STANDARD.encode(&file_content);

    Ok(base64_image)
}

// 使用默认封面图片
pub fn use_default_cover() -> Result<Vec<u8>, String> {
    let default_cover_path = Path::new("resources/default_cover.png");
    if default_cover_path.exists() {
        let file_content = fs::read(&default_cover_path)
            .map_err(|e| format!("Failed to read default cover file: {}", e))?;
        Ok(file_content)
    } else {
        Err("Default cover image not found".to_string())
    }
}

// 将默认封面图片复制到应用程序目录 初始化
pub fn init_default_cover(app_handle: &AppHandle, resource_path: &str) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("unable to get dir: {}", e))?;
    let cover_dir = app_dir.join("cover");
    if !cover_dir.exists() {
        std::fs::create_dir_all(&cover_dir)
            .map_err(|e| format!("Failed to create cover directory: {}", e))?;
    }
    let default_cover_path = cover_dir.join("default_cover.png");
    if !default_cover_path.exists() {
        std::fs::copy(resource_path, default_cover_path)
            .map_err(|e| format!("Failed to copy default cover: {}", e))?;
    }
    Ok(())
}

// 计算文件的 MD5 哈希值
pub fn calculate_md5_hash(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);

    let mut file =
        File::open(path).map_err(|e| format!("Failed to open file for hashing: {}", e))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read file for hashing: {}", e))?;

    let digest = md5::compute(&buffer);

    // 返回 MD5 哈希值的十六进制字符串
    Ok(format!("{:x}", digest))
}
