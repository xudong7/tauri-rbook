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
    return await invoke<HtmlWithImages>("get_epub_html_with_images_command", { path });
  } catch (error) {
    console.error("Error getting HTML with images:", error);
    throw error;
  }
};
