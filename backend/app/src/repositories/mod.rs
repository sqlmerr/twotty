use serde::Serialize;

pub mod user;
pub mod post;

#[async_trait::async_trait]
pub(super) trait Repository: Send + Sync {
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
