use anyhow::Result;
use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::vec;
use std::collections::HashMap;
mod tray;

#[derive(Debug, Serialize, Deserialize)]
struct EpubFile {
    content: Vec<u8>,
    mime: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EpubMetadata {
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    language: Option<String>,
    publisher: Option<String>,
    toc: Vec<String>,
    spine: Vec<String>,
    cover_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EpubBook {
    metadata: EpubMetadata,
    resources: HashMap<String, EpubFile>,
    current_page: usize,
}

// read epub file
#[tauri::command]
fn read_epub_file(path: &str) -> Result<EpubBook, String> {
    let doc = match EpubDoc::new(path) {
        Ok(doc) => doc,
        Err(e) => return Err(format!("Failed to open epub file: {}", e)),
    };
      // Extract metadata
    let title = doc.mdata("title").map(|s| s.to_string());
    let author = doc.mdata("creator").map(|s| s.to_string());
    let description = doc.mdata("description").map(|s| s.to_string());
    let language = doc.mdata("language").map(|s| s.to_string());
    let publisher = doc.mdata("publisher").map(|s| s.to_string());
    
    // Get the table of contents and spine
    let toc = doc.toc.clone().into_iter().map(|item| item.label.clone()).collect();
    let spine = doc.spine.iter().map(|item| item.idref.clone()).collect();
    
    // Get cover ID
    let cover_id = doc.get_cover_id().map(|s| s.to_string());
    
    // Create resources map
    let mut resources = HashMap::new();
    let mut doc_clone = match EpubDoc::new(path) {
        Ok(doc) => doc,
        Err(e) => return Err(format!("Failed to open epub file for resources: {}", e)),
    };
    
    doc.resources.keys().for_each(|k| {
        if let Some(content) = doc_clone.get_resource(k) {
            let (content, mime) = content;
            let part = EpubFile {
                content: content.to_vec(),
                mime: mime.to_string(),
                path: k.clone(),
            };
            resources.insert(k.clone(), part);
        }
    });
    
    // Construct the book
    let metadata = EpubMetadata {
        title,
        author,
        description,
        language,
        publisher,
        toc,
        spine,
        cover_id,
    };
    
    let book = EpubBook {
        metadata,
        resources,
        current_page: 0,
    };
    
    Ok(book)
}

// Get a specific page from the epub file
#[tauri::command]
fn get_epub_page(path: &str, page_index: usize) -> Result<String, String> {
    let mut doc = match EpubDoc::new(path) {
        Ok(doc) => doc,
        Err(e) => return Err(format!("Failed to open epub file: {}", e)),
    };
    
    // Try to navigate to the specified page
    if page_index >= doc.spine.len() {
        return Err(format!("Page index out of range: {}", page_index));
    }
    
    doc.set_current_page(page_index);
      // Get the current page content
    match doc.get_current_str() {
        Some((content, _)) => Ok(content),
        None => Err("Failed to get page content".to_string()),
    }
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
        .invoke_handler(tauri::generate_handler![read_epub_file, get_epub_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
