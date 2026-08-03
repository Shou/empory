
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerError<E, D = ()> {
    pub error: E,
    pub details: D,
}

impl<E, D> ServerError<E, D> {
    pub fn new(error: E, details: D) -> ServerError<E, D> {
        ServerError { error, details }
    }
}

impl<E, D> ServerError<E, D>
where
    E: std::fmt::Display,
    D: Serialize,
{
    pub fn into_json(self) -> serde_json::Value {
        json!({
            "error": self.error.to_string(),
            "details": self.details,
        })
    }
}
