use anyhow::Result;
use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct EpubBook {
    metadata: EpubMetadata,
    resources: HashMap<String, EpubFile>,
    current_page: usize,
}

// read epub file
#[tauri::command]
pub fn read_epub_file(path: &str) -> Result<EpubBook, String> {

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
    let toc = doc
        .toc
        .clone()
        .into_iter()
        .map(|item| item.label.clone())
        .collect();
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

    save_epub_file(&book);

    Ok(book)
}

// Get a specific page from the epub file
#[tauri::command]
pub fn get_epub_page(path: &str, page_index: usize) -> Result<String, String> {
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

// save the epub file on the default disk location
fn save_epub_file(book: &EpubBook) {
    // TODO: Implement saving logic
    // Save the book to a file or database
    // This is a placeholder function. You can implement the actual saving logic here.
}

fn load_epub_file(path: &str) -> Result<EpubBook, String> {
    // TODO: Implement loading logic
    // Load the book from a file or database
    // This is a placeholder function. You can implement the actual loading logic here.
    println!("Loading EPUB book from path: {}", path);
    Err("Not implemented".to_string())
}

// load the imported epub file previously
#[tauri::command]
pub fn load_imported_epub_file(path: &str) -> Result<Vec<EpubBook>, String> {
    // TODO: Implement the logic to load the imported EPUB file
    // Load the book from a file or database
    // This is a placeholder function. You can implement the actual loading logic here.
    println!("Loading imported EPUB book from path: {}", path);
    Err("Not implemented".to_string())
}