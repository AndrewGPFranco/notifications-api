use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::pages::recuperar_notificacoes;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(recuperar_notificacoes))
        .with_state(pool)
}
