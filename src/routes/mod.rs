use axum::{Router, routing::get, routing::put};
use crate::handlers::pages::{process_demands, recover_notifications, update_notification, update_all_notifications};
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/recover-notifications/{id}", get(recover_notifications))
        .route("/api/v1/process-demands", get(process_demands))
        .route("/api/v1/update-notification", put(update_notification))
        .route("/api/v1/update-all-notifications/{id}", put(update_all_notifications))
        .with_state(state)
}