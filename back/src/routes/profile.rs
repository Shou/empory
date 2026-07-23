
use axum::{
    extract::{State, Extension, Multipart},
    Json,
};
use http::{StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::Display;

use crate::{errors::ServerError, models::user::UserId};
use crate::db;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Avatar {
    pub avatar_url: String,
}

#[axum::debug_handler]
pub async fn get_avatar(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
) -> Result<Json<Avatar>, (StatusCode, Json<serde_json::Value>)> {
    println!("get_avatar | {:?}", &user_id);

    let db::Db(db) = app_state.pool;
    let avatar = db::get_avatar(&db, &user_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::NOT_FOUND, Json(json!({ "error": "AvatarNotFound" })))
        })?;

    let avatar_url = avatar.url;
    Ok(Json(Avatar { avatar_url }))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AvatarError {
    AvatarInvalid,
    InternalError,
}

#[axum::debug_handler]
pub async fn upload_avatar(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    mut multipart: Multipart,
) -> Result<Json<Avatar>, (StatusCode, Json<ServerError<AvatarError>>)> {
    println!("upload_avatar | {:?}", &user_id);

    let field = multipart.next_field().await.map_err(|err| {
        println!("uplad_avatar multipart error = {:?}", err);
        (StatusCode::BAD_REQUEST, Json(ServerError::new(AvatarError::AvatarInvalid, ())))
    })?.ok_or_else(|| {
        println!("uplad_avatar why is this an Option...");
        (StatusCode::BAD_REQUEST, Json(ServerError::new(AvatarError::AvatarInvalid, ())))
    })?;
    let data = field.bytes().await.map_err(|err| {
        println!("uplad_avatar file field error = {:?}", err);
        (StatusCode::BAD_REQUEST, Json(ServerError::new(AvatarError::AvatarInvalid, ())))
    })?;

    let response = app_state.s3
        .objects()
        .put("avatars", String::from(user_id))
        .body_bytes(data)
        .send()
        .await;

    println!("upload_avatar response = {:?}", response);

    let avatar_url = format!("/avatars/{}", String::from(user_id));

    let db::Db(db) = app_state.pool;
    let avatar = db::insert_avatar(&db, &user_id, avatar_url)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, Json(ServerError::new(AvatarError::InternalError, ())))
        })?;

    let avatar_url = avatar.url;
    Ok(Json(Avatar { avatar_url }))
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
enum Status {
    Verification, // unused for now
    Onboarding,
    Banned, // unused for now
    LoggedIn,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileStatus<A> {
    pub status: A,
    pub user_id: UserId,
    pub username: String,
    pub avatar_url: Option<String>,
    pub email_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusError {
    InternalError,
}

// TODO this is unfinished. if it never returns an error just remove the Result
#[axum::debug_handler]
pub async fn get_status(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ProfileStatus<String>>, (StatusCode, Json<ServerError<StatusError>>)> {
    println!("get_avatar | {:?}", &user_id);

    let mut status: Status = Status::Verification;
    let db::Db(db) = app_state.pool;

    let user = db::get_user_by_id(&db, &user_id.to_uuid()).await.map_err(|err| {
        println!("get_status db err = {:?}", err);
        (StatusCode::UNAUTHORIZED, Json(ServerError::new(StatusError::InternalError, ())))
    })?;

    let avatar = db::get_avatar(&db, &user_id.to_uuid()).await;
    match avatar {
        Ok(_) => status = Status::LoggedIn,
        Err(_) => status = Status::Onboarding,
    }

    let response = app_state.s3
        .objects()
        .get("avatars", *user_id.to_uuid())
        .send()
        .await;

    println!("s3 avatar = {:?}", response);
    let avatar_url = if response.is_ok() {
        Some(format!("/avatars/{}", String::from(*user_id.to_uuid())))
    } else { None };

    Ok(Json(ProfileStatus {
        status: status.to_string(),
        user_id,
        username: user.username,
        avatar_url,
        email_verified: true,
    }))
}
