use dotenv::dotenv;
use std::env;

pub struct Config {
    pub db_password: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "2832".to_string())
            .parse()
            .expect("PORT must be a number");

        Self { db_password, port }
    }
}
