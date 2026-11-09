use actix_web::web;

use crate::handlers;






pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .route("", web::get().to(handlers::project::list))
            .route("", web::post().to(handlers::project::create))
            .route("/{id}", web::get().to(handlers::project::get_one))
            .route("/{id}", web::put().to(handlers::project::update))
            .route("/{id}", web::delete().to(handlers::project::delete)),
    );
}