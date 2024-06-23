use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::routes::auth_middleware;
use crate::schemas::post::{CreatePostSchema, PostSchema, UpdatePostSchema};
use crate::schemas::user::UserSchema;
use crate::state::AppState;
use crate::utils::errors::{AppError, AuthError};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        create_post,
        get_post,
        get_all_posts,
        get_posts_by_username,
        delete_post,
        update_post
    ),
    components(schemas(
        PostSchema,
        UpdatePostSchema,
        CreatePostSchema,
    )),
    tags(
        (name = "posts", description = "Posts api")
    )
)]
pub(super) struct PostsDoc;

pub fn init_posts_router(state: AppState) -> Router<AppState> {
    let auth_middleware = axum::middleware::from_fn_with_state(state, auth_middleware);
    Router::new()
        .route("/", post(create_post).get(get_all_posts))
        .route("/:id", get(get_post).delete(delete_post).patch(update_post))
        .route("/@:username", get(get_posts_by_username))
        .layer(auth_middleware)
}

#[utoipa::path(
    post,
    path = "",
    tag = "posts",
    request_body = CreatePostSchema,
    responses(
        (status = 201, description = "Post successfully created", body = PostSchema)
    ),
    security(
        ("http" = [])
    )
)]
pub async fn create_post(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    Json(data): Json<CreatePostSchema>,
) -> Result<impl IntoResponse, AppError> {
    let post = state.post_service.create_post(data, &user.id).await?;
    tracing::info!("Created post with id `{}`", post.id);
    Ok((StatusCode::CREATED, Json(post)))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "posts",
    params(
        ("id" = Uuid, Path, description = "Post id from database")
    ),
    responses(
        (status = 200, description = "Post found successfully", body = PostSchema),
        (status = 404, description = "Post not found")
    ),
    security(
        ("http" = [])
    )
)]
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let post = state.post_service.find_one_post(&id).await?;
    Ok(Json(post))
}

#[utoipa::path(
    get,
    path = "",
    tag = "posts",
    responses(
        (status = 200, description = "Your posts", body = Vec<PostSchema>)
    ),
    security(
        ("http" = [])
    )
)]
pub async fn get_all_posts(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
) -> impl IntoResponse {
    let posts = state.post_service.find_all_posts(&user.id).await;
    Json(posts)
}

#[utoipa::path(
    get,
    path = "/@{username}",
    tag = "posts",
    params(
        ("username" = String, Path, description = "Username from database")
    ),
    responses(
        (status = 200, description = "User's posts", body = Vec<PostSchema>)
    ),
    security(
        ("http" = [])
    )
)]
pub async fn get_posts_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.repository.find_one_by_username(&username).await;
    if user.is_none() {
        return Err(AuthError::UserNotFound.into());
    }
    let user = user.unwrap();
    let posts = state.post_service.find_all_posts(&user.id).await;
    Ok(Json(posts))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "posts",
    params(
        ("id" = Uuid, Path, description = "Post id from database")
    ),
    responses(
        (status = 200, description = "Post deleted"),
        (status = 404, description = "Post not found"),
        (status = 403, description = "Doesn't have permission to do this")
    ),
    security(
        ("http" = [])
    )
)]
pub async fn delete_post(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.post_service.delete_post(&id, &user.id).await?;
    Ok(Json(json!({"ok": true})))
}

#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "posts",
    request_body = UpdatePostSchema,
    params(
        ("id" = Uuid, Path, description = "Post id from database"),
    ),
    responses(
        (status = 200, description = "Post updated"),
        (status = 404, description = "Post not found"),
        (status = 403, description = "Doesn't have permission to do this")
    ),
    security(
        ("http" = [])
    )
)]
pub async fn update_post(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    Path(id): Path<Uuid>,
    Json(data): Json<UpdatePostSchema>,
) -> Result<impl IntoResponse, AppError> {
    state.post_service.update_post(&id, data, &user.id).await?;
    Ok(Json(json!({"ok": true})))
}
