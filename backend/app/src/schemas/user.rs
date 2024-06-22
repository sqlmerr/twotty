use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use crate::models::user::User;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct UserSchema {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserSchema {
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: String,
    pub password: String,
    #[validate(url)]
    pub avatar: Option<String>
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserSchema {
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: Option<String>,
    pub password: Option<String>,
    #[validate(url)]
    pub avatar: Option<String>
}

impl From<User> for UserSchema {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            avatar: value.avatar
        }
    }
}
