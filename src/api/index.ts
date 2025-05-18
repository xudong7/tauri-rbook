import { invoke } from "@tauri-apps/api/core";

/**
 * 表示HTML内容及相关图片的接口
 */
export interface HtmlWithImages {
  html_content: string;
  images: ImageItem[];
}

/**
 * 图片项接口
 */
export interface ImageItem {
  path: string; // 图片在HTML中的相对路径
  content: string; // base64编码的图片内容
  mime_type: string; // 图片的MIME类型
}

/**
 * 提交EPUB文件路径，获取转换后的HTML文件路径
 * @param path EPUB文件路径
 * @returns HTML文件路径
 */
export const getEpubToHtmlFile = async (path: string): Promise<string> => {
  try {
    return await invoke<string>("get_epub_to_html_file_command", { path });
  } catch (error) {
    console.error("Error converting EPUB to HTML:", error);
    throw error;
  }
};

/**
 * 提交EPUB文件路径，获取HTML内容和相关图片
 * @param path EPUB文件路径
 * @returns HTML内容和相关图片
 */
export const getEpubHtmlWithImages = async (
  path: string
): Promise<HtmlWithImages> => {
  try {
    return await invoke<HtmlWithImages>("get_epub_html_with_images_command", {
      path,
    });
  } catch (error) {
    console.error("Error getting HTML with images:", error);
    throw error;
  }
};

/**
 * 提交多个EPUB文件路径，获取转换后的最后一个HTML文件路径
 * @param paths EPUB文件路径数组
 * @returns 最后一个处理成功的HTML文件路径
 */
export const getEpubToHtmlFiles = async (paths: string[]): Promise<string> => {
  try {
    return await invoke<string>("get_epub_to_html_files_command", { paths });
  } catch (error) {
    console.error("Error converting multiple EPUBs to HTML:", error);
    throw error;
  }
};

/**
 * 提交多个EPUB文件路径，获取最后一个处理成功的HTML内容和相关图片
 * @param paths EPUB文件路径数组
 * @returns 最后一个处理成功的HTML内容和相关图片
 */
export const getEpubHtmlWithImagesMultiple = async (
  paths: string[]
): Promise<HtmlWithImages> => {
  try {
    return await invoke<HtmlWithImages>("get_epub_html_with_images_multiple_command", {
      paths,
    });
  } catch (error) {
    console.error("Error getting HTML with images for multiple files:", error);
    throw error;
  }
};

/**
 * 菜单项接口
 */
export interface MenuItem {
  cover: string; // 封面图片的base64编码
  file_path: string; // 文件路径
}

/**
 * 获取所有本地电子书文件
 * @returns 菜单项列表
 */
export const getAllLocalFiles = async (): Promise<MenuItem[]> => {
  try {
    return await invoke<MenuItem[]>("get_all_local_files_command");
  } catch (error) {
    console.error("Error getting local files:", error);
    throw error;
  }
};

/**
 * 书籍搜索结果接口
 */
export interface BookSearchResult {
  title: string; // 书籍标题
  author: string; // 作者
  cover_image: string; // 封面图片URL
  description: string; // 书籍描述/摘要
  details_url: string; // 书籍详情页面URL
  download_url: string; // 书籍下载链接
}

/**
 * 搜索在线书籍
 * @param keyword 关键词
 * @param page 页码
 * @returns 搜索结果列表
 */
export const searchOnlineBooks = async (
  keyword: string,
  page: number = 1
): Promise<BookSearchResult[]> => {
  try {
    return await invoke<BookSearchResult[]>("search_online_books_command", {
      keyword,
      page,
    });
  } catch (error) {
    console.error("Error searching online books:", error);
    throw error;
  }
};

/**
 * 下载指定的在线书籍
 * @param book 要下载的书籍信息
 * @returns 下载后的文件路径
 */
export const downloadOnlineBook = async (
  book: BookSearchResult
): Promise<string> => {
  try {
    return await invoke<string>("download_online_book_command", { book });
  } catch (error) {
    console.error("Error downloading online book:", error);
    throw error;
  }
};
