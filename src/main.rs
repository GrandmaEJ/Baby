use axum::{
    extract::Query,
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct AskQuery {
    ask: String,
}

#[derive(Deserialize)]
struct TeachQuery {
    ask: String,
    ans: String,
}

#[derive(Serialize)]
struct Answer {
    ans: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to existing baby.db
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("baby.db")
        .await?;

    // Router
    let app = Router::new()
        .route("/baby", get(get_baby))
        .route("/baby/teach", post(teach_baby))
        .layer(axum::Extension(pool));

    // Bind to PORT from environment (Render sets this)
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "2832".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running at {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// GET /baby?ask=hello
async fn get_baby(
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

// POST /baby/teach?ask=hey&ans=hello
async fn teach_baby(
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
