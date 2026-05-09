use crate::model::id::UserId;
use uuid::Uuid;

pub struct CreateToken {
    pub user_id: UserId,
    pub access_token: String,
}

impl CreateToken {
    pub fn new(user_id: UserId) -> Self {
        let access_token = Uuid::new_v4().simple().to_string();
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
        let parsed_token = Uuid::parse_str(&create_token.access_token).expect("access_token should be a valid UUID");
        assert_eq!(create_token.access_token.len(), 32);
        assert_eq!(parsed_token.get_version(), Some(uuid::Version::Random));
    }

    #[test]
    fn test_create_token_unique_tokens() {
        let user_id = UserId::new();
        let token1 = CreateToken::new(user_id);
        let token2 = CreateToken::new(user_id);

        assert_ne!(token1.access_token, token2.access_token);
    }
}
