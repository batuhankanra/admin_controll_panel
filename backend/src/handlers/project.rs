use actix_web::{HttpResponse, web};

use crate::{services, state::AppState, utils::errors::ApiResult};




pub async fn list(state:web::Data<AppState>)->ApiResult<HttpResponse>{
    let items = services::project::list(&state.db).await?;
    Ok(HttpResponse::Ok().json(items))
}