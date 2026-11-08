use sqlx::PgPool;

use crate::{models::project::Project, utils::errors::ApiResult};




pub async fn list(db:&PgPool)->ApiResult<Vec<Project>>{
    let items = sqlx::query_as::<_,Project>("SELECT * FROM projects ORDER BY created_at DESC")
        .fetch_all(db)
        .await?;
    Ok(items)
}
