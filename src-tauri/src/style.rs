use crate::model::ReaderStyle;
use std::fs;
use tauri::AppHandle;
use tauri::Manager;

// 保存样式配置到本地存储
pub async fn save_style_to_local_storage(
    app_handle: &AppHandle,
    style: &ReaderStyle,
) -> Result<String, String> {
    // 获取应用数据目录
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Unable to get app data directory: {}", e))?;

    // 确保配置目录存在
    let config_dir = app_dir.join("config");
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    // 样式配置文件路径
    let style_file_path = config_dir.join("reader_style.json");

    // 将样式序列化为JSON
    let json_data =
        serde_json::to_string(style).map_err(|e| format!("Failed to serialize style: {}", e))?;

    // 写入文件
    fs::write(&style_file_path, json_data)
        .map_err(|e| format!("Failed to write style to file: {}", e))?;

    println!("Style saved to: {}", style_file_path.display());

    // 返回配置文件路径
    Ok(style_file_path.to_string_lossy().to_string())
}

// 从本地存储加载样式配置
pub async fn load_style_from_local_storage(app_handle: &AppHandle) -> Result<ReaderStyle, String> {
    // 获取应用数据目录
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Unable to get app data directory: {}", e))?;

    // 样式文件路径
    let style_file_path = app_dir.join("config").join("reader_style.json");

    // 如果文件不存在，返回默认样式
    if !style_file_path.exists() {
        println!("Style file not found, using default style");
        return Ok(ReaderStyle::default());
    }

    // 读取文件内容
    let json_data = fs::read_to_string(&style_file_path)
        .map_err(|e| format!("Failed to read style file: {}", e))?;

    // 反序列化JSON数据为ReaderStyle结构
    let style: ReaderStyle = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to deserialize style: {}", e))?;

    Ok(style)
}
