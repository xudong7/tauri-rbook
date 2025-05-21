/**
 * 书签信息接口
 */
export interface Mark {
  page: number; // 页码
  width: number; // 创建书签时的窗口宽度
  height: number; // 创建书签时的窗口高度
}

/**
 * 书签集合接口
 */
export interface BookMark {
  book_path: string; // HTML文件路径
  list: Mark[]; // 书签列表
}

/**
 * 表示HTML内容及相关图片的接口
 */
export interface HtmlWithImages {
  html_content: string;
  images: ImageItem[];
  bookmark?: BookMark; // 可选的书签信息
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
 * 菜单项接口
 */
export interface MenuItem {
  cover: string; // 封面图片的base64编码
  file_path: string; // 文件路径
}

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
 * 阅读器样式接口
 */
export interface ReaderStyle {
  font_family: string; // 字体
  font_size: number; // 字体大小
  line_height: number; // 行高
}
