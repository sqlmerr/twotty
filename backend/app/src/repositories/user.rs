use super::Repository;
use crate::models::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
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
    pub db_connection: DbConn,
}

#[async_trait::async_trait]
impl Repository for UserRepository {
    type Model = user::Model;
    type Id = Uuid;
    type CreateDTO = CreateUserDTO;
    type UpdateDTO = UpdateUserDTO;

    async fn create(&self, data: Self::CreateDTO) -> Self::Model {
        let db = &self.db_connection;
        let task = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(data.username),
            password: Set(data.password),
            ..Default::default()
        };
        task.insert(db).await.unwrap()
    }

    async fn find_one(&self, id: &Self::Id) -> Option<Self::Model> {
        let db = &self.db_connection;
        user::Entity::find_by_id(id.to_owned())
            .one(db)
            .await
            .unwrap()
    }

    async fn find_all(&self) -> Vec<Self::Model> {
        let db = &self.db_connection;
        user::Entity::find().all(db).await.unwrap()
    }

    async fn delete(&self, id: &Self::Id) {
        let db = &self.db_connection;
        user::Entity::delete_by_id(id.to_owned())
            .exec(db)
            .await
            .unwrap();
    }

    async fn update(&self, id: &Self::Id, data: Self::UpdateDTO) {
        let db = &self.db_connection;
        let user = self.find_one(id).await;

        let mut user: user::ActiveModel = user.unwrap().into();
        if let Some(username) = data.username {
            user.username = Set(username)
        }
        if let Some(password) = data.password {
            user.password = Set(password);
        }

        user.update(db).await.unwrap();
    }
}

impl UserRepository {
    pub async fn find_one_by_username(
        &self,
        username: &String,
    ) -> Option<<UserRepository as Repository>::Model> {
        let db = &self.db_connection;
        user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
            .unwrap()
    }
}
