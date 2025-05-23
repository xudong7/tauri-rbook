use tauri::AppHandle;
use tauri::Manager;
use std::path::Path;
use std::fs;

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
