mod auth;
mod posts;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{response::Json, routing::get, Router};

use serde_json::json;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::db::db_connection;
use crate::state::AppState;
use crate::utils::errors::{APIError, AppError, AuthError};
use crate::{repositories, services, utils, Config};

use crate::schemas::user::UserSchema;
use crate::utils::auth::decode_token;
use auth::AuthDoc;
use posts::PostsDoc;

pub async fn init_routers(settings: &Config) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        nest(
            (path = "/auth", api = AuthDoc),
            (path = "/posts", api = PostsDoc)
        ),
        components(schemas(
            utils::errors::APIError
        ))
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "http",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
            }
        }
    }

    let pool = db_connection(settings).await.unwrap();

    let user_repository = repositories::user::UserRepository { pool: pool.clone() };
    let post_repository = repositories::post::PostRepository { pool: pool.clone() };

    let user_service = services::user::UserService {
        repository: user_repository,
    };
    let post_service = services::post::PostService {
        repository: post_repository,
    };
    let state = AppState {
        user_service,
        post_service,
        config: settings.clone(),
    };

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .nest("/auth", auth::init_users_router(state.clone()))
        .nest("/posts", posts::init_posts_router(state.clone()))
        .fallback(handler_404)
        .with_state(state)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        APIError::new(StatusCode::NOT_FOUND, "Not found".to_string()),
    )
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let auth_header = match request.headers_mut().get(axum::http::header::AUTHORIZATION) {
        None => return Err(AuthError::InvalidToken.into()),
        Some(header) => header.to_str().map_err(|_| AuthError::InvalidToken)?,
    };

    let mut header = auth_header.split_whitespace();
    let (_token_type, token) = (header.next(), header.next().ok_or(AuthError::InvalidToken)?);

    let token_data = decode_token(token).map_err(|_| AuthError::InvalidToken)?;
    request.extensions_mut().insert(token_data.claims.clone());

    let user: UserSchema = state
        .user_service
        .repository
        .find_one_by_username(&token_data.claims.sub)
        .await
        .ok_or(AuthError::InvalidToken)?
        .into();
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
