
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
#[sqlx(transparent)]
pub struct PostId(pub sqlx::types::Uuid);

impl PostId {
    pub fn to_uuid(&self) -> &sqlx::types::Uuid {
        let PostId(uuid) = self;
        uuid
    }
}