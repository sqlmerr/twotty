mod auth;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response::Json, routing::get, Router};
use serde_json::json;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::db::db_connection;
use crate::state::AppState;
use crate::utils::errors::APIError;
use crate::{repositories, services, utils, Config};

use auth::AuthDoc;

pub async fn init_routers(settings: &Config) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        nest(
            (path = "/auth", api = AuthDoc)
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

    let db_connection = db_connection(settings).await.unwrap();

    let user_repository = repositories::user::UserRepository { db_connection };
    let user_service = services::user::UserService {
        repository: user_repository,
    };
    let state = AppState {
        user_service,
        config: settings.clone(),
    };

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .route(
            "/",
            get(|| async { Json(json!({"message": "Hello world"})) }),
        )
        .nest("/auth", auth::init_users_router())
        .fallback(handler_404)
        .with_state(state)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        APIError::new(StatusCode::NOT_FOUND, "Not found".to_string()),
    )
}
