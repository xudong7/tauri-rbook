use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;

// const BASE_URL: &str = "https://digilibraries.com";
const SEARCH_URL: &str = "https://digilibraries.com/search/";
// const DOWNLOAD_URL: &str = "https://digilibraries.com/download/";

// 书籍搜索结果的结构
#[derive(Debug, Serialize, Deserialize)]
pub struct BookSearchResult {
    pub title: String,        // 书籍标题
    pub author: String,       // 作者
    pub cover_image: String,  // 封面图片URL
    pub description: String,  // 书籍描述/摘要
    pub details_url: String,  // 书籍详情页面URL
    pub download_url: String, // 书籍下载链接
}

// 搜索书籍
pub async fn search_online_books_by_keyword(
    keyword: &str,
    page: u32,
) -> Result<Vec<BookSearchResult>, String> {
    // 搜索URL：现在网站使用 GET 方法进行搜索，但需要正确构建 URL
    // 例如： https://digilibraries.com/search/story 或 https://digilibraries.com/search/story/2
    // TODO
    let url = if page == 1 {
        format!("{}/{}", SEARCH_URL, keyword)
    } else {
        format!("{}/{}/{}", SEARCH_URL, keyword, page)
    };

    println!("搜索URL: {}", url);

    let search_list_result: Vec<BookSearchResult> = Vec::new();

    println!("总共找到 {} 个书籍", search_list_result.len());

    // 返回结果
    Ok(search_list_result)
}

// 下载选中的书籍
pub async fn download_certain_online_book(
    app_handle: AppHandle,
    book: &BookSearchResult,
) -> Result<String, String> {
    // 详情URL： https://digilibraries.com/book/stories-by-english-authors-africa-selected-by-scribners
    // BookSearchResult.details_url
    // 从返回的html中解析出下载download_id
    // TODO
    let epub_file_path = format!(
        "{}/epub/{}.epub",
        app_handle.path().app_data_dir().unwrap().display(),
        book.title
    );

    println!("电子书已下载到: {}", epub_file_path);

    Ok(epub_file_path)
}
