mod utils;
mod config;
mod state;
mod db;
mod models;
mod router;
mod services;
mod handlers;


use actix_web::{App, HttpServer, middleware::{NormalizePath, from_fn}, web};
use config::env::Env;
use utils::logger::{self, http_logger};

use crate::{state::AppState, utils::errors::{json_config, path_config}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_info!("Starting server....");
    
    let env = Env::load();
    logger::init(env.is_production());
    let pool = match db::postgres::pg_connect(&env.database_url).await {
        Ok(pool)=>{
            log_info!("PostgreSQL connected");
            pool
        }
        Err(e)=>{
            log_error!("Could not connect to PostgreSQL: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e)= sqlx::migrate!("./migrations").run(&pool).await {
        log_error!("Migration errors: {e}");
        std::process::exit(1);
    }

    log_info!("Server starting: http://{}:{}", env.host, env.port);

    let addr = (env.host.clone(), env.port);

    let state= web::Data::new(AppState{ db : pool, env });
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .wrap(from_fn(http_logger))
            .app_data(state.clone())
            .app_data(json_config())
            .app_data(path_config())
            .configure(router::configure)
    })
    .bind(addr)?
    .run()
    .await
}