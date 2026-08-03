use serde::{Deserialize, Serialize};
use sqlx::{
    PgPool,
};
use uuid::Uuid;
use sqlx::types::chrono;


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: crate::models::user::UserId,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: crate::models::post::PostId,
    pub public_id: String,
    pub user_id: crate::models::user::UserId,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Avatar {
    pub id: i32,
    pub user_id: crate::models::user::UserId,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Like {
    pub user_id: crate::models::user::UserId,
    pub post_id: crate::models::post::PostId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Follow {
    pub user_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct JobTimelinePayload {
    pub user_id: crate::models::user::UserId,
    pub post_id: crate::models::post::PostId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


pub async fn insert_user(pool: &PgPool, username: &String, email: &String, hpass: &String) -> Result<User, sqlx::Error> {
    sqlx::query_as("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *")
        .bind(username)
        .bind(email)
        .bind(hpass)
        .fetch_one(pool)
        .await
}

pub async fn get_user_by_id(pool: &PgPool, user_id: &Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
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

pub async fn get_all_posts_latest(pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC LIMIT 20")
        .fetch_all(pool)
        .await
}

pub async fn get_all_posts_page(pool: &PgPool, timestamp: &chrono::DateTime<chrono::Utc>, post_id: &Uuid) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE (created_at, post_id) < ($1, $2) ORDER BY created_at DESC LIMIT 20")
        .bind(timestamp)
        .bind(post_id)
        .fetch_all(pool)
        .await
}

pub async fn get_posts_by_user_id_latest(pool: &PgPool, user_id: &Uuid) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE user_id = $1 AND created_at < $2 ORDER BY created_at DESC LIMIT 20")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn get_posts_by_user_id_page(pool: &PgPool, user_id: &Uuid, timestamp: &chrono::DateTime<chrono::Utc>, post_id: &Uuid) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE user_id = $1 AND created_at < $2 ORDER BY created_at DESC LIMIT 20")
        .bind(user_id)
        .bind(timestamp)
        .bind(post_id)
        .fetch_all(pool)
        .await
}

pub async fn get_feed_posts_latest(pool: &PgPool, owner_id: &Uuid) -> Result<Vec<Post>, sqlx::Error> {
    let query = r#"
        SELECT *
        FROM feed_posts
        WHERE author_user_id = $1
        ORDER BY created_at DESC
        LIMIT 20
    "#;
    sqlx::query_as::<_, Post>(query)
        .bind(owner_id)
        .fetch_all(pool)
        .await
}

pub async fn get_feed_posts_page(pool: &PgPool, owner_id: &Uuid, timestamp: &chrono::DateTime<chrono::Utc>, post_id: &Uuid) -> Result<Vec<Post>, sqlx::Error> {
    let query = r#"
        SELECT *
        FROM feed_posts
        WHERE author_user_id = $1 AND (created_at, post_id) < ($2, $3)
        ORDER BY created_at DESC
        LIMIT 20
    "#;
    sqlx::query_as::<_, Post>(query)
        .bind(owner_id)
        .bind(timestamp)
        .bind(post_id)
        .fetch_all(pool)
        .await
}

// TODO FIXME finish this... after we think about how it'll be used at all...
pub async fn get_full_posts(pool: &PgPool, timestamp: &chrono::DateTime<chrono::Utc>) -> Result<Vec<Post>, sqlx::Error> {
    let query = r#"
        SELECT
            p.*,
            u.username,
            a.avatar_url
        FROM posts p
        JOIN users u ON u.id = p.user_id
        WHERE p.created_at < $1
        ORDER BY p.created_at DESC
        LIMIT 20
    "#;
    sqlx::query_as::<_, Post>(query)
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

pub async fn insert_like(pool: &PgPool, user_id: &Uuid, post_id: &Uuid) -> Result<Like, sqlx::Error> {
    sqlx::query_as::<_, Like>("INSERT INTO likes (user_id, post_id) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(post_id)
        .fetch_one(pool)
        .await
}

pub async fn delete_like(pool: &PgPool, user_id: &Uuid, post_id: &Uuid) -> Result<Like, sqlx::Error> {
    sqlx::query_as::<_, Like>("DELETE FROM likes WHERE user_id = $1 AND post_id = $2 RETURNING *")
        .bind(user_id)
        .bind(post_id)
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

pub async fn follow_user(pool: &PgPool, user_id: &Uuid, target_user_id: &Uuid) -> Result<Follow, sqlx::Error> {
    sqlx::query_as("INSERT INTO follows (user_id, followed_id) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(target_user_id)
        .fetch_one(pool)
        .await
}

// TODO FIXME return paginated follows...
pub async fn get_follows(pool: &PgPool, target_user_id: &Uuid) -> Result<Vec<Follow>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM follows WHERE followed_id = $1")
        .bind(target_user_id)
        .fetch_all(pool)
        .await
}

pub async fn unfollow_user(pool: &PgPool, user_id: &Uuid, target_user_id: &Uuid) -> Result<Follow, sqlx::Error> {
    sqlx::query_as("DELETE FROM follows WHERE user_id = $1 AND followed_id = $2 RETURNING *")
        .bind(user_id)
        .bind(target_user_id)
        .fetch_one(pool)
        .await
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: [u8; 32],
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn insert_user_session(pool: &PgPool, user_id: &Uuid, token_hash: [u8; 32]) -> Result<UserSession, sqlx::Error> {
    sqlx::query_as("INSERT INTO user_sessions (user_id, token_hash) VALUES ($1, $2) RETURNING *")
        .bind(user_id)
        .bind(token_hash)
        .fetch_one(pool)
        .await
}

pub async fn get_user_session(pool: &PgPool, token_hash: [u8; 32]) -> Result<UserSession, sqlx::Error> {
    sqlx::query_as("SELECT * FROM user_sessions WHERE token_hash = $1")
        .bind(token_hash)
        .fetch_one(pool)
        .await
}

pub async fn revoke_user_session(pool: &PgPool, token_hash: [u8; 32]) -> Result<UserSession, sqlx::Error> {
    sqlx::query_as("UPDATE user_sessions SET revoked_at = now() WHERE token_hash = $1 RETURNING *")
        .bind(token_hash)
        .fetch_one(pool)
        .await
}
