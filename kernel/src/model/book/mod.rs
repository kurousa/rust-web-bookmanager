use crate::model::{
    id::{BookId, CheckoutId},
    user::{BookOwner, CheckOutUser},
};
use chrono::{DateTime, Utc};

pub mod event;

#[derive(Debug)]
pub struct CheckoutInfo {
    pub checkout_id: CheckoutId,
    pub checked_out_by: CheckOutUser,
    pub checked_out_at: DateTime<Utc>,
}

/// 蔵書データ
#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout_info: Option<CheckoutInfo>,
}

#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}
