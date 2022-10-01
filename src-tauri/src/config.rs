// use tracing::Level;

// use crate::util::string_to_tracing_level;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub app_secret: String,
    pub database_connection: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| String::from("Web App"));
        let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is not set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let database_connection =
            std::env::var("DATABASE_CONNECTION").expect("DATABASE_CONNECTION is not set");

        Config {
            app_name,
            app_secret,
            database_connection,
            database_url,
        }
    }
}
