use axum::{
    extract::{State},
    Json,
};
use base64::prelude::*;
use http::{StatusCode};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use strum::Display;

use crate::{auth::middleware, models};
use crate::db;
use back::shared::{
    db as dbt,
    errors::ServerError
};


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum RegisterError {
    InvalidEmail,
    UsernameTooLong,
    PasswordInsecure,
    PasswordTooLong,
    UserAlreadyExists,
    InternalError,
}

#[derive(Debug, Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum LoginError {
    InvalidCredentials,
    InternalError,
}

#[derive(Debug, Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum LogoutError {
    AlreadyLoggedOut,
    InternalError,
}

#[derive(Debug, Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum RefreshError {
    LoggedOut,
    InternalError,
}


pub async fn login(
    State(app_state): State<models::app_state::AppState>,
    login_user: Json<LoginUser>,
) -> Result<(http::HeaderMap, Json<RefreshResponse>), (StatusCode, Json<ServerError<LoginError>>)> {
    println!("user = {:?}", &login_user);
    let dbt::Db(pool) = app_state.pool;
    let (session_token, api_token) = crate::auth::middleware::login(
        &pool,
        &app_state.secret,
        &login_user.username,
        &login_user.password,
    )
        .await
        .map_err(|err| {
            println!("login err = {:?}", err);
            let login_error = match err {
                middleware::LoginError::UserNotFound => LoginError::InvalidCredentials,
                middleware::LoginError::WrongCredentials => LoginError::InvalidCredentials,
                _ => LoginError::InternalError,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(login_error, ())))
        })?;

    let cookie_value = BASE64_URL_SAFE_NO_PAD.encode(session_token);
    let mut hm = http::HeaderMap::new();
    let cookie = format!("session={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=2592000", cookie_value)
        .parse()
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(LoginError::InternalError, ())))
        })?;
    hm.insert(
        axum::http::header::SET_COOKIE,
        cookie,
    );

    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

    Ok((hm, Json(RefreshResponse { token: api_token, expires_at })))
}

