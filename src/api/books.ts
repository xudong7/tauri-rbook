import { invoke } from '@tauri-apps/api/core';
import type { BookList } from '../types/model';

/**
 * 使用关键字搜索书籍
 * @param keyword 搜索关键字
 * @returns 书籍列表
 */
export async function searchBooks(keyword: string): Promise<BookList> {
  try {
    const result = await invoke('search_book_with', { keyword }) as BookList;
    return result;
  } catch (error) {
    console.error('搜索书籍失败:', error);
    throw error;
  }
}
