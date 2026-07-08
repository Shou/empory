
pub mod auth;
pub mod db;
pub mod models;
pub mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::{trace::TraceLayer};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use models::app_state::{JwtSecret, AppState};


#[tokio::main]
async fn main() {
    let env_filename = ".env.".to_string() + &std::env::var("APP_ENV").unwrap_or("dev".to_string());
    if let Err(wtf) = dotenvy::from_filename(env_filename) {
        // scream and explode
        panic!("failed to load dotenv: {:?}", wtf)
    }
    let Ok(secret) = std::env::var("JWT_SECRET").map(|secret| JwtSecret(secret)) else {
        panic!("failed to load JWT secret from env")
    };

    let (pool, config) = db::connect().await;
    let state = AppState {
        pool,
        config,
        secret,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let app = Router::new()
        .route("/", get(|| async { "omg hi baddie 💅" }))
        .route("/auth/register", post(crate::routes::register::register))
        .route("/auth/login", post(crate::routes::login::login))
        .route("/auth/logout", post(crate::routes::logout::logout))
        .route("/auth/refresh", get(crate::routes::refresh::refresh))
        .route("/posts", get(crate::routes::posts::get_posts))
        .route("/posts/{user_id}", get(crate::routes::posts::get_user_posts))
        .route("/posts", post(crate::routes::posts::create_post))
        .layer(axum::middleware::from_fn_with_state(state.clone(), crate::auth::middleware::auth_middleware))
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
        .layer(tower_cookies::CookieManagerLayer::new())
        .with_state(state);

    let url = "0.0.0.0:".to_string() + &std::env::var("BACK_PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
