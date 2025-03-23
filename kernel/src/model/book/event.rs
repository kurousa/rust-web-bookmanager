use crate::model::id::{BookId, UserId};
/// 蔵書作成イベント
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

/// 蔵書更新イベント
pub struct UpdateBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub requested_user: UserId,
}

/// 蔵書削除イベント
pub struct DeleteBook {
    pub book_id: BookId,
    pub requested_user: UserId,
}
