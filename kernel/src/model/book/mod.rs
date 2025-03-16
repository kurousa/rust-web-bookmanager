use uuid::Uuid;

pub mod event;

/// 蔵書データ
#[derive(Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
