use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};

#[mockall::automock]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    /// アクセストークンからユーザIDを取得する
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AppResult<Option<UserId>>;
    /// メールアドレスとパスワードの検証
    async fn verify_user(&self, email: &str, password: &str) -> AppResult<UserId>;
    /// アクセストークンの作成
    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken>;
    /// アクセストークンの削除
    async fn delete_token(&self, access_token: AccessToken) -> AppResult<()>;
}
