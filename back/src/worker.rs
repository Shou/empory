
use uuid::Uuid;
use sqlx::types::chrono;
use std::time;

use crate::db;


#[derive(sqlx::FromRow)]
pub struct TimelineJob {
}

async fn process_next_job(db::Db(pool): &db::Db) -> Result<(), ()> {
    let query = r#"
        SELECT id, post_id
        FROM timeline_jobs
        ORDER BY id
        FOR UPDATE SKIP LOCKED
        LIMIT 1
    "#;
    let job = sqlx::query_as::<_, TimelineJob>(query)
        .fetch_one(pool)
        .await
        .map_err(|err| {
            ()
        })?;
    
    let tquery = r#"
        INSERT INTO timeline (user_id, post_id, created_at)
        SELECT 
    "#;
}

pub async fn timeline_worker(db: db::Db) {
    loop {
        if let Err(err) = process_next_job(&db).await {
            //tracing::error!("{err:?}");
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
    }
}