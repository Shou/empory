
use axum::extract::{FromRef};
use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};


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


pub async fn get_pool(config: &Config) -> PgPool {
    let url = format!("postgres://{}:{}@{}:{}/{}", config.admin, config.password, config.url, config.port, config.database);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("fail 2 cnct psql");
    pool
}

pub async fn connect() -> (Db, Config) {
    let config = Config {
        admin: std::env::var("POSTGRES_USER").expect("DATABASE_USER missing in .env"),
        password: std::env::var("POSTGRES_PASSWORD").expect("DATABASE_PASSWORD missing in .env"),
        url: std::env::var("POSTGRES_URL").expect("DATABASE_URL missing in .env"),
        database: std::env::var("POSTGRES_DB").expect("POSTGRES_DB missing in .env"),
        port: std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing in .env"),
    };

    (Db(get_pool(&config).await), config)
}