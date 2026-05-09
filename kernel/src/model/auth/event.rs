use crate::model::{auth::AccessToken, id::UserId};
use uuid::Uuid;

pub struct CreateToken {
    pub user_id: UserId,
    pub access_token: AccessToken,
}

impl CreateToken {
    pub fn new(user_id: UserId) -> Self {
        let access_token = AccessToken(Uuid::new_v4().simple().to_string());
        Self {
            user_id,
            access_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_new() {
        let user_id = UserId::new();
        let create_token = CreateToken::new(user_id);

        assert_eq!(create_token.user_id, user_id);
        assert!(!create_token.access_token.0.is_empty());
        // access_token should be a simple UUID (32 hex characters)
        assert_eq!(create_token.access_token.0.len(), 32);
        assert!(create_token
            .access_token
            .0
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_create_token_unique_tokens() {
        let user_id = UserId::new();
        let token1 = CreateToken::new(user_id);
        let token2 = CreateToken::new(user_id);

        assert_ne!(token1.access_token.0, token2.access_token.0);
    }
}
