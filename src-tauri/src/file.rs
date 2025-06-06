use crate::cover::use_default_cover;
use crate::model::EpubFile;
use epub::doc::EpubDoc;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri::Manager;

// 保存封面到本地
// 返回封面保存的路径
async fn read_epub_cover(dir: &str, epub_path: &str) -> Result<String, String> {
    let mut doc = EpubDoc::new(epub_path).map_err(|e| e.to_string())?;
    let cover_data = match doc.get_cover() {
        Some(data) => {
            // 检查封面图片大小，如果小于1KB则认为已损坏，使用默认封面
            if data.0.len() < 1024 {
                (
                    use_default_cover()
                        .map_err(|e| format!("Failed to read default cover file: {}", e))?,
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
    let f = File::create(format!("{}/cover.jpg", dir));
    assert!(f.is_ok());
    let mut f = f.unwrap();
    f.write_all(&image_data).map_err(|e| e.to_string())?;
    Ok(format!("{}/cover.jpg", dir))
}

// 获取系统当前时间的Unix时间戳
fn get_current_timestamp() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())
        .map(|d| d.as_secs())
}

// 更新打开时间
pub async fn update_last_opened(file_path: &str) -> Result<(), String> {
    let now = get_current_timestamp()?;
    let parent_dir = Path::new(file_path)
        .parent()
        .ok_or_else(|| "Failed to get parent directory".to_string())?;

    let time_file = parent_dir.join(".lastopened");
    std::fs::write(&time_file, now.to_string())
        .map_err(|e| format!("Failed to write last opened time: {}", e))?;

    Ok(())
}

// 获取最后打开时间
fn get_last_opened(dir_path: &Path) -> Option<u64> {
    std::fs::read_to_string(dir_path.join(".lastopened"))
        .ok()
        .and_then(|s| s.parse().ok())
}

// 加载本地所有的epub文件
pub async fn load_all_local_epub_files(app_handle: &AppHandle) -> Result<Vec<EpubFile>, String> {
    // 加载/com.rbook.app/books/xxxxxxx/xxxx.epub 和 /com.rbook.app/books/xxxxxx/cover.jpg
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let books_dir = app_dir.join("books");
    if !books_dir.exists() {
        return Ok(vec![]);
    }
    let mut epub_files = Vec::new();
    // /com.rbook.app/books/xxxxxxxx/xxxx.epub
    // 其中xxxxxx为md5的值
    for hash_dir_entry in std::fs::read_dir(books_dir)
        .map_err(|e| format!("Failed to read books directory: {}", e))?
    {
        let hash_dir_entry =
            hash_dir_entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let hash_dir_path = hash_dir_entry.path();

        // 检查它是否是一个目录
        if hash_dir_path.is_dir() {
            // 遍历MD5哈希目录中的文件
            for file_entry in std::fs::read_dir(&hash_dir_path)
                .map_err(|e| format!("Failed to read hash directory: {}", e))?
            {
                let file_entry =
                    file_entry.map_err(|e| format!("Failed to read file entry: {}", e))?;
                let file_path = file_entry.path();

                // 如果是epub文件
                if file_path
                    .extension()
                    .map(|ext| ext == "epub")
                    .unwrap_or(false)
                {
                    let cover_path = hash_dir_path.join("cover.jpg");
                    // 检查封面文件是否存在
                    if !cover_path.exists() {
                        // 如果不存在，读取epub文件的封面
                        read_epub_cover(
                            hash_dir_path.to_str().unwrap(),
                            file_path.to_str().unwrap(),
                        )
                        .await?;
                    }
                    let last_opened = get_last_opened(&hash_dir_path);

                    epub_files.push(EpubFile {
                        cover: cover_path.to_str().unwrap().to_string(),
                        path: file_path.to_str().unwrap().to_string(),
                        last_opened,
                    });
                }
            }
        }
    }
    // println!("epub_files: {:?}", epub_files);

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
        let last_opened = get_last_opened(&hash_dir);

        return Ok(EpubFile {
            cover: format!("{}/cover.jpg", hash_dir.to_str().unwrap()),
            path: dest_path.to_str().unwrap().to_string(),
            last_opened,
        });
    }

    // 复制文件
    std::fs::copy(origin_path, &dest_path).map_err(|e| format!("Failed to copy file: {}", e))?;
    // 读取封面
    read_epub_cover(hash_dir.to_str().unwrap(), origin_path.to_str().unwrap()).await?;

    // 返回epub文件的路径
    let last_opened = get_last_opened(&hash_dir);

    Ok(EpubFile {
        cover: format!("{}/cover.jpg", hash_dir.to_str().unwrap()),
        path: dest_path.to_str().unwrap().to_string(),
        last_opened,
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

// 读取EPUB文件内容
// 返回二进制数据
pub async fn read_epub_file_content(file_path: &str) -> Result<Vec<u8>, String> {
    let path = Path::new(file_path);
    let mut file = File::open(path).map_err(|e| format!("Failed to open EPUB file: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read EPUB file content: {}", e))?;
    Ok(buffer)
}
