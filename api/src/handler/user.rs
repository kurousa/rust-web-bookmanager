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
pub async fn get_current_user(user: AuthorizedUser) -> Json<UserResponse> {
    Json(UserResponse::from(user.user))
}

/// ユーザー一覧を取得する
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
