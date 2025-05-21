import { invoke } from "@tauri-apps/api/core";
import { HtmlWithImages, MenuItem } from "../types/type";

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
    return await invoke<HtmlWithImages>(
      "get_epub_html_with_images_multiple_command",
      {
        paths,
      }
    );
  } catch (error) {
    console.error("Error getting HTML with images for multiple files:", error);
    throw error;
  }
};

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
