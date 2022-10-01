#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error};

use auth::{login, register, Login, Register};
use config::Config;
use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::auth::{AuthResponse, me};

mod auth;
mod config;
mod util;

#[derive(Clone)]
struct AppState {
    pool: Pool<Sqlite>,
    config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok().expect("Please check .env file for errors");
    let config = Config::new();

    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&config.database_url)
        .await?;

    // Run migrations on app start
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app_state = AppState { config, pool };

    #[tauri::command]
    fn greet(name: &str) -> String {
        format!("{name}")
    }

    #[tauri::command]
    async fn register_command(
        state: tauri::State<'_, AppState>,
        payload: Register,
    ) -> Result<AuthResponse, String> {
        match register(&state.pool, &state.config, payload).await {
            Ok(auth_response) => Ok(auth_response),
            Err(err) => Err(err.to_string()),
        }
    }

    #[tauri::command]
    async fn login_command(
        state: tauri::State<'_, AppState>,
        payload: Login,
    ) -> Result<AuthResponse, String> {
        match login(&state.pool, &state.config, payload).await {
            Ok(auth_response) => Ok(auth_response),
            Err(err) => Err(err.to_string()),
        }
    }

    #[tauri::command]
    async fn me_command(
        state: tauri::State<'_, AppState>,
        token: String,
    ) -> Result<AuthResponse, String> {
        match me(&state.config, token).await {
            Ok(auth_response) => Ok(auth_response),
            Err(err) => Err(err.to_string()),
        }
    }

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            login_command,
            register_command,
            me_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
