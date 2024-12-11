use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::Mutex;

use crate::{
    database::ShortBase,
    models::{UrlData, UrlPayload},
};

pub async fn shorten_url(
    State(db): State<Arc<Mutex<ShortBase>>>,
    Json(payload): Json<UrlPayload>,
) -> Response {
    let db = db.lock().await;

    let current_time = chrono::Utc::now().to_rfc3339();

    let mut url_data = UrlData {
        url: payload.url,
        short_code: generate_short_code(),
        created_at: current_time.clone(),
        updated_at: current_time,
        ..Default::default()
    };

    if db.insert_record(&mut url_data).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Json(url_data).into_response()
}

pub async fn get_url_data(
    State(db): State<Arc<Mutex<ShortBase>>>,
    Path(short_code): Path<String>,
) -> Response {
    let db = db.lock().await;

    match db.get_record(&short_code) {
        Some(url_data) => Json(url_data).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_url_data(
    State(db): State<Arc<Mutex<ShortBase>>>,
    Path(short_code): Path<String>,
    Json(payload): Json<UrlPayload>,
) -> Response {
    let db = db.lock().await;

    match db.update_record(&short_code, &payload.url) {
        Some(url_data) => Json(url_data).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_url_data(
    State(db): State<Arc<Mutex<ShortBase>>>,
    Path(short_code): Path<String>,
) -> Response {
    let db = db.lock().await;

    if db.delete_record(&short_code).is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
