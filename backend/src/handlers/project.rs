use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::{models::project::{NewProject, UpdateProject}, services, state::AppState, utils::errors::ApiResult};




pub async fn list(state:web::Data<AppState>)->ApiResult<HttpResponse>{
    let items = services::project::list(&state.db).await?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_one(state:web::Data<AppState>,id:web::Path<Uuid>)->ApiResult<HttpResponse>{
    let item =services::project::get_one(&state.db, id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn create(state:web::Data<AppState>,body:web::Json<NewProject>)->ApiResult<HttpResponse>{
    let item =services::project::create(&state.db, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(item))
}

pub async fn update(state:web::Data<AppState>,id:web::Path<Uuid>,body:web::Json<UpdateProject>)->ApiResult<HttpResponse>{
    let item = services::project::update(&state.db, id.into_inner(), body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn delete(state:web::Data<AppState>,id:web::Path<Uuid>)->ApiResult<HttpResponse>{
    services::project::delete(&state.db, id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}