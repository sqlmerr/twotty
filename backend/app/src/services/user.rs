use crate::repositories::user::{CreateUserDTO, UpdateUserDTO, UserRepository};
use crate::repositories::Repository;
use crate::schemas::auth::{AuthPayload, Claims};
use crate::schemas::user::{CreateUserSchema, UpdateUserSchema, UserSchema};
use crate::utils::auth::{create_token, hash_password, verify_password};
use crate::utils::errors::{AppError, AuthError};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    pub repository: UserRepository,
}

impl UserService {
    pub async fn create_user(&self, data: CreateUserSchema) -> Result<UserSchema, AppError> {
        if let Some(_) = self.repository.find_one_by_username(&data.username).await {
            return Err(AuthError::UsernameAlreadyOccupied.into());
        }

        let hashed_password = hash_password(data.password);
        let response = self
            .repository
            .create(CreateUserDTO {
                username: data.username,
                password: hashed_password,
                avatar: data.avatar,
                about: data.about
            })
            .await;

        Ok(UserSchema {
            id: response.id,
            username: response.username,
            avatar: response.avatar,
            about: response.about
        })
    }

    pub async fn find_one_user(&self, id: &Uuid) -> Result<UserSchema, AppError> {
        let response = self.repository.find_one(id).await;
        match response {
            None => Err(AppError::EntityNotFound {
                entity: "User",
                id: *id,
            }),
            Some(user) => Ok(UserSchema {
                id: user.id,
                username: user.username,
                avatar: user.avatar,
                about: user.about
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
            if !verify_password(payload.password, user.password) {
                return Err(AuthError::WrongCredentials);
            }

            let claims = Claims::new(payload.username);
            let token = create_token(&claims).map_err(|_| AuthError::TokenCreation)?;

            return Ok(token);
        }

        Err(AuthError::WrongCredentials)
    }

    pub async fn find_all_users(&self) -> Vec<UserSchema> {
        let response = self.repository.find_all(()).await;
        let tasks: Vec<UserSchema> = response
            .iter()
            .map(|u| UserSchema {
                id: u.clone().id,
                username: u.clone().username,
                avatar: u.clone().avatar,
                about: u.clone().about
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
        let user = self.repository.find_one(id).await;
        if user.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "User",
                id: *id,
            });
        }

        if let Some(u) = data.username.clone() {
            let user_with_same_username = self.repository
                .find_one_by_username(&u)
                .await;
            if let Some(user) = user_with_same_username {
                if user.id != *id {
                    return Err(AuthError::UsernameAlreadyOccupied.into());
                }
            }
        }

        let password;

        if let Some(pass) = data.password {
            password = Some(hash_password(pass))
        } else {
            password = data.password
        }

        let dto = UpdateUserDTO {
            username: data.username.clone(),
            password,
            avatar: data.avatar,
            about: data.about,
        };
        self.repository.update(id, dto).await;
        Ok(())
    }
}
