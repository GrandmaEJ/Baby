use axum::Router;
use sqlx::SqlitePool;

mod baby;
mod teach;
mod download_db;

pub fn create_router(pool: SqlitePool, db_password: String) -> Router {
    use axum::Extension;
    use baby::get_baby;
    use teach::teach_baby;
    use download_db::download_db;

    Router::new()
        .route("/baby", axum::routing::get(get_baby))
        .route("/baby/teach", axum::routing::post(teach_baby))
        .route("/baby/download", axum::routing::get(download_db))
        .layer(Extension(pool))
        .layer(Extension(db_password))
}
