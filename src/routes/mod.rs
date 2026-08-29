use axum::{Router, routing::get};

use crate::handlers::pages::{about, index};

/// Agrupa as rotas HTTP da aplicação.
///
/// Equivale, aproximadamente, à classe de configuração de rotas de uma
/// aplicação Java (por exemplo, os controllers registrados pelo Spring).
pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about))
}
