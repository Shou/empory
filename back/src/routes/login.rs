use axum::{
    extract::{State},
    Json,
};
use serde::{Deserialize, Serialize};
use http::{StatusCode};

use crate::models;
use crate::db;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body(content = LoginUser, description = "Login user", content_type = "application/json"),
    responses(
        (status = 200, description = "User authentication login", body = String)
    )
)]
pub async fn login(
    State(app_state): State<models::app_state::AppState>,
    login_user: Json<LoginUser>,
) -> Result<(http::HeaderMap, Json<serde_json::Value>), (StatusCode, &'static str)> {
    let db::Db(db) = app_state.pool;
    println!("user = {:?}", &login_user);
    let db_user = db::get_user_by_username(&db, &login_user.username)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "mole not here")
        })?;

    let did_verify: bool = bcrypt::verify(&login_user.password, &db_user.password_hash)
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "mole not pass")
        })?;

    if did_verify {
        let models::app_state::JwtSecret(secret) = app_state.secret;
        let crate::models::user::UserId(user_id) = db_user.id;
        let token = crate::auth::middleware::create_token(user_id, secret.as_bytes()).map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "mole token disaster")
        })?;
        let mut hm = http::HeaderMap::new();
        let cookie = format!("Authorization=Bearer {}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400", token)
            .parse()
            .map_err(|err| {
                println!("{:?}", err);
                (StatusCode::UNAUTHORIZED, "mole gagged asf")
            })?;
        hm.insert(
            axum::http::header::SET_COOKIE,
            cookie,
        );
        let body = Json(serde_json::json!({ "token": token }));
        Ok((hm, body))
    } else {
        Err((StatusCode::UNAUTHORIZED, "mole wrong phrase"))
    }
}