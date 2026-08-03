
use axum::Json;
use axum::extract::{State};
use http::{StatusCode};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use sqlx::PgPool;
use sqlx::types::Uuid;
use strum::IntoStaticStr;

use crate::db;
use crate::models::app_state::JwtSecret;
use back::shared::errors::ServerError;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub sid: String,
    pub exp: i64,
    pub iat: i64,
}


pub fn create_token(secret: &[u8], user_id: &Uuid, session_id: &Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        sid: session_id.to_string(),
        exp: (now + chrono::Duration::hours(24)).timestamp(),
        iat: now.timestamp(),
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
}

pub fn validate_api_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    ).map(|data| data.claims)
}

#[derive(Debug, IntoStaticStr)]
pub enum LoginError {
    UserNotFound,
    WrongCredentials,
    SessionCreationError,
    CookieError,
    TokenCreationError,
}

pub async fn login(pool: &PgPool, secret: &JwtSecret, username: &String, password: &String) -> Result<([u8; 32], String), LoginError> {
    let db_user = db::get_user_by_username(&pool, &username)
        .await
        .map_err(|err| {
            println!("login err = {:?}", err);
            LoginError::UserNotFound
        })?;

    let did_verify: bool = bcrypt::verify(&password, &db_user.password_hash)
        .map_err(|err| {
            println!("{:?}", err);
            LoginError::WrongCredentials
        })?;
    if !did_verify {
        return Err(LoginError::WrongCredentials)
    }

    let session_token: [u8; 32] = rand::random();
    let session_token_hash = Sha256::digest(&session_token);

    let crate::models::user::UserId(user_id) = db_user.id;
    let user_session = db::insert_user_session(&pool, &user_id, session_token_hash.into())
        .await
        .map_err(|err| {
            println!("{:?}", err);
            LoginError::SessionCreationError
        })?;

    let JwtSecret(secret) = secret;
    let api_token = crate::auth::middleware::create_token(secret.as_bytes(), &user_id, &user_session.id).map_err(|err| {
        println!("{:?}", err);
        LoginError::TokenCreationError
    })?;

    Ok((session_token, api_token))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    Unauthenticated,
    InternalError,
}

pub async fn auth_middleware(
    State(JwtSecret(secret)): State<JwtSecret>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, (StatusCode, Json<ServerError<AuthError>>)> {
    let token = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, Json(ServerError::new(AuthError::Unauthenticated, ()))))?;

    let claim = validate_api_token(token, secret.as_bytes())
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, Json(ServerError::new(AuthError::Unauthenticated, ())))
        })?;

    println!("yo are we good? token = {:?} | claim = {:?}", token, claim);
    let user_id = crate::models::user::make_user_id(claim.sub)
        .map_err(|err| {
            println!("make_user_id failed = {:?}", err);
            (StatusCode::UNAUTHORIZED, Json(ServerError::new(AuthError::Unauthenticated, ())))
        })?;
    let (mut parts, body) = request.into_parts();
    parts.extensions.insert(user_id);

    let req = http::Request::from_parts(parts, body);
    Ok(next.run(req).await)
}