
use axum::{
    Json, extract::{Request, State}
};
use http::{StatusCode};

use back::shared::worker_api;
use crate::app_state;


// TODO wake the queue up, make it poll the DB outbox/jobs - in case we missed new pending ones (e.g. network failure, process crashed, etc)
#[axum::debug_handler]
pub async fn post_job(
    State(app_state): State<app_state::AppState>,
    request: Request,
    //Json(create_post): Json<worker_api::CreateJob>,
) -> Result<StatusCode, (StatusCode, Json<worker_api::CreateJobError>)> {
    tokio::spawn(async move {
        crate::worker::run(&app_state.pool).await;
    });
    Ok(StatusCode::OK)
}
