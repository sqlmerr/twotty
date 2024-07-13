use validator::ValidateLength;

use crate::{
    models::following::Following, repositories::{
        following::{CreateFollowingDTO, FollowingRepository},
        Repository,
    }, schemas::user::UserSchema, utils::errors::{AppError, Result}
};

#[derive(Clone)]
pub struct FollowingService {
    pub repository: FollowingRepository,
}

impl FollowingService {
    pub async fn follow(&self, from: UserSchema, to: UserSchema) -> Result<()> {
        if from.id == to.id {
            return Err(AppError::CantFollowYourself);
        }

        if let Some(_following) = self
            .repository
            .find_one_by_from_and_to_ids(&from.id, &to.id)
            .await
        {
            return Err(AppError::AlreadyFollowed);
        }

        let data = CreateFollowingDTO {
            from_id: from.id,
            to_id: to.id,
        };
        self.repository.create(data).await;
        Ok(())
    }

    pub async fn unfollow(&self, from: UserSchema, to: UserSchema) -> Result<()> {
        match self
            .repository
            .find_one_by_from_and_to_ids(&from.id, &to.id)
            .await
        {
            None => Err(AppError::NotFollowed),
            Some(following) => {
                self.repository.delete(&following.id).await;
                Ok(())
            }
        }
    }

    pub async fn get_followings_count(&self, user: UserSchema) -> Result<u64> {
        let followings = self.repository.find_all_by_from_id(&user.id).await;
        Ok(followings.length().unwrap_or(0))
    }

    pub async fn get_followers_count(&self, user: UserSchema) -> Result<u64> {
        let followings = self.repository.find_all_by_to_id(&user.id).await;
        Ok(followings.length().unwrap_or(0))
    }

    pub async fn get_following_by_from_and_to_ids(&self, from: UserSchema, to: UserSchema) -> Option<Following> {
        self.repository.find_one_by_from_and_to_ids(&from.id, &to.id).await
    }
}
