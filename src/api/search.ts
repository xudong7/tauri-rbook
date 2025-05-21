import { invoke } from "@tauri-apps/api/core";
import { BookSearchResult } from "./types";

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
