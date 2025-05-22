// 定义类型
export interface BookMetadata {
  title: string;
  creator: string;
  publisher: string;
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
