use chrono::{DateTime, Utc};
use kernel::model::{
    book::{Book, CheckoutInfo},
    id::{BookId, CheckoutId, UserId},
    user::{BookOwner, CheckOutUser},
};

/// book レコード型定義
pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
}
impl BookRow {
    pub fn into_book(self, checkout: Option<CheckoutInfo>) -> Book {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            owned_by,
            owner_name,
        } = self;
        Book {
            id: book_id,
            title,
            author,
            isbn,
            description,
            owner: BookOwner {
                id: owned_by,
                name: owner_name,
            },
            checkout_info: checkout,
        }
    }
}
pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}
///貸出し情報を含めた本のレコード型定義
pub struct BookCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub user_name: String,
    pub checked_out_at: DateTime<Utc>,
}
// CheckoutInfo型へ変換するFromトレイトの実装
impl From<BookCheckoutRow> for CheckoutInfo {
    fn from(value: BookCheckoutRow) -> Self {
        let BookCheckoutRow {
            checkout_id,
            book_id: _,
            user_id,
            user_name,
            checked_out_at,
        } = value;
        CheckoutInfo {
            checkout_id,
            checked_out_by: CheckOutUser {
                id: user_id,
                name: user_name,
            },
            checked_out_at,
        }
    }
}
