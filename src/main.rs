mod database;
mod handlers;
mod models;

use std::io;
use std::sync::Arc;

use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use database::ShortBase;
use handlers::{delete_url_data, get_url_data, shorten_url, update_url_data};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> io::Result<()> {
    let state = Arc::new(Mutex::new(ShortBase::new()));

    let app = Router::new()
        .route("/shorten", post(shorten_url))
        .route(
            "/shorten/:short_code",
            get(get_url_data)
                .put(update_url_data)
                .delete(delete_url_data),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(Any),
        )
        .with_state(state);

    let listener = TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
