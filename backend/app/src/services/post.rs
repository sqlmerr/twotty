use crate::models::post::Post;
use crate::repositories::post::{CreatePostDTO, PostFindAllParams, PostRepository, UpdatePostDTO};
use crate::repositories::Repository;
use crate::schemas::post::{CreatePostSchema, PostSchema, UpdatePostSchema};
use crate::utils::errors::AppError;
use uuid::Uuid;
use validator::ValidateLength;

#[derive(Clone)]
pub struct PostService {
    pub repository: PostRepository,
}

impl PostService {
    pub async fn create_post(
        &self,
        data: CreatePostSchema,
        author_id: &Uuid,
    ) -> Result<PostSchema, AppError> {
        if !data.text.validate_length(Some(1), Some(256), None) {
            return Err(AppError::TextTooLong);
        }
        let dto = CreatePostDTO {
            text: data.text,
            author_id: *author_id,
        };
        let post: PostSchema = self.repository.create(dto).await.into();
        Ok(post)
    }

    pub async fn find_one_post(&self, id: &Uuid) -> Result<PostSchema, AppError> {
        match self.repository.find_one(id).await {
            None => Err(AppError::EntityNotFound {
                entity: "Post",
                id: *id,
            }),
            Some(post) => Ok(post.into()),
        }
    }

    pub async fn find_all_posts(&self, author_id: &Uuid) -> Vec<PostSchema> {
        let posts = self
            .repository
            .find_all(PostFindAllParams { author_id: *author_id })
            .await;
        posts.iter().map(|post| post.into()).collect()
    }

    pub async fn delete_post(&self, id: &Uuid, user_id: &Uuid) -> Result<(), AppError> {
        let post = self.find_one_post(&id).await?;
        if post.author_id != *user_id {
            return Err(AppError::CantDoThis)
        }
        self.repository.delete(&id).await;
        Ok(())
    }

    pub async fn update_post(&self, id: &Uuid, data: UpdatePostSchema, author_id: &Uuid) -> Result<(), AppError> {
        let post = self.find_one_post(&id).await?;
        if post.author_id != *author_id {
            return Err(AppError::CantDoThis);
        }

        let dto = UpdatePostDTO { text: data.text };
        self.repository.update(&id, dto).await;
        Ok(())
    }
}
