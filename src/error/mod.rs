use actix_web::Error as ActixError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use core::fmt;
use mongodb::error::Error as MongoError;
use serde::Serialize;

#[derive(Debug)]
pub enum AppErrorType {
    Default,
    Mongo,
    Actix,
    InvalidID,
}

#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl AppError {
    pub fn new() -> Self {
        Self {
            message: None,
            cause: Some(String::from("Error")),
            error_type: AppErrorType::Default,
        }
    }
    fn message(&self) -> String {
        match self {
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::Default,
            } => "Default error".to_string(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::Mongo,
            } => "Mongodb error".to_string(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::Actix,
            } => "Actix error".to_string(),
            AppError {
                cause: Some(cause),
                message: None,
                error_type: AppErrorType::InvalidID,
            } => cause.clone(),
            _ => "Unknown Error".to_string(),
        }
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::Default => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Mongo => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Actix => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::InvalidID => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl From<ActixError> for AppError {
    fn from(error: ActixError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::Default,
        }
    }
}

impl From<MongoError> for AppError {
    fn from(error: MongoError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.kind.to_string()),
            error_type: AppErrorType::Mongo,
        }
    }
}

impl From<String> for AppError {
    fn from(error: String) -> AppError {
        AppError {
            message: None,
            cause: Some(error),
            error_type: AppErrorType::Default,
        }
    }
}

impl From<ActixError> for AppErrorType {
    fn from(_error: ActixError) -> AppErrorType {
        AppErrorType::Actix
    }
}

impl From<MongoError> for AppErrorType {
    fn from(_error: MongoError) -> AppErrorType {
        AppErrorType::Mongo
    }
}
impl From<String> for AppErrorType {
    fn from(_error: String) -> AppErrorType {
        AppErrorType::Default
    }
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}
