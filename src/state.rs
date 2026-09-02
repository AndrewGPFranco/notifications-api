use std::sync::Arc;
use crate::repositories::notification_repository::NotificationRepository;

#[derive(Clone)]
pub struct AppState {
    pub notification_repo: Arc<dyn NotificationRepository>,
    pub admin_api_key: Arc<String>
}