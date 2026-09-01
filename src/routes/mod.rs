use axum::{Router, routing::get};
use crate::handlers::pages::recover_notifications;
use crate::services::notifications_service::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/recover-notifications/{id}", get(recover_notifications))
        .with_state(state)
}