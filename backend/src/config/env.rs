use std::{default, env, error, fmt, str::FromStr};


#[derive(Clone)]
pub struct Env{
    pub database_url:String,
    pub jwt_secret:String,
    pub jwt_expires_hours:i64,
    pub host:String,
    pub port :u16,
    pub frontend_origin:String,
    pub app_env:String,
}

impl Env {
    pub fn load()->Self{
        dotenvy::dotenv().ok();

        let jwt_secret = required("JWT_SECRET");
        if jwt_secret.len() < 32 {
            panic!("JWT_SECRET en az 32 karakter olmalı");
        }

        Self {
            database_url: required("DATABASE_URL"),
            jwt_secret,
            jwt_expires_hours: optional("JWT_EXPIRES_HOURS", 24),
            host: optional("HOST", "127.0.0.1".to_string()),
            port: optional("PORT", 8080),
            frontend_origin: optional("FRONTEND_ORIGIN", "http://localhost:5173".to_string()),
            app_env: optional("APP_ENV", "development".to_string()),
        }
        
    }
    pub fn is_production(&self) -> bool {
        self.app_env.eq_ignore_ascii_case("production")
    }

}

fn required(key:&str)->String{
    env::var(key).unwrap_or_else(|_| panic!("{key} .env not defined in the file"))
}
fn optional<T:FromStr>(key:&str,default:T)->T{
    match env::var(key) {
        Ok(v)=>v.parse().unwrap_or_else(|_| panic!("{key} not a valid value: {v}")),
        Err(_)=>default,
    }
}
impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Env")
            .field("database_url", &"***")
            .field("jwt_secret", &"***")
            .field("jwt_expires_hours", &self.jwt_expires_hours)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("frontend_origin", &self.frontend_origin)
            .finish()
    }
}