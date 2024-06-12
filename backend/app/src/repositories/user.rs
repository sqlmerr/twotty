use super::Repository;
use crate::models::user;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
}

pub struct UpdateUserDTO {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone)]
pub struct UserRepository {
    pub pool: PgPool,
}

#[async_trait::async_trait]
impl Repository for UserRepository {
    type Model = user::User;
    type Id = Uuid;
    type CreateDTO = CreateUserDTO;
    type UpdateDTO = UpdateUserDTO;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model {
        let id = Uuid::new_v4();
        let task = sqlx::query!(
            r#"INSERT INTO "user" VALUES ($1, $2, $3)"#,
            id,
            data.username,
            data.password
        )
        .execute(&self.pool)
        .await
        .unwrap();
        Self::Model {
            id,
            username: data.username,
            password: data.password,
        }
    }

    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model> {
        sqlx::query_as!(user::User, r#"SELECT * FROM "user" WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    async fn find_all(&self) -> Vec<Self::Model> {
        sqlx::query_as!(user::User, r#"SELECT * FROM "user""#)
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    async fn delete(&self, id: &Self::Id) {
        sqlx::query!(r#"DELETE FROM "user" WHERE id = $1"#, id)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn update(&self, id: &Self::Id, data: Self::UpdateDTO) {
        let user = self.find_one(id).await;

        let mut user = user.unwrap();
        if let Some(username) = data.username {
            user.username = username;
        }
        if let Some(password) = data.password {
            user.password = password;
        }

        sqlx::query!(
            r#"UPDATE "user" SET username = $2, password = $3 WHERE id = $1"#,
            user.id,
            user.username,
            user.password
        );
    }
}

impl UserRepository {
    pub async fn find_one_by_username(
        &self,
        username: &String,
    ) -> Option<<UserRepository as Repository>::Model> {
        sqlx::query_as!(
            user::User,
            r#"SELECT * FROM "user" WHERE username = $1"#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }
}
