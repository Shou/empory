use axum::{
    extract::{Extension, State},
    Json,
};
use http::{StatusCode};
use serde::{Deserialize, Serialize};

use crate::models::user::{UserId};
use crate::db;
use back::shared::db as dbt;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FollowUser {
    pub user_id: UserId,
}


#[axum::debug_handler]
pub async fn follow_user(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(UserId(followed_id)): axum::extract::Path<UserId>,
) -> Result<Json<db::Follow>, (StatusCode, &'static str)> {
    println!("follow_user | {:?} = {:?}", &user_id, &followed_id);

    let dbt::Db(pool) = app_state.pool;
    let follow = db::follow_user(&pool, &user_id, &followed_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "panther not here")
        })?;

    Ok(Json(follow))
}

#[axum::debug_handler]
pub async fn unfollow_user(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(UserId(followed_id)): axum::extract::Path<UserId>,
) -> Result<Json<db::Follow>, (StatusCode, &'static str)> {
    println!("follow_user | {:?} = {:?}", &user_id, &followed_id);

    let dbt::Db(pool) = app_state.pool;
    let follow = db::unfollow_user(&pool, &user_id, &followed_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "panther not here")
        })?;

    Ok(Json(follow))
}

#[axum::debug_handler]
pub async fn get_followers(
    State(app_state): State<crate::models::app_state::AppState>,
    axum::extract::Path(UserId(followed_id)): axum::extract::Path<UserId>,
) -> Result<Json<Vec<db::Follow>>, (StatusCode, &'static str)> {
    println!("get_followers | {:?}", &followed_id);

    let dbt::Db(pool) = app_state.pool;
    let follows = db::get_follows(&pool, &followed_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "panther not here")
        })?;

    Ok(Json(follows))
}