use serde::Serialize;

pub mod following;
pub mod post;
pub mod user;

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    type Model: Serialize;
    type Id;
    type CreateDTO;
    type UpdateDTO;
    type FindAllParams;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model;
    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model>;
    async fn find_all(&self, params: Self::FindAllParams) -> Vec<Self::Model>;
    async fn delete(&self, id: &Self::Id);
    async fn update(&self, id: &Self::Id, data: Self::UpdateDTO);
}
