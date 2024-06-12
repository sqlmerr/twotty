use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostSchema {
    pub id: Uuid,
    pub text: String,
    pub author_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePostSchema {
    pub text: String
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePostSchema {
    pub text: Option<String>
}
