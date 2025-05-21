use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubFile {
    pub cover: String,
    pub path: String,
}

// 保存封面到本地
// 返回封面保存的路径
async fn read_epub_cover(dir: &str, epub_path: &str) -> Result<String, String> {
    let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    let cover_data = doc.get_cover().unwrap();
    let (image_data, _mime_type) = cover_data;
    let f = File::create(format!("{}/cover.jpg", dir));
    assert!(f.is_ok());
    let mut f = f.unwrap();
    f.write_all(&image_data).map_err(|e| e.to_string())?;
    Ok(format!("{}/cover.jpg", dir))
}

// 加载本地所有的epub文件
pub async fn load_all_local_epub_files(app_handle: &AppHandle) -> Result<Vec<EpubFile>, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let books_dir = app_dir.join("books");

    if !books_dir.exists() {
        return Ok(Vec::new());
    }

    let mut epub_files = Vec::new();

    // /com.rbook.app/books/xxxxxxxx/xxxx.epub
    // 其中xxxxxx为md5的值
    for entry in std::fs::read_dir(books_dir)
        .map_err(|e| format!("Failed to read books directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.extension().map(|ext| ext == "epub").unwrap_or(false) {
            let cover = format!("{}/cover.jpg", path.to_str().unwrap());
            epub_files.push(EpubFile {
                cover,
                path: path.to_str().unwrap().to_string(),
            });
        }
    }

    Ok(epub_files)
}

// 将传来的路径的文件复制到本地
// 返回保存的本地路径
pub async fn save_file_and_return_local_path(
    app_handle: &AppHandle,
    origin_path: &str,
) -> Result<EpubFile, String> {
    let origin_path = Path::new(origin_path);
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let books_dir = app_dir.join("books");
    if !books_dir.exists() {
        std::fs::create_dir_all(&books_dir)
            .map_err(|e| format!("Failed to create books directory: {}", e))?;
    }
    // /com.rbook.app/books/xxxxxxxx/xxxx.epub
    // 其中xxxxxx为md5的值

    // 计算文件的 MD5 哈希值
    let md5_hash = calculate_md5_hash(origin_path.to_str().unwrap()).await?;

    // 创建hash值的文件夹 并将文件复制过去
    let hash_dir = books_dir.join(&md5_hash);
    if !hash_dir.exists() {
        std::fs::create_dir_all(&hash_dir)
            .map_err(|e| format!("Failed to create hash directory: {}", e))?;
    }

    let file_name = origin_path.file_name().ok_or("Failed to get file name")?;
    let dest_path = hash_dir.join(file_name);

    // 如果文件已存在，直接返回路径
    if dest_path.exists() {
        return Ok(EpubFile {
            cover: format!("{}/cover.jpg", hash_dir.to_str().unwrap()),
            path: dest_path.to_str().unwrap().to_string(),
        });
    }

    // 复制文件
    std::fs::copy(origin_path, &dest_path).map_err(|e| format!("Failed to copy file: {}", e))?;
    // 读取封面
    read_epub_cover(hash_dir.to_str().unwrap(), origin_path.to_str().unwrap()).await?;

    // 返回epub文件的路径
    Ok(EpubFile {
        cover: format!("{}/cover.jpg", hash_dir.to_str().unwrap()),
        path: dest_path.to_str().unwrap().to_string(),
    })
}

// 计算文件的 MD5 哈希值
pub async fn calculate_md5_hash(file_path: &str) -> Result<String, String> {
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
