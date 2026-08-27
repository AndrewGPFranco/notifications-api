use axum::{routing::get, Router};
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(index)).route("/about", get(about));
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn index() -> &'static str {
    "Home"
}

async fn about() -> &'static str {
    "About"
}