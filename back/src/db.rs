use axum::{
    routing::{get, post},
    extract::{FromRef, State},
    Router,
    Json
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    admin: String,
    password: String,
    url: String,
    port: String,
    database: String,
}

#[derive(Clone, FromRef)]
pub struct Db(pub PgPool);

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn get_pool(config: &Config) -> PgPool {
    let url = format!("postgres://{}:{}@{}:{}/{}", config.admin, config.password, config.url, config.port, config.database);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("fail 2 cnct psql");
    pool
}

pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, username, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}


pub async fn connect() -> (Db, Config) {
    dotenvy::dotenv();

    // TODO make const? (can we)
    let config = Config {
        admin: std::env::var("POSTGRES_USER").expect("DATABASE_USER missing in .env"),
        password: std::env::var("POSTGRES_PASSWORD").expect("DATABASE_PASSWORD missing in .env"),
        url: std::env::var("POSTGRES_URL").expect("DATABASE_URL missing in .env"),
        database: std::env::var("POSTGRES_DB").expect("POSTGRES_DB missing in .env"),
        port: std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing in .env"),
    };

    (Db(get_pool(&config).await), config)
}
