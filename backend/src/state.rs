use sqlx::PgPool;
use crate::config::env::Env;



pub struct AppState{
    pub db:PgPool,
    pub env:Env,
}