mod utils;
mod config;
mod state;

use actix_web::{middleware::from_fn, web, App, HttpServer};
use config::env::Env;
use utils::logger::{self, http_logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::load();
    logger::init(env.is_production());

    log_info!("Sunucu başlıyor: http://{}:{}", env.host, env.port);

    let addr = (env.host.clone(), env.port);

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(http_logger))
            .app_data(web::Data::new(env.clone()))
    })
    .bind(addr)?
    .run()
    .await
}