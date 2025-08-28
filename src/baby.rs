use axum::{extract::Query, Json};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(serde::Deserialize)]
pub struct AskQuery {
    pub ask: String,
}

#[derive(Serialize)]
pub struct Answer {
    pub ans: String,
}

pub async fn get_baby(
    Query(params): Query<AskQuery>,
    axum::Extension(pool): axum::Extension<SqlitePool>,
) -> Result<Json<Answer>, axum::http::StatusCode> {
    if let Ok(row) = sqlx::query!("SELECT ans FROM baby WHERE ask = ?", params.ask)
        .fetch_optional(&pool)
        .await
    {
        if let Some(record) = row {
            return Ok(Json(Answer { ans: record.ans }));
        }
    }
    Err(axum::http::StatusCode::NOT_FOUND)
}
