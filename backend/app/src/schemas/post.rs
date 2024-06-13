use crate::models::post::Post;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct PostSchema {
    pub id: Uuid,
    pub text: String,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePostSchema {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePostSchema {
    pub text: Option<String>,
}

impl From<&Post> for PostSchema {
    fn from(value: &Post) -> Self {
        Self {
            id: value.clone().id,
            text: value.clone().text,
            author_id: value.author_id,
            created_at: value.created_at,
        }
    }
}
