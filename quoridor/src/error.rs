use serde::Serialize;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    NotFoundError,
    IllegalArgumentError,
    PlayError
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError { 
                message: Some(message),
                cause: _, 
                error_type: _} => message.clone(),
            AppError { 
                message: None, 
                cause: _, 
                error_type: AppErrorType::NotFoundError } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occured".to_string()
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::IllegalArgumentError => StatusCode::BAD_REQUEST,
            AppErrorType::PlayError => StatusCode::CONFLICT
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .json(AppErrorResponse {error: self.message()})
    }

}
