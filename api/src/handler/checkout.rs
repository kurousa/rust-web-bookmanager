use crate::{extractor::AuthorizedUser, model::checkout::CheckoutsResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kernel::model::{
    checkout::event::{CreateCheckout, UpdateReturned},
    id::{BookId, CheckoutId},
};
use registry::AppRegistry;
use shared::error::AppResult;

/// 書籍貸出
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/v1/books/{book_id}/checkouts",
        responses (
            (status = 201, description = "貸出登録成功"),
            (status = 400, description = "リクエストパラメータ不正"),
            (status = 401, description = "認証エラー"),
            (status = 404, description = "指定された書籍が見つからない場合"),
            (status = 422, description = "既に貸出中の場合"),
        ),
        params(
            ("book_id" = BookId, Path, description = "貸出する書籍のID"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn checkout_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let create_checkout_history = CreateCheckout::new(book_id, user.id(), chrono::Utc::now());

    registry
        .check_out_repository()
        .create_checkout(create_checkout_history)
        .await
        .map(|_| StatusCode::CREATED)
}

/// 書籍返却
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/books/{book_id}/checkouts/{checkout_id}",
        responses (
            (status = 200, description = "返却処理成功"),
            (status = 401, description = "認証エラー"),
            (status = 404, description = "指定された貸出が見つからない場合"),
        ),
        params(
            ("book_id" = BookId, Path, description = "返却する書籍のID"),
            ("checkout_id" = CheckoutId, Path, description = "貸出履歴ID"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn return_book(
    user: AuthorizedUser,
    Path((book_id, checkout_id)): Path<(BookId, CheckoutId)>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let update_returned = UpdateReturned::new(checkout_id, book_id, user.id(), chrono::Utc::now());

    registry
        .check_out_repository()
        .update_returned(update_returned)
        .await
        .map(|_| StatusCode::OK)
}

/// 全ての貸出中書籍一覧を表示
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/checkouts",
        responses (
            (status = 200, description = "貸出中書籍一覧取得成功", body = CheckoutsResponse),
            (status = 401, description = "認証エラー"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn show_checked_out_list(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .check_out_repository()
        .find_unreturned_all()
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}

/// 書籍ごとの貸出履歴を表示
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books/{book_id}/checkouts",
        responses (
            (status = 200, description = "書籍の貸出履歴取得成功", body = CheckoutsResponse),
            (status = 401, description = "認証エラー"),
            (status = 404, description = "指定された書籍が見つからない場合"),
        ),
        params(
            ("book_id" = BookId, Path, description = "履歴を取得する書籍のID"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn checkout_history_by_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .check_out_repository()
        .find_history_by_book_id(book_id)
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
