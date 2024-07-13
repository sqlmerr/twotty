use super::Repository;
use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
    pub avatar: Option<String>,
    pub about: String,
}

pub struct UpdateUserDTO {
    pub username: Option<String>,
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub about: Option<String>,
}

#[derive(Clone)]
pub struct UserRepository {
    pub pool: PgPool,
}

#[async_trait::async_trait]
impl Repository for UserRepository {
    type Model = User;
    type Id = Uuid;
    type CreateDTO = CreateUserDTO;
    type UpdateDTO = UpdateUserDTO;
    type FindAllParams = ();

    async fn create(&self, data: Self::CreateDTO) -> Self::Model {
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"INSERT INTO "user" (id, username, password, avatar, about) VALUES ($1, $2, $3, $4, $5)"#,
            id,
            data.username,
            data.password,
            data.avatar,
            data.about
        )
        .execute(&self.pool)
        .await
        .unwrap();
        User {
            id,
            username: data.username,
            password: data.password,
            avatar: data.avatar,
            about: data.about,
        }
    }

    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model> {
        sqlx::query_as!(User, r#"SELECT * FROM "user" WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
    }

    async fn find_all(&self, _params: Self::FindAllParams) -> Vec<Self::Model> {
        sqlx::query_as!(User, r#"SELECT * FROM "user""#)
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

        if let Some(avatar) = data.avatar {
            user.avatar = Some(avatar);
        }

        if let Some(about) = data.about {
            user.about = about;
        }

        sqlx::query!(
            r#"UPDATE "user" SET username = $2, password = $3, avatar = $4, about = $5 WHERE id = $1"#,
            user.id,
            user.username,
            user.password,
            user.avatar,
            user.about
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}

impl UserRepository {
    pub async fn find_one_by_username(
        &self,
        username: &String,
    ) -> Option<<UserRepository as Repository>::Model> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM "user" WHERE lower(username) = LOWER($1)"#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }
}
