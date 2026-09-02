mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod configs;
mod dtos;
mod state;

use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use crate::configs::database::connect;
use crate::repositories::notification_repository::PostgresNotificationRepository;
use crate::routes::routes;
use crate::services::notifications_service::{setup_cron};
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = connect().await?;

    let api_key = env::var("API_KEY_ADMIN").expect("API_KEY_ADMIN não definida");

    let state = AppState {
        notification_repo: Arc::new(
            PostgresNotificationRepository::new(pool.clone())
        ),
        admin_api_key: Arc::new(api_key),
    };

    let _scheduler = setup_cron(state.clone()).await?;

    let app = routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
