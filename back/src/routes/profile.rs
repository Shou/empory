
use axum::{
    extract::{State, Extension, Multipart},
    Json,
};
use http::{StatusCode};
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::models::user::{UserId};
use crate::db;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Avatar {
    pub avatar_url: String,
}

#[axum::debug_handler]
pub async fn get_avatar(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
) -> Result<Json<Avatar>, (StatusCode, &'static str)> {
    println!("get_avatar | {:?}", &user_id);

    let db::Db(db) = app_state.pool;
    let avatar = db::get_avatar(&db, &user_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::NOT_FOUND, "cat not here")
        })?;

    let response = app_state.s3
        .objects()
        .get("avatars", String::from(user_id))
        .send()
        .await;

    println!("get_avatar response = {:?}", response);

    let avatar_url = format!("http://localhost:9000/profile_avatars/{}", String::from(user_id));

    let avatar_url = avatar.url;
    Ok(Json(Avatar { avatar_url }))
}

#[axum::debug_handler]
pub async fn upload_avatar(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    mut multipart: Multipart,
) -> Result<Json<Avatar>, (StatusCode, &'static str)> {
    println!("upload_avatar | {:?}", &user_id);

    if false {
        return Err((StatusCode::BAD_REQUEST, "cat needs ozempic"))
    }

    let field = multipart.next_field().await.map_err(|err| {
        println!("uplad_avatar multipart error = {:?}", err);
        (StatusCode::BAD_REQUEST, "cat is hungry")
    })?.ok_or_else(|| {
        println!("uplad_avatar why is this an option...");
        (StatusCode::BAD_REQUEST, "cat bowl empty")
    })?;
    let data = field.bytes().await.map_err(|err| {
        println!("uplad_avatar file field error = {:?}", err);
        (StatusCode::BAD_REQUEST, "cat bowl empty")
    })?;

    let response = app_state.s3
        .objects()
        .put("avatars", String::from(user_id))
        .body_bytes(data)
        .send()
        .await;

    println!("upload_avatar response = {:?}", response);

    let avatar_url = format!("http://localhost:9000/profile_avatars/{}", String::from(user_id));

    let db::Db(db) = app_state.pool;
    let avatar = db::insert_avatar(&db, &user_id, avatar_url)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "cat not here")
        })?;

    let avatar_url = avatar.url;
    Ok(Json(Avatar { avatar_url }))
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
enum Status {
    Guest,
    Verification, // unused for now
    Onboarding,
    Banned, // unused for now
    LoggedIn,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileStatus<A> {
    status: A,
    user_id: UserId,
    username: String,
    avatar_url: Option<String>,
    email_verified: bool,
}

#[axum::debug_handler]
pub async fn get_status(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ProfileStatus<String>>, (StatusCode, &'static str)> {
    println!("get_avatar | {:?}", &user_id);

    let db::Db(db) = app_state.pool;
    let avatar = db::get_avatar(&db, &user_id.to_uuid())
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::NOT_FOUND, "cat not here")
        })?;

    let response = app_state.s3
        .objects()
        .get("avatars", String::from(*user_id.to_uuid()))
        .send()
        .await;

    println!("get_avatar response = {:?}", response);

    let status: Status = Status::Guest;
    let avatar_url = Some(format!("http://localhost:9000/profile_avatars/{}", String::from(*user_id.to_uuid())));
    let username: String = "".into();
    let email_verified: bool = true;

    Ok(Json(ProfileStatus {
        status: status.to_string(),
        user_id,
        username,
        avatar_url,
        email_verified,
    }))
}