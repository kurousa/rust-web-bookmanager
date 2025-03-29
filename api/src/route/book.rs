use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::{
    book::{delete_book, register_book, show_book, show_book_list, update_book},
    checkout::{checkout_book, checkout_history_by_book, return_book, show_checked_out_list},
};

pub fn build_book_routers() -> Router<AppRegistry> {
    // 蔵書に関するルーティング
    let book_routers = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book))
        .route("/:book_id", put(update_book))
        .route("/:book_id", delete(delete_book));

    // 貸出に関するルーティング
    let checkout_routers = Router::new()
        .route("/checkouts", get(show_checked_out_list))
        .route("/:book_id/checkouts", post(checkout_book))
        .route(
            "/:book_id/checkouts/:checkout_id/returned",
            put(return_book),
        )
        .route("/:book_id/checkout-history", get(checkout_history_by_book));

    Router::new().nest("/books", book_routers.merge(checkout_routers))
}
