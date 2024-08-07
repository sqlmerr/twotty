use sqlx::PgPool;
use uuid::Uuid;

use super::Repository;
use crate::models::post::Post;

pub struct CreatePostDTO {
    pub text: String,
    pub author_id: Uuid,
}

pub struct UpdatePostDTO {
    pub text: Option<String>,
}

pub struct PostFindAllParams {
    pub author_id: Uuid,
}

#[derive(Clone)]
pub struct PostRepository {
    pub pool: PgPool,
}

#[async_trait::async_trait]
impl Repository for PostRepository {
    type Model = Post;
    type Id = Uuid;
    type CreateDTO = CreatePostDTO;
    type UpdateDTO = UpdatePostDTO;
    type FindAllParams = PostFindAllParams;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model {
        let id = Uuid::new_v4();
        let response = sqlx::query!(
            r#"INSERT INTO "post" (id, text, author_id) VALUES ($1, $2, $3) RETURNING created_at, edited"#,
            id,
            data.text,
            data.author_id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();
        Post {
            id,
            text: data.text,
            author_id: data.author_id,
            created_at: response.created_at,
            edited: response.edited,
        }
    }

    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model> {
        sqlx::query_as!(Post, r#"SELECT * FROM "post" WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    async fn find_all(&self, params: Self::FindAllParams) -> Vec<Self::Model> {
        sqlx::query_as!(
            Post,
            r#"SELECT * FROM "post" WHERE author_id = $1 ORDER BY created_at DESC"#,
            params.author_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    async fn delete(&self, id: &Self::Id) {
        sqlx::query!(r#"DELETE FROM "post" WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn update(&self, id: &Self::Id, data: Self::UpdateDTO) {
        let post = self.find_one(id).await;

        let mut post = post.unwrap();
        if let Some(text) = data.text {
            post.text = text
        }

        sqlx::query!(
            r#"UPDATE "post" SET text = $1, edited =  true WHERE id = $2"#,
            post.text,
            id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
