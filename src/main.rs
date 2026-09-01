mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod configs;
mod dtos;

use std::sync::Arc;
use crate::configs::database::connect;
use crate::repositories::notification_repository::PostgresNotificationRepository;
use crate::routes::routes;
use crate::services::notifications_service::{setup_cron, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect().await?;

    let state = AppState {
        notification_repo: Arc::new(
            PostgresNotificationRepository::new(pool.clone())
        ),
    };

    let _scheduler = setup_cron(state.clone()).await?;

    let app = routes(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
