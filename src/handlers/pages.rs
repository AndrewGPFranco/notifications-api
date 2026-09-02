use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::dtos::notification_dto::OutputNotificationDTO;
use crate::services::notifications_service::{process_notifications, recover_notifications_user};
use crate::state::AppState;

pub async fn recover_notifications(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Vec<OutputNotificationDTO>>, StatusCode> {
    let notifications = recover_notifications_user(id, &state)
        .await
        .map_err(|error| {
            eprintln!("Erro ao recuperar notificações: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(notifications))
}

pub async fn process_demands(State(state): State<AppState>) {
    process_notifications(&state).await
}