
use axum::extract::{State};
use http::{StatusCode};
use serde::{Deserialize, Serialize};

use crate::models::app_state::{JwtSecret};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}


pub fn create_token(user_id: sqlx::types::Uuid, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + chrono::Duration::hours(24)).timestamp(),
        iat: now.timestamp(),
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    ).map(|data| data.claims)
}

pub async fn auth_middleware(
    State(JwtSecret(secret)): State<JwtSecret>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    // HACK? idk seems like this isnt a good way to scale this...
    let url = request.uri().path();
    println!("auth_middleware {:?}", &url);
    if url.starts_with("/auth") {
        return Ok(next.run(request).await)
    }

    let token = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claim = validate_token(token, secret.as_bytes())
        .map_err(|err| {
            println!("{:?}", err);
            StatusCode::UNAUTHORIZED
        })?;

    println!("yo are we good? token = {:?} | claim = {:?}", token, claim);
    let user_id = crate::models::user::make_user_id(claim.sub)
        .map_err(|err| {
            println!("make_user_id failed = {:?}", err);
            StatusCode::UNAUTHORIZED
        })?;
    let (mut parts, body) = request.into_parts();
    parts.extensions.insert(user_id);

    let req = http::Request::from_parts(parts, body);
    Ok(next.run(req).await)
}