use crate::repositories::user::{CreateUserDTO, UpdateUserDTO, UserRepository};
use crate::repositories::Repository;
use crate::schemas::auth::{AuthPayload, Claims};
use crate::schemas::user::{CreateUserSchema, UpdateUserSchema, UserSchema};
use crate::utils::auth::create_token;
use crate::utils::errors::{AppError, AuthError};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub async fn create_user(&self, data: CreateUserSchema) -> UserSchema {
        let response = self
            .repository
            .create(CreateUserDTO {
                username: data.username,
                password: data.password, // TODO: hash password
            })
            .await;

        UserSchema {
            id: response.id,
            username: response.username,
        }
    }

    pub async fn find_one_user(&self, id: &Uuid) -> Result<UserSchema, AppError> {
        let response = self.repository.find_one(id).await;
        match response {
            None => Err(AppError::EntityNotFound {
                entity: "User",
                id: *id,
            }),
            Some(task) => Ok(UserSchema {
                id: task.id,
                username: task.username,
            }),
        }
    }

    pub async fn login_by_username(&self, payload: AuthPayload) -> Result<String, AuthError> {
        if payload.username.is_empty() || payload.password.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        let user = self
            .repository
            .find_one_by_username(&payload.username)
            .await;
        if let Some(user) = user {
            if user.password != payload.password {
                return Err(AuthError::WrongCredentials);
            } // TODO: decode password

            let claims = Claims::new(payload.username);
            let token = create_token(&claims).map_err(|_| AuthError::TokenCreation)?;

            return Ok(token);
        }

        Err(AuthError::WrongCredentials)
    }

    pub async fn find_all_users(&self) -> Vec<UserSchema> {
        let response = self.repository.find_all().await;
        let tasks: Vec<UserSchema> = response
            .iter()
            .map(|t| UserSchema {
                id: t.id,
                username: t.to_owned().username,
            })
            .collect();
        tasks
    }

    pub async fn delete_user(&self, id: &Uuid) -> Result<(), AppError> {
        let task = self.repository.find_one(id).await;
        if task.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "User",
                id: *id,
            });
        }

        self.repository.delete(id).await;
        Ok(())
    }

    pub async fn update_user(&self, id: &Uuid, data: UpdateUserSchema) -> Result<(), AppError> {
        let task = self.repository.find_one(id).await;
        if task.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "User",
                id: *id,
            });
        }

        let dto = UpdateUserDTO {
            username: data.username,
            password: data.password, // TODO: hash password
        };
        self.repository.update(id, dto).await;
        Ok(())
    }
}
