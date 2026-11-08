use actix_web::{HttpRequest, HttpResponse, ResponseError, error::{JsonPayloadError, PathError}, http::StatusCode, web};
use serde::Serialize;

use crate::{log_error, log_warn};


#[derive(thiserror::Error,Debug)]
pub enum ApiError{
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(&'static str),
    #[error("{0}")]
    Forbidden(&'static str),
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("{0}")]
    Conflict(&'static str),
    #[error("validation error")]
    Validation(Vec<FieldError>),
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("internal error: {0}")]
    Internal(String),
}
#[derive(Debug,Serialize)]
pub struct FieldError{
    pub field:&'static str,
    pub message:String
}

impl FieldError {
    pub fn new(field:&'static str,message:impl Into<String>)->Self{
        Self { field, message: message.into() }
    }
}

#[derive(Serialize)]
struct ErrorBody<'a>{
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<&'a [FieldError]>,
}


impl ApiError {
    pub fn from_db(e:sqlx::Error,conflict_msg:&'static str)->Self{
        match &e {
            sqlx::Error::Database(db) if db.is_unique_violation()=>{
                ApiError::Conflict(conflict_msg)
            },
            _=>ApiError::Db(e)
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
           
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Db(_) | ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let status=self.status_code();

        let message=if status ==StatusCode::INTERNAL_SERVER_ERROR {
            log_error!("{self}");
            "Internal server error".to_string()
        }else {
            self.to_string()
        };

        let errors=match self {
            ApiError::Validation(list)=>Some(list.as_slice()),
            _=>None
        };
        HttpResponse::build(status).json(ErrorBody {
            errors,
            message
        })
    }
}



pub type ApiResult<T> =Result<T,ApiError>;


pub fn json_config()->web::JsonConfig{
    web::JsonConfig::default().limit(64*1024).error_handler(|err:JsonPayloadError,req:&HttpRequest| {
        log_warn!("invalid path {} {}: {err}", req.method(), req.path());

        ApiError::BadRequest("invalid path parameter".into()).into()
    })
}

pub fn path_config()->web::PathConfig{
    web::PathConfig::default().error_handler(|err:PathError,req:&HttpRequest| {
        log_warn!("invalid path {} {}: {err}", req.method(), req.path());
        ApiError::BadRequest("Invalid path parameter".into()).into()
    })
}