pub async fn logout(
    State(app_state): State<models::app_state::AppState>,
    cookies: tower_cookies::Cookies,
) -> Result<http::HeaderMap, (StatusCode, Json<ServerError<LogoutError>>)> {
    let cookie: tower_cookies::Cookie = cookies.get("session")
        .ok_or_else(|| {
            println!("refresh session cookie not found");
            (StatusCode::UNAUTHORIZED, Json(ServerError::new(LogoutError::AlreadyLoggedOut, ())))
        })?;
    
    let base64value = cookie.value();
    let user_session_hash = Sha256::digest(cookie.value());

    let dbt::Db(pool) = app_state.pool;
    crate::db::revoke_user_session(&pool, user_session_hash.into())
        .await
        .map_err(|err| {
            println!("logout session db error = {:?}", err);
            (StatusCode::UNAUTHORIZED, Json(ServerError::new(LogoutError::AlreadyLoggedOut, ())))
        })?;

    let mut hm = http::HeaderMap::new();
    let cookie = format!("session=deleted; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0")
        .parse()
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, Json(ServerError::new(LogoutError::InternalError, ())))
        })?;
    hm.insert(
        axum::http::header::SET_COOKIE,
        cookie,
    );
    Ok(hm)
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefreshResponse {
    token: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

#[axum::debug_handler]
pub async fn refresh(
    State(app_state): State<models::app_state::AppState>,
    cookies: tower_cookies::Cookies,
) -> Result<Json<RefreshResponse>, (StatusCode, http::HeaderMap, Json<ServerError<RefreshError>>)> {
    let cookie: tower_cookies::Cookie = cookies.get("session")
        .ok_or_else(|| {
            println!("refresh session cookie not found");
            (StatusCode::UNAUTHORIZED, http::HeaderMap::new(), Json(ServerError::new(RefreshError::LoggedOut, ())))
        })?;
    
    let session_token = BASE64_URL_SAFE_NO_PAD
        .decode(cookie.value())
        .map_err(|err| {
            println!("refresh b64 session token decode failure = {:?}", err);
            (StatusCode::UNAUTHORIZED, http::HeaderMap::new(), Json(ServerError::new(RefreshError::LoggedOut, ())))
        })?;
    let user_session_hash = Sha256::digest(session_token);
    let dbt::Db(pool) = app_state.pool;
    let old_session = crate::db::get_user_session(&pool, user_session_hash.into())
        .await
        .map_err(|err| {
            println!("refresh session db error = {:?}", err);
            (StatusCode::UNAUTHORIZED, http::HeaderMap::new(), Json(ServerError::new(RefreshError::LoggedOut, ())))
        })?;

    let now = chrono::Utc::now();
    if old_session.expires_at < now {
        let mut hm = http::HeaderMap::new();
        let cookie = format!("session=deleted; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0")
            .parse()
            .map_err(|err| {
                println!("{:?}", err);
                (StatusCode::UNAUTHORIZED, http::HeaderMap::new(), Json(ServerError::new(RefreshError::InternalError, ())))
            })?;
        hm.insert(
            axum::http::header::SET_COOKIE,
            cookie,
        );
        return Err((StatusCode::UNAUTHORIZED, http::HeaderMap::new(), Json(ServerError::new(RefreshError::LoggedOut, ()))))
    };

    let models::app_state::JwtSecret(secret) = app_state.secret;
    let token = crate::auth::middleware::create_token(secret.as_bytes(), &old_session.user_id, &old_session.id).map_err(|err| {
        println!("{:?}", err);
        (StatusCode::UNAUTHORIZED, http::HeaderMap::new(), Json(ServerError::new(RefreshError::InternalError, ())))
    })?;
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

    Ok(Json(RefreshResponse { token, expires_at }))
}

#[axum::debug_handler]
pub async fn register(
    State(app_state): State<crate::models::app_state::AppState>,
    user: Json<RegisterUser>
) -> Result<(http::HeaderMap, Json<RefreshResponse>), (StatusCode, Json<ServerError<RegisterError>>)> {
    println!("user = {:?}", &user);
    // TODO check if username is ascii i guess???
    if !user.email.contains("@") {
        return Err((StatusCode::BAD_REQUEST, Json(ServerError::new(RegisterError::InvalidEmail, ()))))
    } else if user.username.len() > 32 {
        return Err((StatusCode::BAD_REQUEST, Json(ServerError::new(RegisterError::UsernameTooLong, ()))))
    }
    // NOTE this checks BYTES not characters... in terms of pw complexity thats fine but uhh dont use chinese for ur password...
    if user.password.len() < 10 {
        return Err((StatusCode::BAD_REQUEST, Json(ServerError::new(RegisterError::PasswordInsecure, ()))))
    } else if user.password.len() > 512 {
        return Err((StatusCode::BAD_REQUEST, Json(ServerError::new(RegisterError::PasswordTooLong, ()))))
    }

    // check if user already exists
    let dbt::Db(pool) = app_state.pool;
    if db::get_user_by_username(&pool, &user.username).await.is_ok() {
        return Err((StatusCode::BAD_REQUEST, Json(ServerError::new(RegisterError::UserAlreadyExists, ()))))
    }

    let hpass = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST)
        .map_err(|err| {
            println!("register bcrypt error = {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(RegisterError::InternalError, ())))
        })?;

    db::insert_user(&pool, &user.username, &user.email, &hpass)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(RegisterError::InternalError, ())))
        })?;

    let (session_token, api_token) = crate::auth::middleware::login(
        &pool,
        &app_state.secret,
        &user.username,
        &user.password,
    )
        .await
        .map_err(|err| {
            println!("register login error = {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(RegisterError::InternalError, ())))
        })?;

    let cookie_value = BASE64_URL_SAFE_NO_PAD.encode(session_token);
    let mut hm = http::HeaderMap::new();
    let cookie = format!("session={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=2592000", cookie_value)
        .parse()
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(RegisterError::InternalError, ())))
        })?;
    hm.insert(
        axum::http::header::SET_COOKIE,
        cookie,
    );

    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);
    Ok((hm, Json(RefreshResponse { token: api_token, expires_at })))
}