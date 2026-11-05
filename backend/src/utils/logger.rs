use std::fmt::Arguments;
use std::fs::{self,File,OpenOptions};
use std::io::Write;
use std::sync::atomic::{AtomicBool,AtomicU8,Ordering};
use std::sync::{Mutex,OnceLock};
use std::time::Instant;

use actix_web::{body::MessageBody,dev::{ServiceRequest,ServiceResponse},middleware::Next,Error};
use chrono::{FixedOffset,Utc};


const LOG_DIR: &str = "log";
const LOG_FILE: &str = "log/app.log";
const TR_OFFSET_SECS: i32 = 3 * 3600;
const TIME_FORMAT: &str = "%d-%m-%Y %H:%M";


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Level{
    Debug=0,
    Info=1,
    Warn=2,
    Error=3
}
impl Level {
    fn as_str(self)->&'static str{
        match self {
            Level::Debug=>"DEBUG",
            Level::Error=>"ERROR",
            Level::Info=>"INFO",
            Level::Warn=>"WARN"
        }
    }
    fn parse(s:&str)->Option<Self>{
        match s.trim().to_ascii_lowercase().as_str() {
            "debug"=>Some(Level::Debug),
            "error"=>Some(Level::Error),
            "info"=>Some(Level::Info),
            "warn"=>Some(Level::Warn),
            _=>None,
        }
    }
}


static MIN_LEVEL:AtomicU8=AtomicU8::new(Level::Info as u8);
static JSON_FORMAT:AtomicBool=AtomicBool::new(false);
static FILE:OnceLock<Mutex<File>>=OnceLock::new();

pub fn init(is_production:bool){
    JSON_FORMAT.store(is_production, Ordering::Relaxed);
    if let Some(level)=std::env::var("LOG_LEVEL").ok().as_deref().and_then(Level::parse){
        MIN_LEVEL.store(level as u8, Ordering::Relaxed);
    }
    let opened = fs::create_dir_all(LOG_DIR).and_then(| _ | OpenOptions::new().create(true).append(true).open(LOG_FILE));
    match opened {
        Ok(file)=>{
            let _ =FILE.set(Mutex::new(file));
        }
        Err(e)=>eprintln!("Log file could not be opened ({LOG_FILE}):{e} ")
    }
}

fn now_tr()->String{
    let tz = FixedOffset::east_opt(TR_OFFSET_SECS).expect("Current offset");
    Utc::now().with_timezone(&tz).format(TIME_FORMAT).to_string()
}

pub fn log(level:Level,target:&str,args:Arguments){
    if (level as u8) < MIN_LEVEL.load(Ordering::Relaxed){
        return;
    }
    let ts =now_tr();
    let line =if JSON_FORMAT.load(Ordering::Relaxed){
        serde_json::json!({
            "ts":ts,
            "level":level.as_str(),
            "target":target,
            "message":args.to_string(),
        }).to_string()
    }else {
        format!("[{ts} {:<5} {target}] {args}", level.as_str())
    };

    eprintln!("{line}");
    if let Some(file)=FILE.get(){
        if let Ok(mut f)= file.lock(){
            let _ =writeln!(f,"{line}");
        }
    }
}
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::utils::logger::log($crate::utils::logger::Level::Debug, module_path!(), format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::utils::logger::log($crate::utils::logger::Level::Info, module_path!(), format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::utils::logger::log($crate::utils::logger::Level::Warn, module_path!(), format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::utils::logger::log($crate::utils::logger::Level::Error, module_path!(), format_args!($($arg)*))
    };
}


pub async fn http_logger(req:ServiceRequest,next:Next<impl MessageBody>)->Result<ServiceResponse<impl MessageBody>,Error>{
    let start = Instant::now();
    let method = req.method().clone();
    let path = req.path().to_owned();
    let ip = req.peer_addr().map(|a| a.ip().to_string()).unwrap_or_else(||"-".into());

    let res = next.call(req).await;

    let ms = start.elapsed().as_millis();
    let status = match &res {
        Ok(r)=>r.status().as_u16(),
        Err(e)=>e.as_response_error().status_code().as_u16()
    };
    let level = match status {
        500.. => Level::Error,
        400..=499 => Level::Warn,
        _ => Level::Info
    };
    log(level, "http", format_args!("{ip} \"{method} {path}\" {status} {ms}ms"));
    res
}