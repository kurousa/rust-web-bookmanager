use shared::error::{AppError, AppResult};
use std::str::FromStr;

use kernel::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};

use crate::redis::model::{RedisKey, RedisValue};

pub struct UserItem {
    pub user_id: UserId,
    pub password_hash: String,
}

pub struct AuthorizationKey(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorizedUserId(UserId);

pub fn from(event: CreateToken) -> (AuthorizationKey, AuthorizedUserId) {
    (
        AuthorizationKey(event.access_token.0),
        AuthorizedUserId(event.user_id),
    )
}

impl From<AuthorizationKey> for AccessToken {
    fn from(key: AuthorizationKey) -> Self {
        Self(key.0)
    }
}

impl From<AccessToken> for AuthorizationKey {
    fn from(token: AccessToken) -> Self {
        Self(token.0)
    }
}

impl From<&AccessToken> for AuthorizationKey {
    fn from(token: &AccessToken) -> Self {
        Self(token.0.to_string())
    }
}

impl RedisKey for AuthorizationKey {
    type Value = AuthorizedUserId;

    fn inner(&self) -> String {
        self.0.clone()
    }
}

impl RedisValue for AuthorizedUserId {
    fn inner(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for AuthorizedUserId {
    type Error = AppError;

    fn try_from(value: String) -> AppResult<Self> {
        Ok(Self(UserId::from_str(&value).map_err(|e| {
            AppError::ConversionEntityError(e.to_string())
        })?))
    }
}

impl AuthorizedUserId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_authorized_user_id_try_from_valid_string() {
        let uuid = Uuid::new_v4();
        let expected = AuthorizedUserId(UserId::from(uuid));

        let check = |input: String| {
            let result = AuthorizedUserId::try_from(input);
            assert_eq!(result.unwrap(), expected);
        };

        check(uuid.to_string());
        check(uuid.simple().to_string());
    }
    fn test_authorized_user_id_try_from_invalid_string() {
        let invalid_str = "not-a-uuid".to_string();

        let result = AuthorizedUserId::try_from(invalid_str);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::ConversionEntityError(_)));
    }
}
