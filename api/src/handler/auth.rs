use axum::{extract::State, http::StatusCode, Json};
use kernel::model::auth::event::CreateToken;
use registry::AppRegistry;
use shared::error::AppResult;

use crate::{
    extractor::AuthorizedUser,
    model::auth::{AccessTokenResponse, LoginRequest},
};

/// ログイン処理
/// ユーザーの認証を行い、アクセストークンを発行する
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        post,
        path = "/api/auth/login",
        request_body = LoginRequest,
        responses (
            (status = 200, description = "ログイン成功", body = AccessTokenResponse),
        ),
    )
)]
pub async fn login(
    State(registry): State<AppRegistry>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AccessTokenResponse>> {
    let user_id = registry
        .auth_repository()
        .verify_user(&req.email, &req.password)
        .await?;
    let access_token = registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await?;

    Ok(Json(AccessTokenResponse {
        user_id,
        access_token: access_token.0,
    }))
}
/// ログアウト処理
/// ユーザーのアクセストークンを削除する
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/auth/logout",
        responses (
            (status = 204, description = "ログアウト成功",),
        ),
    )
)]
pub async fn logout(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    registry
        .auth_repository()
        .delete_token(user.access_token)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
