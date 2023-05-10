use axum::{
    routing::{get, post},
    BoxError, Router,
};
use gluttonizer::{
    controller::{gluttonize, home, restaurants},
    repository::StandardRepository,
};

static DEFAULT_HTTP_HOST: &str = "127.0.0.1";
static DEFAULT_HTTP_PORT: &str = "8000";

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    StandardRepository::init()?;

    let app = Router::new()
        .route("/", get(home))
        .route("/results", post(gluttonize))
        .route("/restaurants", get(restaurants));

    let host = dotenv::var("HTTP_HOST").unwrap_or(DEFAULT_HTTP_HOST.to_string());
    let port = dotenv::var("HTTP_PORT").unwrap_or(DEFAULT_HTTP_PORT.to_string());

    let address = format!("{}:{}", host, port);

    println!("Server will listen on {}", address);

    axum::Server::bind(&address.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
