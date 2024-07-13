use crate::models::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct UserSchema {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub about: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserSchema {
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: String,
    pub password: String,
    #[validate(url)]
    pub avatar: Option<String>,
    #[validate(length(max = 255, message = "About must be less than 255 characters"))]
    pub about: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserSchema {
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: Option<String>,
    pub password: Option<String>,
    #[validate(url)]
    pub avatar: Option<String>,
    #[validate(length(max = 255, message = "About must be less than 255 characters"))]
    pub about: Option<String>,
}

impl From<User> for UserSchema {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            avatar: value.avatar,
            about: value.about,
        }
    }
}
