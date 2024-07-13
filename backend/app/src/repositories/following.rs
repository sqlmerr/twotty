use sqlx::PgPool;
use uuid::Uuid;

use crate::models::following::Following;

use super::Repository;

#[derive(Clone)]
pub struct FollowingRepository {
    pub pool: PgPool,
}

pub struct CreateFollowingDTO {
    pub from_id: Uuid,
    pub to_id: Uuid,
}

pub struct FindAllFollowingsParams {
    pub from_id: Uuid,
}

#[async_trait::async_trait]
impl Repository for FollowingRepository {
    type Model = Following;
    type Id = Uuid;
    type CreateDTO = CreateFollowingDTO;
    type UpdateDTO = ();
    type FindAllParams = FindAllFollowingsParams;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model {
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"INSERT INTO "following" (id, from_id, to_id) VALUES ($1, $2, $3)"#,
            id,
            data.from_id,
            data.to_id
        )
        .execute(&self.pool)
        .await
        .unwrap();

        Following {
            id,
            from_id: data.from_id,
            to_id: data.to_id,
        }
    }

    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model> {
        sqlx::query_as!(Following, r#"SELECT * FROM "following" WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    async fn find_all(&self, params: Self::FindAllParams) -> Vec<Self::Model> {
        sqlx::query_as!(
            Following,
            r#"SELECT * FROM "following" WHERE from_id = $1"#,
            params.from_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    async fn delete(&self, id: &Self::Id) {
        sqlx::query!(r#"DELETE FROM "following" WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn update(&self, _id: &Self::Id, _data: Self::UpdateDTO) {}
}

impl FollowingRepository {
    pub async fn find_one_by_from_and_to_ids(
        &self,
        from_id: &Uuid,
        to_id: &Uuid,
    ) -> Option<<FollowingRepository as Repository>::Model> {
        sqlx::query_as!(
            Following,
            r#"SELECT * FROM "following" WHERE from_id = $1 AND to_id = $2"#,
            from_id,
            to_id
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_all_by_from_id(
        &self,
        from_id: &Uuid,
    ) -> Vec<<FollowingRepository as Repository>::Model> {
        sqlx::query_as!(
            Following,
            r#"SELECT * FROM "following" WHERE from_id = $1"#,
            from_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn find_all_by_to_id(
        &self,
        to_id: &Uuid,
    ) -> Vec<<FollowingRepository as Repository>::Model> {
        sqlx::query_as!(
            Following,
            r#"SELECT * FROM "following" WHERE to_id = $1"#,
            to_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
