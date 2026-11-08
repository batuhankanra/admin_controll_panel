use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::utils::errors::{ApiError, ApiResult, FieldError};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Published,
    Removed,
    Inactive,
    Active,
    InProgress,
    Developed,
}
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Project{
    pub id : Uuid,
    pub name : String,
    pub stack : Vec<String>,
    pub status : ProjectStatus,
    pub url : Option<String>,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>
}

#[derive(Debug,Deserialize)]
pub struct NewProject{
    pub name:String,
    #[serde(default)]
    pub stack:Vec<String>,
    pub status:Option<ProjectStatus>,
    pub url : Option<String>
}


#[derive(Debug, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub stack: Option<Vec<String>>,
    pub status: Option<ProjectStatus>,
    pub url: Option<String>,
}


fn clean_stack(stack:Vec<String>)->Vec<String>{
    let mut out:Vec<String>=Vec::new();
    for s in stack {
        let s = s.trim().to_string();
        if s.is_empty() && !out.iter().any(|x| x.eq_ignore_ascii_case(&s)){
            out.push(s);
        }
    }
    out
}
fn check_name(name: &str, errors: &mut Vec<FieldError>) {
    if name.is_empty() {
        errors.push(FieldError::new("name", "name cannot be empty"));
    } else if name.chars().count() > 100 {
        errors.push(FieldError::new("name", "name must be at most 100 characters"));
    }
}

fn check_url(url: &str, errors: &mut Vec<FieldError>) {
    if !url.is_empty() && !(url.starts_with("http://") || url.starts_with("https://")) {
        errors.push(FieldError::new("url", "url must start with http:// or https://"));
    }
}


impl NewProject {
    pub fn normalize(mut self)->ApiResult<Self>{
        let mut errors=Vec::new();

        self.name = self.name.trim().to_string();
        check_name(&self.name, &mut errors);
        self.stack=clean_stack(self.stack);
        self.url = self.url.map(|u| u.trim().to_string()).filter(|s| !s.is_empty());
        if let Some(u) = &self.url{
            check_url(u, &mut errors);
        }
        if errors.is_empty() {Ok(self)}else {Err(ApiError::Validation(errors))}
    }

}

impl UpdateProject {
    pub fn normalize(mut self) -> ApiResult<Self> {
        let mut errors = Vec::new();

        if let Some(name) = self.name.take() {
            let name = name.trim().to_string();
            check_name(&name, &mut errors);
            self.name = Some(name);
        }

        self.stack = self.stack.map(clean_stack);

        if let Some(url) = self.url.take() {
            let url = url.trim().to_string();
            check_url(&url, &mut errors);
            self.url = Some(url);
        }

        if errors.is_empty() { Ok(self) } else { Err(ApiError::Validation(errors)) }
    }
}