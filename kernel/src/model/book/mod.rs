use super::id::BookId;

pub mod event;

/// 蔵書データ
#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
