use crate::{
    extractor::AuthorizedUser,
    model::book::{
        BookListQuery, BookResponse, CreateBookRequest, PaginatedBookResponse, UpdateBookRequest,
        UpdateBookRequestWithIds,
    },
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{book::event::DeleteBook, id::BookId};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

/// 蔵書登録
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/v1/books",
        responses (
            (status = 201, description = "蔵書登録成功"),
            (status = 400, description = "リクエストパラメータ不正"),
            (status = 401, description = "認証エラー"),
        ),
        request_body = CreateBookRequest,
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .book_repository()
        .create(req.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
}

/// 蔵書一覧取得
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books",
        responses (
            (status = 200, description = "蔵書一覧取得成功", body = PaginatedBookResponse),
            (status = 400, description = "リクエストパラメータ不正",),
            (status = 401, description = "認証エラー",),
        ),
        params(
            ("limit" = i64, Query, description = "取得件数"),
            ("offset" = i64, Query, description = "取得開始位置"),
        )
    )
)]
pub async fn show_book_list(
    _user: AuthorizedUser,
    Query(query): Query<BookListQuery>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<PaginatedBookResponse>> {
    query.validate(&())?;

    registry
        .book_repository()
        .find_all(query.into())
        .await
        .map(PaginatedBookResponse::from)
        .map(Json)
}

/// ID指定蔵書取得
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/books/{book_id}",
        responses (
            (status = 200, description = "蔵書取得成功", body = BookResponse),
            (status = 401, description = "認証エラー"),
            (status = 404, description = "蔵書が見つからない場合"),
        ),
        params(
            ("book_id" = BookId, Path, description = "蔵書ID"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
// スパン生成
#[tracing::instrument(
    // スパンに含めない情報を指定
    skip(_user, registry),
    // スパンに含める情報に対し加工を行う場合
    fields(
        book_id = %book_id.to_string(),
        user_id = %_user.user.id.to_string(),
    )
)]
pub async fn show_book(
    _user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    tracing::info!("show_book called");
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::NotFoundError("The book_id book not found".into())),
        })
}

/// 蔵書更新
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/books/{book_id}",
        responses (
            (status = 200, description = "蔵書更新成功"),
            (status = 400, description = "リクエストパラメータ不正"),
            (status = 401, description = "認証エラー"),
            (status = 404, description = "蔵書が見つからない場合"),
        ),
        params(
            ("book_id" = BookId, Path, description = "更新対象の蔵書ID"),
        ),
        request_body = UpdateBookRequest,
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn update_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    let update_book = UpdateBookRequestWithIds::new(book_id, user.id(), req);

    registry
        .book_repository()
        .update(update_book.into())
        .await
        .map(|_| StatusCode::OK)
}

/// 蔵書削除
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        delete,
        path = "/api/v1/books/{book_id}",
        responses (
            (status = 204, description = "蔵書削除成功"),
            (status = 401, description = "認証エラー"),
            (status = 404, description = "蔵書が見つからない場合"),
        ),
        params(
            ("book_id" = BookId, Path, description = "削除対象の蔵書ID"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let delete_book = DeleteBook {
        book_id,
        requested_user: user.id(),
    };

    registry
        .book_repository()
        .delete(delete_book)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}
