use crate::services::following::FollowingService;
use crate::services::post::PostService;
use crate::services::user::UserService;
use crate::Config;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub post_service: PostService,
    pub following_service: FollowingService,
    pub config: Config,
}
