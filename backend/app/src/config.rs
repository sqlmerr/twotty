use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub frontend_origin: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenvy::dotenv().ok();

        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
