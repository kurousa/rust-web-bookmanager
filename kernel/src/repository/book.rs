use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::model::book::{event::CreateBook, Book};

#[async_trait]
pub trait BookRepository: Send + Sync {
    /// 蔵書レコード作成
    async fn create(&self, event: CreateBook) -> Result<()>;
    /// 蔵書の一覧を取得
    async fn find_all(&self) -> Result<Vec<Book>>;
    /// 蔵書IDを指定して蔵書データを取得
    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>>;
}
