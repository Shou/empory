
use axum::{
    Json, extract::{Extension, Query, State}
};
use http::{StatusCode};
use serde::{Deserialize, Serialize};

use crate::{db::JobTimelinePayload, models::user::UserId};
use crate::models::post::PostId;
use crate::db;
use back::shared::{
    db as dbt,
    worker_api,
    errors::ServerError,
};


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostsQuery {
    #[serde(flatten)]
    pub before_at: Option<chrono::DateTime<chrono::Utc>>,
    pub before_post_id: Option<uuid::Uuid>,
    pub limit: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreatePost {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GetPostsError {
    MissingQueryParam,
    InternalError,
}

fn parse_cursor(query: PostsQuery) -> Result<Option<(chrono::DateTime<chrono::Utc>, uuid::Uuid)>, GetPostsError> {
    match (query.before_at, query.before_post_id) {
        (Some(before_at), Some(before_post_id)) => Ok(Some((before_at, before_post_id))),
        (None, None) => Ok(None),
        _ => {
            Err(GetPostsError::MissingQueryParam)
        }
    }
}

#[axum::debug_handler]
pub async fn get_posts(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    Query(query): Query<PostsQuery>,
) -> Result<Json<Vec<db::Post>>, (StatusCode, Json<ServerError<GetPostsError>>)> {
    println!("get_all_posts | {:?}", &user_id);

    let cursor_opt = parse_cursor(query).map_err(|err| {
        println!("get_posts: missing query param pair");
        (StatusCode::BAD_REQUEST, Json(ServerError::new(err, ())))
    })?;

    let dbt::Db(db) = app_state.pool;
    let method = if let Some((before_at, before_post_id)) = cursor_opt {
        db::get_all_posts_page(&db, &before_at, &before_post_id).await
    } else {
        db::get_all_posts_latest(&db).await
    };
    let posts = method.map_err(|err| {
        println!("{:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(GetPostsError::InternalError, ())))
    })?;

    let json = Json(posts);
    Ok(json)
}

#[axum::debug_handler]
pub async fn get_user_posts(
    State(app_state): State<crate::models::app_state::AppState>,
    axum::extract::Path(UserId(user_id)): axum::extract::Path<UserId>,
    Query(query): Query<PostsQuery>,
) -> Result<Json<Vec<db::Post>>, (StatusCode, Json<ServerError<GetPostsError>>)> {
    println!("get_user_posts | {:?}", &user_id);

    let cursor_opt = parse_cursor(query).map_err(|err| {
        println!("get_posts: missing query param pair");
        (StatusCode::BAD_REQUEST, Json(ServerError::new(err, ())))
    })?;

    let dbt::Db(db) = app_state.pool;
    let method = if let Some((before_at, before_post_id)) = cursor_opt {
        db::get_posts_by_user_id_page(&db, &user_id, &before_at, &before_post_id).await
    } else {
        db::get_posts_by_user_id_latest(&db, &user_id).await
    };
    let posts = method.map_err(|err| {
        println!("{:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(GetPostsError::InternalError, ())))
    })?;

    Ok(Json(posts))
}

#[axum::debug_handler]
pub async fn get_full_posts(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    Query(query): Query<PostsQuery>,
) -> Result<Json<Vec<db::Post>>, (StatusCode, Json<ServerError<GetPostsError>>)> {
    println!("get_all_posts | {:?}", &user_id);

    let cursor_opt = parse_cursor(query).map_err(|err| {
        println!("get_posts: missing query param pair");
        (StatusCode::BAD_REQUEST, Json(ServerError::new(err, ())))
    })?;

    let dbt::Db(db) = app_state.pool;
    let posts: Vec<_> = db::get_all_posts_latest(&db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(GetPostsError::InternalError, ())))
        })?;

    let json = Json(posts);
    Ok(json)
}

#[axum::debug_handler]
pub async fn get_feed(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    Query(query): Query<PostsQuery>,
) -> Result<Json<Vec<db::Post>>, (StatusCode, Json<ServerError<GetPostsError>>)> {
    println!("get_feed | {:?}", &user_id);

    let cursor_opt = parse_cursor(query).map_err(|err| {
        println!("get_posts: missing query param pair");
        (StatusCode::BAD_REQUEST, Json(ServerError::new(err, ())))
    })?;

    let dbt::Db(db) = app_state.pool;
    let method = if let Some((before_at, before_post_id)) = cursor_opt {
        db::get_feed_posts_page(&db, &user_id, &before_at, &before_post_id).await
    } else {
        db::get_feed_posts_latest(&db, &user_id).await
    };
    let posts = method.map_err(|err| {
        println!("{:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError::new(GetPostsError::InternalError, ())))
    })?;

    Ok(Json(posts))
}

#[axum::debug_handler]
pub async fn create_post(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    Json(create_post): Json<CreatePost>
) -> Result<Json<db::Post>, (StatusCode, &'static str)> {
    println!("create_post | {:?} = {:?}", &user_id, &create_post);

    if create_post.content.chars().count() > 300 || create_post.content.len() > 3000 {
        return Err((StatusCode::BAD_REQUEST, "pigeon needs ozempic"))
    }

    let dbt::Db(db) = app_state.pool;
    let query = r#"
        WITH new_post AS (
            INSERT INTO posts (user_id, content) VALUES ($1, $2) RETURNING *
        ),
        new_job AS (
            INSERT INTO jobs (type, payload)
            SELECT
                'timeline',
                jsonb_build_object(
                    'post_id', id,
                    'user_id', user_id
                )
            FROM new_post
        )
        SELECT *
        FROM new_post;
    "#;
    let post: db::Post = sqlx::query_as(query)
        .bind(user_id)
        .bind(&create_post.content)
        .fetch_one(&db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "pigeon not here")
        })?;

    let worker_url: String = format!("http://{}:{}", app_state.worker.host, app_state.worker.port);
    let worker_resp = worker_api::create_job(&app_state.client, worker_url, &post.id.to_uuid()).await;
    println!("worker response = {:?}", worker_resp);

    Ok(Json(post))
}

#[axum::debug_handler]
pub async fn like_post(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(PostId(post_id)): axum::extract::Path<PostId>,
) -> Result<Json<db::Like>, (StatusCode, &'static str)> {
    println!("like_post | {:?} = {:?}", &user_id, &post_id);

    let dbt::Db(db) = app_state.pool;
    let like = db::insert_like(&db, &user_id, &post_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "pigeon not here")
        })?;

    Ok(Json(like))
}

#[axum::debug_handler]
pub async fn unlike_post(
    State(app_state): State<crate::models::app_state::AppState>,
    Extension(UserId(user_id)): Extension<UserId>,
    axum::extract::Path(PostId(post_id)): axum::extract::Path<PostId>,
) -> Result<Json<db::Like>, (StatusCode, &'static str)> {
    println!("unlike_post | {:?} = {:?}", &user_id, &post_id);

    let dbt::Db(db) = app_state.pool;
    let like = db::delete_like(&db, &user_id, &post_id)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            (StatusCode::UNAUTHORIZED, "pigeon not here")
        })?;

    Ok(Json(like))
}