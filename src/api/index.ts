import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export interface EpubFile {
  content: number[];
  mime: string;
  path: string;
}

export interface EpubMetadata {
  title: string | null;
  author: string | null;
  description: string | null;
  language: string | null;
  publisher: string | null;
  toc: string[];
  spine: string[];
  cover_id: string | null;
}

export interface EpubBook {
  metadata: EpubMetadata;
  resources: Record<string, EpubFile>;
  current_page: number;
}

/**
 * Opens a file dialog to select an EPUB file and reads its contents
 */
export const openEpubFile = async (): Promise<{ path: string; book: EpubBook } | null> => {
  // Open file dialog
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'EPUB',
      extensions: ['epub']
    }]
  });
  
  if (!selected || Array.isArray(selected)) {
    return null;
  }
  
  const path = selected as string;
  
  // Read the EPUB file
  const book = await invoke('read_epub_file', { path }) as EpubBook;
  
  return { path, book };
};

/**
 * Gets the content of a specific page from an EPUB file
 */
export const getEpubPage = async (path: string, pageIndex: number): Promise<string> => {
  return await invoke('get_epub_page', { path, pageIndex }) as string;
};

/**
 * Converts binary content to base64
 */
export const binaryToBase64 = (content: number[]): string => {
  const uint8Array = new Uint8Array(content);
  let binaryString = '';
  uint8Array.forEach(byte => {
    binaryString += String.fromCharCode(byte);
  });
  return btoa(binaryString);
};

/**
 * Gets the data URL for a resource in the EPUB book
 */
export const getResourceDataUrl = (book: EpubBook, path: string): string => {
  if (!book || !book.resources[path]) return '';
  
  const resource = book.resources[path];
  return `data:${resource.mime};base64,${binaryToBase64(resource.content)}`;
};

/**
 * Processes HTML content from an EPUB page to replace relative URLs with data URLs
 */
export const processEpubHtml = (html: string, book: EpubBook): string => {
  // Replace relative URLs with data URLs
  const parser = new DOMParser();
  const doc = parser.parseFromString(html, 'text/html');
  
  // Process images
  doc.querySelectorAll('img').forEach(img => {
    const src = img.getAttribute('src');
    if (src && !src.startsWith('data:') && !src.startsWith('http')) {
      const dataUrl = getResourceDataUrl(book, src);
      if (dataUrl) {
        img.setAttribute('src', dataUrl);
      }
    }
  });
  
  // Process stylesheets
  doc.querySelectorAll('link[rel="stylesheet"]').forEach(link => {
    const href = link.getAttribute('href');
    if (href && !href.startsWith('data:') && !href.startsWith('http')) {
      const dataUrl = getResourceDataUrl(book, href);
      if (dataUrl) {
        link.setAttribute('href', dataUrl);
      }
    }
  });
  
  return doc.documentElement.outerHTML;
};
