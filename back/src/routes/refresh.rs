use axum::{
    extract::{State},
    Json,
};
use serde::{Deserialize, Serialize};
use http::{StatusCode};

use crate::models;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[utoipa::path(
    get,
    path = "/auth/refresh",
    params(
        ("idk" = String, Cookie, description = "HTTP-only session cookie"),
    ),
    responses(
        (status = 200, description = "User authentication login", body = String)
    )
)]
pub async fn refresh(
    State(app_state): State<models::app_state::AppState>,
    cookies: tower_cookies::Cookies,
) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    let cookie: tower_cookies::Cookie = cookies.get("Authorization")
        .ok_or_else(|| {
            println!("refresh auth cookie not found");
            (StatusCode::UNAUTHORIZED, "beaver lost cookie")
        })?;
    let token = cookie.value().strip_prefix("Bearer ")
        .ok_or_else(|| {
            println!("refresh: failed to strip_prefix on Authorization cookie");
            (StatusCode::UNAUTHORIZED, "beaver lacks string")
        })?;
    let models::app_state::JwtSecret(secret) = app_state.secret;
    let claim = crate::auth::middleware::validate_token(token, secret.as_bytes())
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "beaver feels invalid")
        })?;

    println!("yo are we good? token = {:?} | claim = {:?}", token, claim);
    let crate::models::user::UserId(user_id) = crate::models::user::make_user_id(claim.sub)
        .map_err(|err| {
            println!("make_user_id failed = {:?}", err);
            (StatusCode::UNAUTHORIZED, "beaver lacks identity")
        })?;

    let token = crate::auth::middleware::create_token(user_id, secret.as_bytes()).map_err(|err| {
        println!("{:?}", err);
        (StatusCode::UNAUTHORIZED, "beaver token disaster")
    })?;
    let mut hm = http::HeaderMap::new();
    let cookie = format!("Authorization=Bearer {}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400", token)
        .parse()
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "beaver gagged asf")
        })?;
    hm.insert(
        axum::http::header::SET_COOKIE,
        cookie,
    );
    let body = Json(serde_json::json!({ "token": token }));
    Ok(body)
}