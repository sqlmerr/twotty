pub mod config;
pub mod db;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod state;
pub mod utils;

pub use config::Config;

#[tokio::main]
async fn main() {
    let settings = Config::from_env();

    let filter = tracing_subscriber::filter::EnvFilter::default()
        .add_directive(tracing::Level::INFO.into())
        .add_directive("sqlx=error".parse().unwrap());

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db = db::db_connection(&settings).await.unwrap();
    sqlx::migrate!("../migrations").run(&db).await.unwrap();

    let app = routes::init_routers(&settings).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
