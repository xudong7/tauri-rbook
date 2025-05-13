use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpubFile {
    pub content: Vec<u8>,
    pub mime: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpubMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub toc: Vec<TocItem>,
    pub spine: Vec<SpineItem>,
    pub cover_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TocItem {
    pub label: String,
    pub content: String,
    pub children: Vec<TocItem>,
    pub play_order: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpineItem {
    pub idref: String,
    pub id: Option<String>,
    pub properties: Option<String>,
    pub linear: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpubBook {
    pub metadata: EpubMetadata,
    pub resources: HashMap<String, EpubFile>,
    pub current_page: usize,
}