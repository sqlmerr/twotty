use crate::Config;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn db_connection(settings: &Config) -> Result<PgPool, sqlx::Error> {
    let pull = PgPoolOptions::new()
        .connect(settings.database_url.as_str())
        .await?;

    Ok(pull)
}
