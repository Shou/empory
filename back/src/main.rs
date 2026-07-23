
pub mod auth;
pub mod db;
pub mod models;
pub mod routes;
pub mod errors;
pub mod worker;

use axum::{
    Router, extract::DefaultBodyLimit, routing::{delete, get, post}
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
    let Ok(s3endpoint) = std::env::var("S3_ENDPOINT") else {
        panic!("failed to load S3 endpoint from env")
    };

    let (pool, config) = db::connect().await;
    let Ok(s3) = s3::Client::builder(s3endpoint)
        .and_then(|client| {
            let env_auth = s3::Auth::from_env()?;
            let s3::Auth::Static(sauth) = &env_auth else {
                panic!("umm this is literally just for debugging anyway......");
            };
            println!("S3 env auth, access = {:?}, secret = {:?}", sauth.access_key_id, sauth.secret_access_key);
            client.region("eu-west-2").auth(env_auth).build()
        }) else {
            panic!("failed to load S3 credentials from env")
        };
    
    let wpool = pool.clone();
    let state = AppState {
        pool,
        config,
        secret,
        s3,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let public_routes = Router::new()
        .route("/auth/register", post(crate::routes::auth::register))
        .route("/auth/login", post(crate::routes::auth::login))
        .route("/auth/logout", post(crate::routes::auth::logout))
        .route("/auth/refresh", get(crate::routes::auth::refresh));

    let private_routes = Router::new()
        .route("/", get(|| async { "omg hi baddie 💅" }))
        .route("/posts", get(crate::routes::posts::get_posts))
        .route("/posts", post(crate::routes::posts::create_post))
        .route("/posts/{post_id}/like", post(crate::routes::posts::like_post))
        .route("/posts/{post_id}/like", delete(crate::routes::posts::unlike_post))
        //.route("/posts/{post_id}", get(crate::routes::posts::get_user_posts))
        .route("/feed", get(crate::routes::posts::get_feed))
        .route("/me", get(crate::routes::profile::get_status))
        .route("/profile/avatar", get(crate::routes::profile::get_avatar))
        .route(
            "/profile/avatar",
            post(crate::routes::profile::upload_avatar).layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        )
        .route("/user/{user_id}/follow", get(crate::routes::users::get_followers))
        .route("/user/{user_id}/follow", post(crate::routes::users::follow_user))
        .route("/user/{user_id}/follow", delete(crate::routes::users::unfollow_user))
        .route("/user/{user_id}/posts", get(crate::routes::posts::get_user_posts))

        .layer(axum::middleware::from_fn_with_state(state.clone(), crate::auth::middleware::auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .layer(
            tower::ServiceBuilder::new().layer(TraceLayer::new_for_http())
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
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .with_state(state);

    tokio::spawn(async move {
        worker::timeline_worker(wpool).await;
    });

    let url = "0.0.0.0:".to_string() + &std::env::var("BACK_PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
