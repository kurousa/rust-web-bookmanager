use chrono::{DateTime, Utc};
use kernel::model::{
    checkout::{Checkout, CheckoutBook},
    id::{BookId, CheckoutId, UserId},
};
use serde::Serialize;
#[cfg(debug_assertions)]
use utoipa::ToSchema;

/// 貸出情報内の本の情報を返すレスポンス
#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CheckoutBookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}
impl From<CheckoutBook> for CheckoutBookResponse {
    fn from(value: CheckoutBook) -> Self {
        let CheckoutBook {
            book_id,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
        }
    }
}

/// 貸出情報のレスポンス
#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CheckoutResponse {
    pub id: CheckoutId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBookResponse,
}
impl From<Checkout> for CheckoutResponse {
    fn from(value: Checkout) -> Self {
        let Checkout {
            id,
            checked_out_by,
            checked_out_at,
            returned_at,
            book,
        } = value;
        Self {
            id,
            checked_out_by,
            checked_out_at,
            returned_at,
            book: book.into(),
        }
    }
}

/// 貸出情報のレスポンスのリスト
#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CheckoutsResponse {
    pub items: Vec<CheckoutResponse>,
}
impl From<Vec<Checkout>> for CheckoutsResponse {
    fn from(value: Vec<Checkout>) -> Self {
        Self {
            items: value.into_iter().map(CheckoutResponse::from).collect(),
        }
    }
}
