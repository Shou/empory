
pub mod app_state;
pub mod middleware;
pub mod routes;
pub mod worker;

use base64::prelude::*;
use ed25519_dalek::VerifyingKey;
use sqlx::types::chrono;
use std::time;
use axum::{
    Router, extract::DefaultBodyLimit, routing::{get, post}
};
use tokio::net::TcpListener;
use tower_http::{trace::TraceLayer};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

use app_state::{AppState};
use back::shared::db as dbt;


#[tokio::main]
async fn main() {
    let env_filename = ".env.".to_string() + &std::env::var("APP_ENV").unwrap_or("dev".to_string());
    if let Err(wtf) = dotenvy::from_filename(env_filename) {
        // scream and explode
        panic!("failed to load dotenv: {:?}", wtf)
    }

    let (pool, config) = dbt::connect().await;
    let wpool = pool.clone();

    let state = AppState {
        pool,
        config,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let private_routes = Router::new()
        .route("/", get(|| async { "omg hi baddie 💅" }))
        //.route("/job", get(crate::routes::get_job))
        .route("/job", post(crate::routes::post_job))
        //.route("/health", get(crate::routes::get_health))

        .layer(axum::middleware::from_fn_with_state(state.clone(), middleware::auth_middleware));

    let app = Router::new()
        .merge(private_routes)
        .layer(tower::ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(
            tower::ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
                        .allow_headers(Any)
                        .allow_origin(Any)
                )
        )
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .with_state(state);

    // Run the worker on startup
    tokio::spawn(async move {
        worker::run(&wpool).await;
    });

    let url = "0.0.0.0:".to_string() + &std::env::var("WORKER_PORT").unwrap_or("3002".to_string());
    let listener = TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}