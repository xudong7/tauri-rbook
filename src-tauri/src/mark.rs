use crate::model::BookMark;
use std::fs;
use std::path::Path;

pub async fn save_bookmark_to_local_storage(bookmark: &BookMark) -> Result<String, String> {
    // 将书签以 JSON 格式保存到本地存储
    // /com.rbook.app/books/xxxxxxx/xxxx.epub
    let book_path = &bookmark.book_path;

    // 获取epub文件所在的目录
    let epub_dir = match Path::new(book_path).parent() {
        Some(dir) => dir,
        None => return Err("Failed to get parent directory from book path".to_string()),
    };

    // 在epub文件同级目录下创建mark.json
    let mark_file_path = epub_dir.join("mark.json");
    if !mark_file_path.exists() {
        // 如果文件不存在，创建一个新的文件
        match fs::File::create(&mark_file_path) {
            Ok(_) => {}
            Err(e) => return Err(format!("Failed to create bookmark file: {}", e)),
        };
    }
    // 将书签序列化为 JSON 格式
    let json_data = serde_json::to_string(bookmark)
        .map_err(|e| format!("Failed to serialize bookmark: {}", e))?;
    // 将 JSON 数据写入文件
    fs::write(&mark_file_path, json_data)
        .map_err(|e| format!("Failed to write bookmark to file: {}", e))?;
    println!("Bookmark saved to: {}", mark_file_path.display());

    // 返回json文件的本地路径
    Ok(mark_file_path.to_string_lossy().to_string())
}

pub async fn load_bookmark_from_local_storage(book_path: &str) -> Result<BookMark, String> {
    // 从本地存储加载书签
    let epub_dir = match Path::new(book_path).parent() {
        Some(dir) => dir,
        None => return Err("Failed to get parent directory from book path".to_string()),
    };

    // 在epub文件同级目录下查找mark.json
    let mark_file_path = epub_dir.join("mark.json");
    if !mark_file_path.exists() {
        // 如果书签文件不存在，创建一个新的空书签
        return Ok(BookMark::new(book_path.to_string()));
    }

    // 读取文件内容
    let json_data = fs::read_to_string(&mark_file_path)
        .map_err(|e| format!("Failed to read bookmark file: {}", e))?;
    // 反序列化 JSON 数据为 BookMark 结构体
    let bookmark: BookMark = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to deserialize bookmark: {}", e))?;
    Ok(bookmark)
}
