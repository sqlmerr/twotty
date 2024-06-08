use crate::services::user::UserService;
use crate::Config;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub config: Config,
}
