use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub text: String,
    pub author_id: Uuid,
}

impl Post {
    pub fn new(text: String, author_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            author_id,
        }
    }
}
