use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    book::{
        event::{CreateBook, DeleteBook, UpdateBook},
        Book, BookListOptions,
    },
    id::{BookId, UserId},
    list::PaginatedList,
};

#[async_trait]
pub trait BookRepository: Send + Sync {
    /// 蔵書レコード作成
    async fn create(&self, event: CreateBook, user_id: UserId) -> AppResult<()>;
    /// 蔵書の一覧を取得
    async fn find_all(&self, options: BookListOptions) -> AppResult<PaginatedList<Book>>;
    /// 蔵書IDを指定して蔵書データを取得
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>>;
    /// 蔵書データを更新
    async fn update(&self, event: UpdateBook) -> AppResult<()>;
    /// 蔵書データを削除
    async fn delete(&self, event: DeleteBook) -> AppResult<()>;
}
