mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use crate::routes::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
