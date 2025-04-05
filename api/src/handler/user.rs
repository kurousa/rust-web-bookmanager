use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use garde::Validate;
use kernel::model::{id::UserId, user::event::DeleteUser};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::checkout::CheckoutsResponse,
    model::user::{
        CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
        UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserResponse, UsersResponse,
    },
};

/// ユーザー情報を取得する
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/users/me",
        responses (
            (status = 200, description = "ユーザー情報取得成功", body = UserResponse),
            (status = 401, description = "認証エラー"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

/// ユーザー一覧を取得する
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/users",
        responses (
            (status = 200, description = "ユーザー一覧取得成功", body = UsersResponse),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限不足"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn list_users(State(registry): State<AppRegistry>) -> AppResult<Json<UsersResponse>> {
    let users = registry
        .user_repository()
        .find_all()
        .await?
        .into_iter()
        .map(UserResponse::from)
        .collect();
    Ok(Json(UsersResponse { users }))
}

/// パスワードを変更する
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/users/me/password",
        responses (
            (status = 200, description = "パスワード更新成功"),
            (status = 400, description = "リクエストパラメータ不正"),
            (status = 401, description = "認証エラー"),
        ),
        request_body = UpdateUserPasswordRequest,
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn change_password(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    registry
        .user_repository()
        .update_password(UpdateUserPasswordRequestWithUserId::new(user.id(), req).into())
        .await?;
    Ok(StatusCode::OK)
}

/// ユーザーを追加する(管理者のみ)
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/v1/users",
        responses (
            (status = 200, description = "ユーザー登録成功", body = UserResponse),
            (status = 400, description = "リクエストパラメータ不正"),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限不足"),
        ),
        request_body = CreateUserRequest,
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenError);
    }

    req.validate(&())?;

    let registered_user = registry.user_repository().create(req.into()).await?;

    Ok(Json(registered_user.into()))
}

/// ユーザーを削除する(管理者のみ)
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        delete,
        path = "/api/v1/users/{user_id}",
        responses (
            (status = 204, description = "ユーザー削除成功"),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限不足"),
            (status = 404, description = "指定されたユーザーが見つからない場合"),
        ),
        params(
            ("user_id" = UserId, Path, description = "削除対象のユーザーID"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn delete_user(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenError);
    }

    registry
        .user_repository()
        .delete(DeleteUser { user_id })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// ユーザーのロールを更新する(管理者のみ)
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        put,
        path = "/api/v1/users/{user_id}/role",
        responses (
            (status = 200, description = "ロール更新成功"),
            (status = 400, description = "リクエストパラメータ不正"),
            (status = 401, description = "認証エラー"),
            (status = 403, description = "権限不足"),
            (status = 404, description = "指定されたユーザーが見つからない場合"),
        ),
        params(
            ("user_id" = UserId, Path, description = "更新対象のユーザーID"),
        ),
        request_body = UpdateUserRoleRequest,
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn change_role(
    user: AuthorizedUser,
    Path(user_id): Path<UserId>,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> AppResult<StatusCode> {
    if !user.is_admin() {
        return Err(AppError::ForbiddenError);
    }

    registry
        .user_repository()
        .update_role(UpdateUserRoleRequestWithUserId::new(user_id, req).into())
        .await?;

    Ok(StatusCode::OK)
}

/// 自身が借りている書籍の一覧を取得
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/users/me/checkouts",
        responses (
            (status = 200, description = "自身の貸出中書籍一覧取得成功", body = CheckoutsResponse),
            (status = 401, description = "認証エラー"),
        ),
        security(
            ("bearer_auth" = [])
        )
    )
)]
pub async fn get_checkouts(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<CheckoutsResponse>> {
    registry
        .check_out_repository()
        .find_unreturned_by_user_id(user.id())
        .await
        .map(CheckoutsResponse::from)
        .map(Json)
}
