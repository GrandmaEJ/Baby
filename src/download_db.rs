use axum::{extract::Query, response::IntoResponse, http::StatusCode};
use std::fs;
use std::path::Path;

#[derive(serde::Deserialize)]
pub struct DownloadQuery {
    pub password: String,
}

pub async fn download_db(
    Query(params): Query<DownloadQuery>,
    axum::Extension(db_password): axum::Extension<String>,
) -> impl IntoResponse {
    if params.password != db_password {
        return (StatusCode::UNAUTHORIZED, "Invalid password").into_response();
    }

    let db_path = Path::new("baby.db");
    if !db_path.exists() {
        return (StatusCode::NOT_FOUND, "DB file not found").into_response();
    }

    match fs::read(db_path) {
        Ok(bytes) => (StatusCode::OK, bytes).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read DB").into_response(),
    }
}
