
use axum::extract::{FromRef};
use std::sync::Arc;
use std::collections::HashMap;

use crate::db;


#[derive(Clone)]
pub struct JwtSecret(pub String);

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: db::Config,
    pub pool: db::Db,
    pub secret: JwtSecret,
    pub s3: s3::Client,
}