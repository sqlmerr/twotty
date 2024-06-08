use axum::routing::{get, post};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::schemas::auth::Claims;
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
        get_user,
        get_all_users,
        login,
        register_user,
        delete_user,
        update_user,
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

#[utoipa::path(
    get,
    path = "",
    tag = "auth",
    responses(
        (status = 200, description = "Users", body = Vec<UserSchema>)
    ),
    security(
        ("http" = [])
    )
)]
pub async fn get_all_users(_claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    let tasks = state.user_service.find_all_users().await;
    Json(json!(tasks))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "auth",
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
pub async fn get_user(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = state.user_service.find_one_user(&id).await?;
    Ok(Json(json!(task)))
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
) -> impl IntoResponse {
    let user = state.user_service.create_user(user).await;
    tracing::info!("Successfully created a user: {:?}", user);
    (StatusCode::CREATED, Json(user))
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
    path = "/{id}",
    tag = "auth",
    request_body=UpdateUserSchema,
    responses(
        (status = 200, description = "User edited successfully"),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User id from database")
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdateUserSchema>,
) -> Result<impl IntoResponse, AppError> {
    state.user_service.update_user(&id, body).await?;
    Ok(Json(json!({ "message": "Task updated!" })))
}

pub fn init_users_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_users))
        .route("/:id", get(get_user).delete(delete_user).patch(update_user))
        .route("/login", post(login))
        .route("/register", post(register_user))
}
