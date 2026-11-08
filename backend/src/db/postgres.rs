use sqlx::{postgres::PgPoolOptions,PgPool,Error};
use std::time::Duration;


pub async fn pg_connect(database_url:&str)->Result<PgPool,Error>{
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}