import type { Theme } from "./themeManager";
import { THEME_CONFIGS } from "./themeManager";

/**
 * 书籍内容主题样式定义
 */
export interface BookContentThemeStyle {
  body: Record<string, string>;
  p: Record<string, string>;
  h1: Record<string, string>;
  h2: Record<string, string>;
  h3: Record<string, string>;
  h4: Record<string, string>;
  h5: Record<string, string>;
  h6: Record<string, string>;
  "*": Record<string, string>;
}

/**
 * 浅色模式书籍内容样式
 */
const lightThemeStyle: BookContentThemeStyle = {
  body: {
    "background-color": "#ffffff !important",
    color: "#333333 !important",
  },
  p: {
    color: "#333333 !important",
  },
  h1: {
    color: "#333333 !important",
  },
  h2: {
    color: "#333333 !important",
  },
  h3: {
    color: "#333333 !important",
  },
  h4: {
    color: "#333333 !important",
  },
  h5: {
    color: "#333333 !important",
  },
  h6: {
    color: "#333333 !important",
  },
  "*": {
    color: "#333333 !important",
  },
};

/**
 * 深色模式书籍内容样式
 */
const darkThemeStyle: BookContentThemeStyle = {
  body: {
    "background-color": "#1a1a1a !important",
    color: "#e0e0e0 !important",
  },
  p: {
    color: "#e0e0e0 !important",
  },
  h1: {
    color: "#e0e0e0 !important",
  },
  h2: {
    color: "#e0e0e0 !important",
  },
  h3: {
    color: "#e0e0e0 !important",
  },
  h4: {
    color: "#e0e0e0 !important",
  },
  h5: {
    color: "#e0e0e0 !important",
  },
  h6: {
    color: "#e0e0e0 !important",
  },
  "*": {
    color: "#e0e0e0 !important",
  },
};

/**
 * 护眼模式书籍内容样式
 */
const sepiaThemeStyle: BookContentThemeStyle = {
  body: {
    "background-color": "#f7f3e9 !important",
    color: "#5c4b37 !important",
  },
  p: {
    color: "#5c4b37 !important",
  },
  h1: {
    color: "#5c4b37 !important",
  },
  h2: {
    color: "#5c4b37 !important",
  },
  h3: {
    color: "#5c4b37 !important",
  },
  h4: {
    color: "#5c4b37 !important",
  },
  h5: {
    color: "#5c4b37 !important",
  },
  h6: {
    color: "#5c4b37 !important",
  },
  "*": {
    color: "#5c4b37 !important",
  },
};

/**
 * 所有书籍内容主题样式的映射
 */
const bookContentThemes: Record<Theme, BookContentThemeStyle> = {
  light: lightThemeStyle,
  dark: darkThemeStyle,
  sepia: sepiaThemeStyle,
};

/**
 * 获取指定主题的书籍内容样式
 * @param theme 主题名称
 * @returns 主题样式对象
 */
export const getBookContentTheme = (theme: Theme): BookContentThemeStyle => {
  return bookContentThemes[theme] || lightThemeStyle;
};

/**
 * 获取所有可用的主题名称
 * @returns 主题名称数组
 */
export const getAvailableThemes = (): Theme[] => {
  return THEME_CONFIGS.map(config => config.key);
};

/**
 * 应用书籍内容主题到渲染器
 * @param rendition EPUB渲染器实例
 * @param theme 要应用的主题
 */
export const applyBookContentTheme = (rendition: any, theme: Theme): void => {
  if (!rendition) {
    console.warn("Rendition is not available");
    return;
  }

  const contentColors = getBookContentTheme(theme);

  try {
    // 注册内容主题
    rendition.themes.register("content-theme", contentColors);

    // 应用内容主题
    rendition.themes.select("content-theme");

    console.log("应用书籍内容主题:", theme, contentColors);
  } catch (error) {
    console.error("应用书籍内容主题失败:", error);
  }
};

export default {
  getBookContentTheme,
  getAvailableThemes,
  applyBookContentTheme,
};
