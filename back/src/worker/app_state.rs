
use axum::extract::{FromRef};
use back::shared::db as dbt;


#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: dbt::Config,
    pub pool: dbt::Db,
}