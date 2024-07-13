use crate::models::post::Post;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PostSchema {
    pub id: Uuid,
    pub text: String,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime,
    pub edited: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreatePostSchema {
    #[validate(length(
        min = 1,
        max = 256,
        message = "Text length must be between 1 and 256 characters"
    ))]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdatePostSchema {
    #[validate(length(
        min = 1,
        max = 256,
        message = "Text length must be between 1 and 256 characters"
    ))]
    pub text: Option<String>,
}

impl From<&Post> for PostSchema {
    fn from(value: &Post) -> Self {
        Self {
            id: value.clone().id,
            text: value.clone().text,
            author_id: value.author_id,
            created_at: value.created_at,
            edited: value.edited,
        }
    }
}
