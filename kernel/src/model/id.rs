use serde::{Deserialize, Serialize};
use shared::error::AppError;
use std::str::FromStr;
#[cfg(debug_assertions)]
use utoipa::ToSchema;
/// IDの型を定義するマクロ
macro_rules! define_id {
    ($id_type: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
        #[cfg_attr(debug_assertions, derive(ToSchema))]
        #[serde(into = "String")]
        #[sqlx(transparent)]
        pub struct $id_type(uuid::Uuid);

        impl $id_type {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn raw(self) -> uuid::Uuid {
                self.0
            }
        }

        impl Default for $id_type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl FromStr for $id_type {
            type Err = AppError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(uuid::Uuid::parse_str(s)?))
            }
        }

        impl From<uuid::Uuid> for $id_type {
            fn from(u: uuid::Uuid) -> Self {
                Self(u)
            }
        }

        impl From<$id_type> for String {
            fn from(id: $id_type) -> Self {
                id.to_string()
            }
        }

        impl std::fmt::Display for $id_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    self.0
                        .as_simple()
                        .encode_lower(&mut uuid::Uuid::encode_buffer())
                )
            }
        }
    };
}

define_id!(UserId);
define_id!(BookId);
define_id!(CheckoutId);
