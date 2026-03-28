use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use kernel::model::{auth::AccessToken, id::UserId, role::Role, user::User};
use registry::AppRegistry;
use shared::error::AppError;

/// 認証済みユーザー情報
pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}
impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }
    pub fn is_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, registry)
            .await
            .map_err(|_| AppError::UnauthorizedError)?;

        let access_token = jar
            .get("access_token")
            .map(|cookie| AccessToken(cookie.value().to_string()))
            .ok_or(AppError::UnauthorizedError)?;

        // トークンからユーザーIDを取得
        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        // ユーザーIDからユーザー情報を取得
        let user = registry
            .user_repository()
            .find_current_user(user_id)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        Ok(Self { access_token, user })
    }
}
