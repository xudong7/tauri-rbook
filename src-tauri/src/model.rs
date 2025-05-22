use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpubFile {
    pub cover: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMark {
    pub book_path: String, // Path to the epub
    pub list: Vec<Mark>,   // List of marks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mark {
    pub page: u32,   // Page number
    pub content: String, // mark content
    pub width: u32,  // width of window when mark was created
    pub height: u32, // height of window when mark was created
    pub cfi: String, // EPUB Content Fragment Identifier for precise location
}

impl BookMark {
    pub fn new(book_path: String) -> Self {
        BookMark {
            book_path,
            list: Vec::new(),
        }
    } // 添加书签，如果页面已存在书签则更新
    pub fn add_mark(&mut self, page: u32, content: String, width: u32, height: u32, cfi: String) {
        // 检查是否已有该页面的书签
        if let Some(existing_mark) = self.list.iter_mut().find(|m| m.page == page) {
            existing_mark.content = content;
            existing_mark.width = width;
            existing_mark.height = height;
            existing_mark.cfi = cfi;
        } else {
            // 添加新书签
            self.list.push(Mark {
                page,
                content,
                width,
                height,
                cfi,
            });
        }
    }

    // 移除指定页面的书签
    pub fn remove_mark(&mut self, page: u32) {
        // 移除匹配页码的书签
        self.list.retain(|m| m.page != page);
    }
}

// 阅读器样式结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderStyle {
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f32,
}

impl Default for ReaderStyle {
    fn default() -> Self {
        ReaderStyle {
            font_family: "Noto Serif".to_string(),
            font_size: 18,
            line_height: 1.4,
        }
    }
}
