use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct UserSchema {
    pub id: Uuid,
    pub username: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUserSchema {
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserSchema {
    #[validate(length(min = 4, message = "Username must be at least 4 characters long"))]
    pub username: Option<String>,
    pub password: Option<String>,
}
