
use axum::extract::{FromRef};
use back::shared::{db as dbt, worker_api::ClientConfig};


#[derive(Clone)]
pub struct JwtSecret(pub String);

#[derive(Clone, FromRef)]
pub struct WorkerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: dbt::Config,
    pub pool: dbt::Db,
    pub secret: JwtSecret,
    pub s3: s3::Client,
    pub client: ClientConfig,
    pub worker: WorkerConfig,
}
