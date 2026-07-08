use axum::{
    routing::{get, post},
    extract::{FromRef, State},
    Router,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use http::{StatusCode};
use tower_http::{trace::TraceLayer};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use crate::models::user::{User};
use crate::db;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct RegisterUser {
    username: String,
    email: String,
    password: String,
}


#[utoipa::path(
    post,
    path = "/auth/register",
    request_body(content = RegisterUser, description = "Register user", content_type = "application/json"),
    responses(
        (status = 200, description = "User authentication login", body = String)
    )
)]
#[axum::debug_handler]
pub async fn register(
    State(app_state): State<crate::models::app_state::AppState>,
    user: Json<RegisterUser>
) -> Result<(http::HeaderMap, Json<serde_json::Value>), (StatusCode, &'static str)> {
    println!("user = {:?}", &user);
    // TODO explode and complain to the user
    // TODO check if username is ascii i guess???
    if !user.email.contains("@") {
        return Err((StatusCode::BAD_REQUEST, "birds carry mail"))
    } else if user.username.len() > 32 {
        return Err((StatusCode::BAD_REQUEST, "birds have names"))
    }
    // NOTE this checks BYTES not characters... in terms of pw complexity thats fine but uhh dont use chinese for ur password...
    if user.password.len() < 10 {
        return Err((StatusCode::BAD_REQUEST, "birds risk secrets"))
    } else if user.password.len() > 512 {
        return Err((StatusCode::BAD_REQUEST, "birds spill secrets"))
    }

    // check if user already exists
    let db::Db(pool) = app_state.pool;
    if db::get_user_by_username(&pool, &user.username).await.is_ok() {
        return Err((StatusCode::BAD_REQUEST, "birds already vibing"))
    }

    if let Ok(hpass) = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST) {
        let inserted_user: User = sqlx::query_as("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING new.id, new.username, new.email")
            .bind(&user.username)
            .bind(&user.email)
            .bind(hpass)
            .fetch_one(&pool)
            .await
            .map_err(|err| {
                println!("{:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "birds eating disorder")
            })?;

        let crate::models::app_state::JwtSecret(secret) = app_state.secret;
        let crate::models::user::UserId(user_id) = inserted_user.id;
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
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "birds cant fly"))
    }
}