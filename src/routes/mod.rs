use axum::{Router, routing::get};
use crate::handlers::pages::{process_demands, recover_notifications};
use crate::state::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/recover-notifications/{id}", get(recover_notifications))
        .route("/api/v1/process-demands", get(process_demands))
        .with_state(state)
}