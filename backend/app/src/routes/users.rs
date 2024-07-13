use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use super::auth_middleware;
use crate::{
    schemas::user::UserSchema,
    state::AppState,
    utils::errors::{AppError, AuthError},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        get_all_users,
        get_user_by_username,
        get_user,
        follow_user_by_id,
        unfollow_from_user,
        get_followings_count,
        get_followers_count,
        is_followed,
    ),
    components(schemas(UserSchema)),
    tags(
        (name = "users", description = "User api")
    )
)]
pub(super) struct UsersDoc;

pub(super) fn init_users_router(state: AppState) -> Router<AppState> {
    let auth_middleware = axum::middleware::from_fn_with_state(state, auth_middleware);

    Router::new()
        .route("/", get(get_all_users).layer(auth_middleware.clone()))
        .route("/:id", get(get_user).layer(auth_middleware.clone()))
        .route(
            "/@:username",
            get(get_user_by_username).layer(auth_middleware.clone()),
        )
        .route(
            "/:id/follow",
            post(follow_user_by_id).layer(auth_middleware.clone()),
        )
        .route(
            "/:id/unfollow",
            post(unfollow_from_user).layer(auth_middleware.clone()),
        )
        .route(
            "/:id/followings",
            get(get_followings_count).layer(auth_middleware.clone()),
        )
        .route(
            "/:id/followers",
            get(get_followers_count).layer(auth_middleware.clone()),
        )
        .route("/:id/followed", get(is_followed).layer(auth_middleware.clone()))
}

#[utoipa::path(
    get,
    path = "",
    tag = "users",
    responses(
        (status = 200, description = "Users", body = Vec<UserSchema>)
    ),
    security(
        ("http" = [])
    )
)]
async fn get_all_users(State(state): State<AppState>) -> impl IntoResponse {
    let tasks = state.user_service.find_all_users().await;
    Json(json!(tasks))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "users",
    responses(
        (status = 200, description = "User found successfully", body = UserSchema),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User id from database")
    ),
    security(
        ("http" = [])
    )
)]
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = state.user_service.find_one_user(&id).await?;
    Ok(Json(json!(task)))
}

#[utoipa::path(
    get,
    path = "/@{username}",
    tag = "users",
    responses(
        (status = 200, description = "User found", body = UserSchema),
        (status = 404, description = "User not found")
    ),
    params(
        ("username" = String, Path, description = "User's username")
    ),
    security(
        ("http" = [])
    )
)]
async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user = state
        .user_service
        .repository
        .find_one_by_username(&username)
        .await;
    if let Some(user) = user {
        return Ok(Json(UserSchema::from(user)));
    }
    return Err(AuthError::UserNotFound.into());
}

#[utoipa::path(
    post,
    path = "/{id}/follow",
    tag = "users",
    responses(
        (status = 201, description = "Successfully followed to this user."),
        (status = 400, description = "You're already following this user")
    ),
    params(
        ("id" = Uuid, Path, description = "User id")
    ),
    security(
        ("http" = [])
    )
)]
async fn follow_user_by_id(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let to_user = state.user_service.find_one_user(&id).await?;
    state.following_service.follow(user, to_user).await?;

    Ok(Json(json!({"ok": true})))
}

#[utoipa::path(
    post,
    path = "/{id}/unfollow",
    tag = "users",
    responses(
        (status = 201, description = "Successfully unfollowed from this user."),
        (status = 400, description = "You're not following this user")
    ),
    params(
        ("id" = Uuid, Path, description = "User id")
    ),
    security(
        ("http" = [])
    )
)]
async fn unfollow_from_user(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let to_user = state.user_service.find_one_user(&id).await?;
    state.following_service.unfollow(user, to_user).await?;

    Ok(Json(json!({"ok": true})))
}

#[utoipa::path(
    get,
    path = "/{id}/followings",
    tag = "users",
    responses(
        (status = 200, description = "User's followings count")
    ),
    params(
        ("id" = Uuid, Path, description = "User's id")
    ),
    security(
        ("http" = [])
    )
)]
async fn get_followings_count(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.find_one_user(&id).await?;
    let count = state.following_service.get_followings_count(user).await?;
    Ok(Json(json!({"count": count})))
}

#[utoipa::path(
    get,
    path = "/{id}/followers",
    tag = "users",
    responses(
        (status = 200, description = "User's followers count")
    ),
    params(
        ("id" = Uuid, Path, description = "User's id")
    ),
    security(
        ("http" = [])
    )
)]
async fn get_followers_count(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.find_one_user(&id).await?;
    let count = state.following_service.get_followers_count(user).await?;
    Ok(Json(json!({"count": count})))
}


#[utoipa::path(
    get,
    path = "/{id}/followed",
    tag = "users",
    responses(
        (status = 200),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User's id")
    ),
    security(
        ("http" = [])
    )
)]
async fn is_followed(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    Path(id): Path<Uuid>
) -> Result<impl IntoResponse, AppError> {
    let to = state.user_service.find_one_user(&id).await?;
    let following = state.following_service.get_following_by_from_and_to_ids(user, to).await;
    
    Ok(Json(json!({"isFollowed": following.is_some()})))
}