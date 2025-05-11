use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct ReadResult {
    content: String,
    filename: String,
}

#[tauri::command]
fn read_markdown_file(path: &str) -> Result<ReadResult, String> {
    // Read the file
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

    // Get the filename
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();

    Ok(ReadResult { content, filename })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![read_markdown_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
