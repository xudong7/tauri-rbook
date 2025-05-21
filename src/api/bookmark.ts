import { invoke } from "@tauri-apps/api/core";
import { BookMark } from "../types/type";
/**
 * 保存或删除书签
 * @param book_path 书籍HTML文件路径
 * @param page 页码
 * @param width 窗口宽度
 * @param height 窗口高度
 * @param action 操作类型: 默认为0(添加书签), 1(删除书签)
 * @returns 保存书签的文件路径
 */
export const saveBookmark = async (
  book_path: string,
  page: number,
  width: number,
  height: number,
  action?: number
): Promise<string> => {
  try {
    return await invoke<string>("save_bookmark_command", {
      bookPath: book_path,
      page,
      width,
      height,
      action,
    });
  } catch (error) {
    console.error("Error saving bookmark:", error);
    throw error;
  }
};

/**
 * 获取书签
 * @param book_path 书籍HTML文件路径
 * @returns 书签信息
 */
export const getBookmark = async (book_path: string): Promise<BookMark> => {
  try {
    return await invoke<BookMark>("get_bookmark_command", {
      bookPath: book_path,
    });
  } catch (error) {
    console.error("Error getting bookmark:", error);
    throw error;
  }
};
