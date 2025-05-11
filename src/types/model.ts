export interface Book {
  title: string;
  author: string;
  publisher: string;
  year: string;
  isbn: string;
  description: string;
  book_url: string;
  cover_url: string;
}

export interface BookList {
  books: Book[];
}
