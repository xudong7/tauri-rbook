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
}

// 图片项，包含图片路径和base64编码的内容
#[derive(serde::Serialize)]
pub struct ImageItem {
    pub path: String,      // 图片在HTML中的相对路径
    pub content: String,   // base64编码的图片内容
    pub mime_type: String, // 图片的MIME类型
}
