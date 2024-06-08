use crate::Config;
use sea_orm::{Database, DbConn, DbErr};

pub async fn db_connection(settings: &Config) -> Result<DbConn, DbErr> {
    let db = Database::connect(&settings.database_url).await.unwrap();

    Ok(db)
}
