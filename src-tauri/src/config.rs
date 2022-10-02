#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub app_secret: String,
    pub database_connection: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| String::from("Tauri App"));
        let app_secret = std::env::var("APP_SECRET")
            .unwrap_or_else(|_| String::from("5621709a6e58ea88d1c257f77c86fbfe"));
        let database_connection =
            std::env::var("DATABASE_CONNECTION").unwrap_or_else(|_| String::from("sqlite"));
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("sqlite://db.sqlite"));

        Config {
            app_name,
            app_secret,
            database_connection,
            database_url,
        }
    }
}
