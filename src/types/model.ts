// 定义类型
export interface BookMetadata {
  title: string;
  creator: string;
  publisher: string;
}

export interface MenuItem {
  cover: string; // path to cover image
  path: string; // file path to the .epub file
  last_opened?: number; // timestamp when the book was last opened
}

export interface TocItem {
  label: string;
  href: string;
  level: number;
  subitems?: TocItem[];
}

export interface ReaderStyle {
  font_family: string;
  font_size: number;
  line_height: number;
}

export interface BookMark {
  book_path: string;
  list: Mark[];
}

export interface Mark {
  page: number;
  content: string; // 书签备注内容
  width: number;
  height: number;
  cfi?: string; // EPUB Content Fragment Identifier for precise location
}
