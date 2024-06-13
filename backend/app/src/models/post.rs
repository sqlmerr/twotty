use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};

use crate::schemas::post::PostSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub text: String,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl Post {
    pub fn new(text: String, author_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            author_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}

impl Into<PostSchema> for Post {
    fn into(self) -> PostSchema {
        PostSchema {
            id: self.id,
            author_id: self.author_id,
            text: self.text,
            created_at: self.created_at
        }
    }
}
