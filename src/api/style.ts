import { invoke } from "@tauri-apps/api/core";

/**
 * 阅读器样式接口
 */
export interface ReaderStyle {
  font_family: string; // 字体
  font_size: number; // 字体大小
  line_height: number; // 行高
}

/**
 * 保存阅读器样式到本地
 * @param fontFamily 字体
 * @param fontSize 字体大小
 * @param lineHeight 行高
 * @returns 保存路径
 */
export const saveReaderStyle = async (
  fontFamily: string,
  fontSize: number,
  lineHeight: number
): Promise<string> => {
  try {
    return await invoke<string>("save_reader_style_command", {
      fontFamily,
      fontSize,
      lineHeight,
    });
  } catch (error) {
    console.error("Error saving reader style:", error);
    throw error;
  }
};

/**
 * 从本地获取阅读器样式
 * @returns 阅读器样式
 */
export const getReaderStyle = async (): Promise<ReaderStyle> => {
  try {
    return await invoke<ReaderStyle>("get_reader_style_command");
  } catch (error) {
    console.error("Error loading reader style:", error);
    throw error;
  }
};
