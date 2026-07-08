
use axum::{
    extract::{State, Extension},
    Json,
};
use http::{StatusCode};
use serde::{Deserialize, Serialize};

use crate::models::user::{UserId};
use crate::db;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct CreatePost {
    pub content: String,
}

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "Get timeline posts", body = String)
    )
)]
#[axum::debug_handler]
pub async fn get_posts(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
) -> Result<Json<Vec<db::Post>>, (StatusCode, &'static str)> {
    println!("get_all_posts | {:?}", &user_id);
    let db::Db(db) = app_state.pool;
    let posts = db::get_all_posts(&db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "pigeon not here")
        })?;

    let json = Json(posts);
    Ok(json)
}

#[axum::debug_handler]
pub async fn get_user_posts(
    State(app_state): State<crate::models::app_state::AppState>,
    //Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(UserId(user_id)): axum::extract::Path<UserId>,
) -> Result<Json<Vec<db::Post>>, (StatusCode, &'static str)> {
    println!("get_user_posts | {:?}", &user_id);
    let db::Db(db) = app_state.pool;
    let posts = db::get_posts_by_user_id(&db, &user_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "pigeon not here")
        })?;

    let json = Json(posts);
    Ok(json)
}

// TODO replace this with TypeSpec? whats the point of type safety if the interface barely cares...
#[utoipa::path(
    post,
    path = "/posts",
    request_body(content = CreatePost, description = "Create post", content_type = "application/json"),
    responses(
        (status = 200, description = "Create timeline post", body = String)
    )
)]
#[axum::debug_handler]
pub async fn create_post(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    Json(create_post): Json<CreatePost>
) -> Result<Json<db::Post>, (StatusCode, &'static str)> {
    println!("create_post | {:?} = {:?}", &user_id, &create_post);
    let db::Db(db) = app_state.pool;
    let post = db::insert_post(&db, &user_id, &create_post.content)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "pigeon not here")
        })?;

    Ok(Json(post))
}