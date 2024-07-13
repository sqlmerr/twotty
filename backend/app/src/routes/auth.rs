use axum::routing::{delete, get, patch, post};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension, Router,
};
use serde_json::json;
use uuid::Uuid;

use super::auth_middleware;
use crate::{
    schemas::{
        auth::{AuthBody, AuthPayload},
        user::{CreateUserSchema, UpdateUserSchema, UserSchema},
    },
    state::AppState,
    utils::{errors::AppError, validator::ValidatedJson},
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        login,
        register_user,
        delete_user,
        update_user,
        get_me,
    ),
    components(schemas(
        UserSchema,
        CreateUserSchema,
        UpdateUserSchema,
        AuthBody,
        AuthPayload,
    )),
    tags(
        (name = "auth", description = "Auth api")
    )
)]
pub(super) struct AuthDoc;

pub(super) fn init_auth_router(state: AppState) -> Router<AppState> {
    let auth_middleware = axum::middleware::from_fn_with_state(state, auth_middleware);
    Router::new()
        .route("/", patch(update_user).layer(auth_middleware.clone()))
        .route("/:id", delete(delete_user).layer(auth_middleware.clone()))
        .route("/login", post(login))
        .route("/register", post(register_user))
        .route("/me", get(get_me).layer(auth_middleware.clone()))
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "auth",
    responses(
        (status = 200, description = "Successfully signed in", body = AuthBody)
    ),
    request_body = AuthPayload,
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AppError> {
    let access_token = state.user_service.login_by_username(payload).await?;
    Ok(Json(AuthBody::new(access_token)))
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "auth",
    responses(
        (status = 201, description = "User registered successfully", body = UserSchema)
    ),
    request_body = CreateUserSchema
)]
pub async fn register_user(
    State(state): State<AppState>,
    ValidatedJson(user): ValidatedJson<CreateUserSchema>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.create_user(user).await?;
    tracing::info!("Successfully created a user: {:?}", user);
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "auth",
    responses(
        (status = 200, description = "User deleted successfully")
    ),
    params(
        ("id" = Uuid, Path, description = "User id from database")
    ),
    security(
        ("http" = [])
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.user_service.delete_user(&id).await?;
    Ok(Json(json!({"message": "User deleted"})))
}

#[utoipa::path(
    patch,
    path = "",
    tag = "auth",
    request_body = UpdateUserSchema,
    responses(
        (status = 200, description = "User edited successfully"),
        (status = 404, description = "User not found")
    ),
    security(
        ("http" = [])
    ),
)]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(user): Extension<UserSchema>,
    ValidatedJson(body): ValidatedJson<UpdateUserSchema>,
) -> Result<impl IntoResponse, AppError> {
    state.user_service.update_user(&user.id, body).await?;
    Ok(Json(json!({ "ok": true })))
}

#[utoipa::path(
    get,
    path = "/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current user", body = UserSchema)
    ),
    security(
        ("http" = [])
    )
)]
pub async fn get_me(Extension(me): Extension<UserSchema>) -> Result<impl IntoResponse, AppError> {
    Ok(Json(me))
}
