use anyhow::Result;
use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::vec;
mod tray;

#[derive(Debug, Serialize, Deserialize)]
struct EpubFile {
    content: Vec<u8>,
    mime: String,
}

// read epub file
#[tauri::command]
fn read_epub_file(path: &str) -> Result<Vec<EpubFile>, String> {
    let doc = EpubDoc::new(path).unwrap();
    let mut returned: Vec<EpubFile> = vec![];
    let mut doc_clone = EpubDoc::new(path).unwrap();
    doc.resources.keys().for_each(|k| {
        if let Some(content) = doc_clone.get_resource(k) {
            let (content, mime) = content;
            let part = EpubFile {
                content: content.to_vec(),
                mime: mime.to_string(),
            };
            returned.push(part);
        }
    });
    Ok(returned)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // setup the tray icon
            tray::setup_tray(app).unwrap();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![read_epub_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
