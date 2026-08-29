use axum::{Router, routing::get};

use crate::handlers::pages::{recuperar_notificacoes};

pub fn routes() -> Router {
    Router::new().route("/", get(recuperar_notificacoes))
}
