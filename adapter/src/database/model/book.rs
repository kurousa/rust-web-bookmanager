use kernel::model::{book::Book, id::BookId};

/// book レコード型定義
pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        // パターンマッチによる構造体からの値取り出し
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        // kernelで定義したBook構造体の形式へ変換
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
