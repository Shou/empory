
use axum::extract::{FromRef};

use crate::db;


#[derive(Clone)]
pub struct JwtSecret(pub String);

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: db::Config,
    pub pool: db::Db,
    pub secret: JwtSecret,
}