use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    pub id: String,
    pub text: Option<String>,
}

// 用于返回HTML内容及相关图片的结构
#[derive(serde::Serialize)]
pub struct HtmlWithImages {
    pub html_content: String,
    pub images: Vec<ImageItem>,
    pub bookmark: Option<BookMark>, // 书签信息
}

// 图片项，包含图片路径和base64编码的内容
#[derive(serde::Serialize)]
pub struct ImageItem {
    pub path: String,      // 图片在HTML中的相对路径
    pub content: String,   // base64编码的图片内容
    pub mime_type: String, // 图片的MIME类型
}

// 返回的HTML内容和图片，菜单展示已有电子书，展示封面
#[derive(serde::Serialize)]
pub struct MenuItem {
    pub cover: String, // 封面图片的base64编码
    pub file_path: String, // 文件路径
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMark {
    pub book_path: String, // Path to the html
    pub list: Vec<Mark>,   // List of marks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mark {
    pub page: u32,   // Page number
    pub width: u32,  // width of window when mark was created
    pub height: u32, // height of window when mark was created
}

impl BookMark {
    pub fn new(book_path: String) -> Self {
        BookMark {
            book_path,
            list: Vec::new(),
        }
    }

    // 添加书签，如果页面已存在书签则更新
    pub fn add_mark(&mut self, page: u32, width: u32, height: u32) {
        // 检查是否已有该页面的书签
        if let Some(existing_mark) = self.list.iter_mut().find(|m| m.page == page) {
            // 更新已有书签的宽高
            existing_mark.width = width;
            existing_mark.height = height;
        } else {
            // 添加新书签
            self.list.push(Mark { page, width, height });
        }
    }

    // 移除指定页面的书签
    pub fn remove_mark(&mut self, page: u32) {    
        // 移除匹配页码的书签
        self.list.retain(|m| m.page != page);
    }
}
