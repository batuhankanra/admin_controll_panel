use actix_web::web;

use crate::handlers;






pub fn config(cfg:&mut web::ServiceConfig){
    cfg.service(
        web::scope("/projects")
        .route("", web::get().to(handlers::project::list))
    );
}