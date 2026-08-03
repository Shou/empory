
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub sqlx::types::Uuid);

impl UserId {
    pub fn to_uuid(&self) -> &sqlx::types::Uuid {
        let UserId(uuid) = self;
        uuid
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
}

pub fn make_user_id(uid: String) -> Result<UserId, uuid::Error> {
    sqlx::types::Uuid::parse_str(&uid).map(|uuid| UserId(uuid))
}