use sqlx::PgPool;
use uuid::Uuid;

use crate::{log_info, models::project::{NewProject, Project, UpdateProject}, utils::errors::{ApiError, ApiResult}};




pub async fn list(db:&PgPool)->ApiResult<Vec<Project>>{
    let items = sqlx::query_as::<_,Project>("SELECT * FROM projects ORDER BY created_at DESC")
        .fetch_all(db)
        .await?;
    Ok(items)
}
pub async fn get_one(db:&PgPool,id:Uuid)->ApiResult<Project>{
    sqlx::query_as::<_,Project>("SELECT * FROM project WHERE id = $1")
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(ApiError::NotFound("projects"))

}
pub async fn create(db: &PgPool, input: NewProject) -> ApiResult<Project> {
    let input = input.normalize()?;

    let project = sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, stack, status, url)
         VALUES ($1, $2, COALESCE($3, 'in_progress'::project_status), $4)
         RETURNING *",
    )
    .bind(&input.name)
    .bind(&input.stack)
    .bind(input.status)
    .bind(&input.url)
    .fetch_one(db)
    .await?;

    log_info!("project created: {} ({})", project.name, project.id);
    Ok(project)
}

pub async fn update(db: &PgPool, id: Uuid, input: UpdateProject) -> ApiResult<Project> {
    if input.name.is_none()
        && input.stack.is_none()
        && input.status.is_none()
        && input.url.is_none()
    {
        return Err(ApiError::BadRequest("no fields to update".into()));
    }

    let input = input.normalize()?;

    let project = sqlx::query_as::<_, Project>(
        "UPDATE projects SET
            name       = COALESCE($2, name),
            stack      = COALESCE($3, stack),
            status     = COALESCE($4, status),
            url        = COALESCE($5, url),
            updated_at = now()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.stack)
    .bind(input.status)
    .bind(&input.url)
    .fetch_optional(db)
    .await?
    .ok_or(ApiError::NotFound("project"))?;

    log_info!("project updated: {}", project.id);
    Ok(project)
}

pub async fn delete(db: &PgPool, id: Uuid) -> ApiResult<()> {
    let result = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("project"));
    }

    log_info!("project deleted: {id}");
    Ok(())
}