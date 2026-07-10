use axum::extract::{FromRef};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};
use uuid::Uuid;
use sqlx::types::chrono;

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
    pub id: crate::models::user::UserId,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub user_id: crate::models::user::UserId,
    pub content: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Avatar {
    pub id: i32,
    pub user_id: crate::models::user::UserId,
    pub url: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
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

pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn get_user_by_username(pool: &PgPool, username: &String) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
}

pub async fn get_all_posts(pool: &PgPool, timestamp: &chrono::DateTime<chrono::Utc>) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE created_at < $1 ORDER BY created_at DESC LIMIT 20")
        .bind(timestamp)
        .fetch_all(pool)
        .await
}

pub async fn get_posts_by_user_id(pool: &PgPool, user_id: &Uuid, timestamp: &chrono::DateTime<chrono::Utc>) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE user_id = $1 AND created_at < $2 ORDER BY created_at DESC LIMIT 20")
        .bind(user_id)
        .bind(timestamp)
        .fetch_all(pool)
        .await
}

pub async fn get_feed_posts(pool: &PgPool, user_id: &Uuid, timestamp: &chrono::DateTime<chrono::Utc>) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM user_feeds WHERE feed_owner_id = $1 AND created_at < $2 ORDER BY created_at DESC LIMIT 20")
        .bind(user_id)
        .bind(timestamp)
        .fetch_all(pool)
        .await
}

pub async fn insert_post(pool: &PgPool, user_id: &Uuid, content: &String) -> Result<Post, sqlx::Error> {
    sqlx::query_as("INSERT INTO posts (user_id, content) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(content)
        .fetch_one(pool)
        .await
}

pub async fn get_avatar(pool: &PgPool, user_id: &Uuid) -> Result<Avatar, sqlx::Error> {
    sqlx::query_as("SELECT * FROM avatars WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn insert_avatar(pool: &PgPool, user_id: &Uuid, avatar_url: String) -> Result<Avatar, sqlx::Error> {
    sqlx::query_as("INSERT INTO avatars (user_id, url) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(avatar_url)
        .fetch_one(pool)
        .await
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Follow {
    pub user_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

pub async fn follow_user(pool: &PgPool, user_id: &Uuid, target_user_id: &Uuid) -> Result<Follow, sqlx::Error> {
    sqlx::query_as("INSERT INTO followers (user_id, followed_id) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(target_user_id)
        .fetch_one(pool)
        .await
}

pub async fn unfollow_user(pool: &PgPool, user_id: &Uuid, target_user_id: &Uuid) -> Result<Follow, sqlx::Error> {
    sqlx::query_as("DELETE FROM followers WHERE user_id = $1 AND followed_id = $2 RETURNING *")
        .bind(user_id)
        .bind(target_user_id)
        .fetch_one(pool)
        .await
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
