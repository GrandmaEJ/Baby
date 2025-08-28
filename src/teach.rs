use axum::{extract::Query, Json};
use sqlx::SqlitePool;

#[derive(serde::Deserialize)]
pub struct TeachQuery {
    pub ask: String,
    pub ans: String,
}

pub async fn teach_baby(
    Query(params): Query<TeachQuery>,
    axum::Extension(pool): axum::Extension<SqlitePool>,
) -> Result<Json<&'static str>, axum::http::StatusCode> {
    match sqlx::query!("INSERT INTO baby (ask, ans) VALUES (?, ?)", params.ask, params.ans)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok(Json("success res")),
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                Ok(Json("already haved"))
            } else {
                Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
