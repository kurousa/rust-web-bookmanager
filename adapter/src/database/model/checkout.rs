use kernel::model::{
    checkout::{Checkout, CheckoutBook},
    id::{BookId, CheckoutId, UserId},
};
use sqlx::types::chrono::{DateTime, Utc};

pub struct CheckoutStateRow {
    pub book_id: BookId,
    pub checkout_id: Option<CheckoutId>,
    pub user_id: Option<UserId>,
}

pub struct CheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}
impl From<CheckoutRow> for Checkout {
    fn from(value: CheckoutRow) -> Self {
        let CheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            title,
            author,
            isbn,
        } = value;
        Checkout {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            returned_at: None,
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}

pub struct ReturnedCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<ReturnedCheckoutRow> for Checkout {
    fn from(value: ReturnedCheckoutRow) -> Self {
        let ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            returned_at,
            title,
            author,
            isbn,
        } = value;
        Checkout {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            returned_at: Some(returned_at),
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_checkout_row() {
        let checkout_id = CheckoutId::new();
        let book_id = BookId::new();
        let user_id = UserId::new();
        let now = Utc::now();

        let row = CheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at: now,
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            isbn: "1234567890".to_string(),
        };

        let checkout = Checkout::from(row);

        assert_eq!(checkout.id, checkout_id);
        assert_eq!(checkout.checked_out_by, user_id);
        assert_eq!(checkout.checked_out_at, now);
        assert_eq!(checkout.returned_at, None);
        assert_eq!(checkout.book.book_id, book_id);
        assert_eq!(checkout.book.title, "Test Book");
        assert_eq!(checkout.book.author, "Test Author");
        assert_eq!(checkout.book.isbn, "1234567890");
    }

    #[test]
    fn test_from_returned_checkout_row() {
        let checkout_id = CheckoutId::new();
        let book_id = BookId::new();
        let user_id = UserId::new();
        let now = Utc::now();
        let returned_at = Utc::now();

        let row = ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at: now,
            returned_at,
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            isbn: "1234567890".to_string(),
        };

        let checkout = Checkout::from(row);

        assert_eq!(checkout.id, checkout_id);
        assert_eq!(checkout.checked_out_by, user_id);
        assert_eq!(checkout.checked_out_at, now);
        assert_eq!(checkout.returned_at, Some(returned_at));
        assert_eq!(checkout.book.book_id, book_id);
        assert_eq!(checkout.book.title, "Test Book");
        assert_eq!(checkout.book.author, "Test Author");
        assert_eq!(checkout.book.isbn, "1234567890");
    }

    #[test]
    fn test_from_returned_checkout_row_for_checkout() {
        let checkout_id = CheckoutId::new();
        let book_id = BookId::new();
        let user_id = UserId::new();
        let checked_out_at = Utc::now();
        let returned_at = Utc::now();

        let row = ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            returned_at,
            title: "Test Title".to_string(),
            author: "Test Author".to_string(),
            isbn: "Test ISBN".to_string(),
        };

        let checkout: Checkout = row.into();

        assert_eq!(checkout.id, checkout_id);
        assert_eq!(checkout.checked_out_by, user_id);
        assert_eq!(checkout.checked_out_at, checked_out_at);
        assert_eq!(checkout.returned_at, Some(returned_at));
        assert_eq!(checkout.book.book_id, book_id);
        assert_eq!(checkout.book.title, "Test Title");
        assert_eq!(checkout.book.author, "Test Author");
        assert_eq!(checkout.book.isbn, "Test ISBN");
    }
}
