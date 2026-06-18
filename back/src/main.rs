use axum::{
    routing::{get, post},
    extract::{FromRef, State},
    Router,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::sync::Arc;
use http::{Request, Response, StatusCode};
use tower_http::{trace::TraceLayer};
use tower_http::cors::{Any, CorsLayer};
use std::convert::Infallible;
use http_body_util::Full;
use tracing_subscriber;
use models::user::{User};

mod db;
mod models;

#[derive(Clone, FromRef)]
struct AppState {
    config: db::Config,
    pool: db::Db,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct LoginUser {
    username: String,
    password: String,
}

#[axum::debug_handler]
async fn register(State(db::Db(pool)): State<db::Db>, user: Json<RegisterUser>) -> impl IntoResponse {
    // TODO explode and complain to the user
    // TODO check if username is ascii i guess???
    if !user.email.contains("@") {
        return (StatusCode::BAD_REQUEST, "birds carry mail")
    } else if user.username.len() > 32 {
        return (StatusCode::BAD_REQUEST, "birds have names")
    }
    // NOTE this checks BYTES not characters... in terms of pw complexity thats fine but uhh dont use chinese for ur password...
    if user.password.len() < 10 {
        return (StatusCode::BAD_REQUEST, "birds risk secrets")
    } else if user.password.len() > 512 {
        return (StatusCode::BAD_REQUEST, "birds spill secrets")
    }

    let existing_user: Result<_, _> = sqlx::query_as::<_, User>("SELECT 1 FROM users WHERE username = $1")
        .bind(&user.username)
        .fetch_one(&pool)
        .await;
    
    if existing_user.is_ok() {
        return (StatusCode::BAD_REQUEST, "birds already vibing")
    }

    // TODO what's a good cost? paranoid.gif
    if let Ok(hpass) = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST) {
        let wat = sqlx::query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)")
            .bind(&user.username)
            .bind(&user.email)
            .bind(hpass)
            .execute(&pool)
            .await;

        match wat {
            Ok(_hmm) => return (StatusCode::OK, ""),
            Err(_no) => return (StatusCode::INTERNAL_SERVER_ERROR, "birds eating disorder")
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "birds cant fly")
    }
}

async fn login(State(db::Db(pool)): State<db::Db>, login_user: Json<LoginUser>) -> Result<(), (StatusCode, &'static str)> {
    let db_user = sqlx::query_as::<_, db::User>("SELECT 1 FROM users WHERE username = $1")
        .bind(&login_user.username)
        .fetch_one(&pool)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "mole not here"))?;

    let did_verify: bool = bcrypt::verify(&login_user.password, &db_user.password_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "mole not pass"))?;

    if did_verify {
        Ok(())
    } else {
        Err((StatusCode::UNAUTHORIZED, "mole wrong phrase"))
    }
}

#[tokio::main]
async fn main() {
    if let Err(wtf) = dotenvy::dotenv() {
        // scream and explode
        panic!("failed to load dotenv: {:?}", wtf)
    }

    let (pool, config) = db::connect().await;
    let state = AppState {
        pool,
        config,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let app = Router::new()
        .route("/", get(|| async { "omg hi baddie 💅" }))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .layer(
            tower::ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        .layer(
            tower::ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
                        .allow_headers(Any)
                        .allow_origin(Any)
                )
        )
        .with_state(state);

    // TODO move this to some config file ? at least the port i g
    let url = "0.0.0.0:3000";
    let listener = TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
