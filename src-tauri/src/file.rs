use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

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
