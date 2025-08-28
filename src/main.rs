mod server;
mod config;
mod baby;
mod teach;
mod download_db;

use sqlx::sqlite::SqlitePoolOptions;
use server::create_router;
use config::Config;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Config::from_env();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("baby.db")
        .await?;

    let app = create_router(pool, cfg.db_password);

    let addr = SocketAddr::from(([0,0,0,0], cfg.port));
    println!("Server running at {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
