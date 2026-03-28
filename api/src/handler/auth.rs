use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
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
    jar: CookieJar,
    Json(req): Json<LoginRequest>,
) -> AppResult<(CookieJar, Json<AccessTokenResponse>)> {
    let user_id = registry
        .auth_repository()
        .verify_user(&req.email, &req.password)
        .await?;
    let access_token = registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await?;

    let cookie = Cookie::build(("access_token", access_token.0.clone()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    Ok((
        jar.add(cookie),
        Json(AccessTokenResponse { user_id }),
    ))
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
    jar: CookieJar,
) -> AppResult<(CookieJar, StatusCode)> {
    registry
        .auth_repository()
        .delete_token(user.access_token)
        .await?;

    let cookie = Cookie::build(("access_token", ""))
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    Ok((jar.add(cookie), StatusCode::NO_CONTENT))
}
