use crate::file::{calculate_md5_hash, compare_if_has_exists, generate_random_string};
use crate::model::{EpubBook, EpubFile, EpubMetadata, SpineItem, TocItem};
use anyhow::Result;
use epub::doc::EpubDoc;
use std::collections::HashMap;
use std::path::Path;
use tauri::{AppHandle, Manager};

// read epub file from the given path
#[tauri::command]
pub fn read_epub_file(app_handle: AppHandle, path: &str) -> Result<EpubBook, String> {
    // save the epub file and get returned path
    let local_path = save_epub_file(app_handle, path)?;
    let local_path = local_path.as_str();

    println!("local_path: {}", local_path);

    // load the epub file by given path
    load_epub_file_by_given_path(local_path).map_err(|e| format!("Failed to load epub file: {}", e))
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

// save the epub file to the default directory
// if the file already exists, return the existing path
fn save_epub_file(app_handle: AppHandle, origin_path: &str) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("unable to get dir: {}", e))?;

    // Create books directory if it doesn't exist
    let books_dir = app_dir.join("books");
    std::fs::create_dir_all(&books_dir)
        .map_err(|e| format!("Failed to create books directory: {}", e))?;

    // generate md5 hash of origin file
    let origin_hash = calculate_md5_hash(origin_path)?;

    // compare hash with existing files
    // scan books dir
    if let Some(existing_path) = compare_if_has_exists(&books_dir, &origin_hash) {
        // if file exists, return local path
        return Ok(existing_path);
    }

    // if not, copy file to books directory
    // Create a unique directory for this book
    let book_dir_name = generate_random_string(10);
    let book_dir = books_dir.join(&book_dir_name);
    std::fs::create_dir_all(&book_dir)
        .map_err(|e| format!("Failed to create book directory: {}", e))?;

    // Keep the original file name if possible, or use a generic name
    let origin_file_name = Path::new(origin_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("book.epub");

    let dest_path = book_dir.join(origin_file_name);

    // Copy the file from origin path to the new path
    std::fs::copy(origin_path, &dest_path).map_err(|e| format!("Failed to copy file: {}", e))?;

    // Save the hash to a file inside the book directory
    let hash_file_path = book_dir.join("file_hash.txt");
    std::fs::write(hash_file_path, origin_hash)
        .map_err(|e| format!("Failed to write hash file: {}", e))?;

    // return local path
    Ok(dest_path.to_string_lossy().to_string())
}

// load epub file by given path
// return EpubBook
fn load_epub_file_by_given_path(path: &str) -> Result<EpubBook, String> {
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
    let toc: Vec<TocItem> = doc
        .toc
        .clone()
        .into_iter()
        .map(|item| TocItem {
            label: item.label.clone(),
            content: item.content.to_string_lossy().to_string(),
            children: item
                .children
                .into_iter()
                .map(|child| TocItem {
                    label: child.label.clone(),
                    content: child.content.to_string_lossy().to_string(),
                    children: Vec::new(),
                    play_order: child.play_order,
                })
                .collect(),
            play_order: item.play_order,
        })
        .collect();

    let spine: Vec<SpineItem> = doc
        .spine
        .iter()
        .map(|item| SpineItem {
            idref: item.idref.clone(),
            id: item.id.clone(),
            properties: item.properties.clone(),
            linear: item.linear,
        })
        .collect();

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


// load all default epub files
#[tauri::command]
fn load_all_default_epub_files(app_handle: AppHandle) -> Result<Vec<EpubBook>, String> {
    // TODO: implement this function
    Ok(vec![])
}