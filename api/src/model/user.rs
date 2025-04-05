use derive_new::new;
use garde::Validate;
use kernel::model::{
    id::UserId,
    role::Role,
    user::{
        event::{CreateUser, UpdateUserPassword, UpdateUserRole},
        User,
    },
};
use serde::{Deserialize, Serialize};
use strum::VariantNames;
#[cfg(debug_assertions)]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, VariantNames)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[strum(serialize_all = "kebab-case")]
/// UserResponseにおけるRoleの列挙型
pub enum RoleName {
    Admin,
    User,
}
impl From<Role> for RoleName {
    fn from(value: Role) -> Self {
        match value {
            Role::Admin => Self::Admin,
            Role::User => Self::User,
        }
    }
}
impl From<RoleName> for Role {
    fn from(value: RoleName) -> Self {
        match value {
            RoleName::Admin => Self::Admin,
            RoleName::User => Self::User,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
/// UserResponseを一覧で返すためのモデル
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
/// ユーザー情報のレスポンスモデル
pub struct UserResponse {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: RoleName,
}
impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        let User {
            id,
            name,
            email,
            role,
        } = value;
        Self {
            id,
            name,
            email,
            role: RoleName::from(role),
        }
    }
}

#[derive(Deserialize, Validate)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
/// パスワード更新ペイロード
pub struct UpdateUserPasswordRequest {
    #[garde(length(min = 1))]
    current_password: String,
    #[garde(length(min = 1))]
    new_password: String,
}

#[derive(new)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
/// ユーザーIDを持つパスワード更新ペイロード
pub struct UpdateUserPasswordRequestWithUserId(UserId, UpdateUserPasswordRequest);
impl From<UpdateUserPasswordRequestWithUserId> for UpdateUserPassword {
    fn from(value: UpdateUserPasswordRequestWithUserId) -> Self {
        let UpdateUserPasswordRequestWithUserId(
            user_id,
            UpdateUserPasswordRequest {
                current_password,
                new_password,
            },
        ) = value;

        UpdateUserPassword {
            user_id,
            current_password,
            new_password,
        }
    }
}

#[derive(Deserialize, Validate)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
/// ユーザー作成ペイロード
pub struct CreateUserRequest {
    #[garde(length(min = 1))]
    name: String,
    #[garde(email)]
    email: String,
    #[garde(length(min = 1))]
    password: String,
}
impl From<CreateUserRequest> for CreateUser {
    fn from(value: CreateUserRequest) -> Self {
        let CreateUserRequest {
            name,
            email,
            password,
        } = value;
        Self {
            name,
            email,
            password,
        }
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
/// ロール更新ペイロード
pub struct UpdateUserRoleRequest {
    role: RoleName,
}
#[derive(new)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
/// ユーザーIDを持つロール更新ペイロード
pub struct UpdateUserRoleRequestWithUserId(UserId, UpdateUserRoleRequest);
impl From<UpdateUserRoleRequestWithUserId> for UpdateUserRole {
    fn from(value: UpdateUserRoleRequestWithUserId) -> Self {
        let UpdateUserRoleRequestWithUserId(user_id, UpdateUserRoleRequest { role }) = value;
        Self {
            user_id,
            role: Role::from(role),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct BookOwner {
    pub id: UserId,
    pub name: String,
}
impl From<kernel::model::user::BookOwner> for BookOwner {
    fn from(value: kernel::model::user::BookOwner) -> Self {
        let kernel::model::user::BookOwner { id, name } = value;
        Self { id, name }
    }
}

#[derive(Serialize, Debug, Deserialize)]
#[cfg_attr(debug_assertions, derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CheckOutUser {
    pub id: UserId,
    pub name: String,
}
impl From<kernel::model::user::CheckOutUser> for CheckOutUser {
    fn from(value: kernel::model::user::CheckOutUser) -> Self {
        let kernel::model::user::CheckOutUser { id, name } = value;
        Self { id, name }
    }
}
