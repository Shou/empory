
use sqlx::{
    types::chrono,
    Row,
};
use std::time;
use back::shared::db as dbt;


#[derive(sqlx::FromRow)]
pub struct Job {
}

async fn process_next_job(dbt::Db(pool): &dbt::Db) -> Result<(), ()> {
    let query = r#"
        WITH next_job AS (
            SELECT id, payload
            FROM jobs
            WHERE
                type = 'timeline'
                AND available_at <= now()
                AND locked_at IS NULL
            ORDER BY available_at, id
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        ),
        claimed AS (
            UPDATE jobs
            SET locked_at = now()
            FROM next_job
            WHERE jobs.id = next_job.id
            RETURNING jobs.id, jobs.payload
        ),
        inserted AS (
            INSERT INTO timeline (user_id, post_id, created_at)
            SELECT
                f.user_id,
                (claimed.payload->>'post_id')::UUID,
                now()
            FROM claimed
            JOIN follows f
                ON f.followed_id = (claimed.payload->>'user_id')::UUID
            ON CONFLICT (user_id, post_id) DO NOTHING
            RETURNING *
        ),
        deleted AS (
            DELETE FROM jobs
            USING claimed
            WHERE jobs.id = claimed.id
            RETURNING claimed.*
        )
        SELECT
            (SELECT COUNT(*) from next_job) as j_count,
            (SELECT COUNT(*) from claimed) as c_count,
            (SELECT COUNT(*) from inserted) as i_count,
            (SELECT COUNT(*) from deleted) as d_count;
    "#;

    let res: (i64, i64, i64, i64) = sqlx::query_as(query)
        .fetch_one(pool)
        .await
        .map_err(|err| {
            println!("sqlx query failed: {:?}", err);
            ()
        })?;
    
    println!("(job, claimed, inserted, deleted): {:?}", res);
    
    Ok(())
}

async fn has_more_jobs(dbt::Db(pool): &dbt::Db) -> bool {
    let query = r#"
        SELECT EXISTS(
            SELECT 1 FROM jobs WHERE
                type = 'timeline'
                AND available_at <= now()
                AND locked_at IS NULL
        )
    "#;
    sqlx::query_scalar(query)
        .fetch_one(pool)
        .await
        .unwrap_or(false)
}

pub async fn run(db: &dbt::Db) {
    loop {
        if let Err(err) = process_next_job(db).await {
            //tracing::error!("{err:?}");
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
        let should_continue = has_more_jobs(db).await;
        if !should_continue { break }
    }
}