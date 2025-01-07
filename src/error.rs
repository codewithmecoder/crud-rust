use core::fmt;

use actix_web::{body, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::dtos::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ErrorMessage {
    pub fn to_str(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password cannot exceed {} characters", max_length)
            }
            ErrorMessage::HashingError => "Error hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid hash format".to_string(),
            ErrorMessage::InvalidToken => "Invalid token".to_string(),
            ErrorMessage::ServerError => "Server error, Please try again later".to_string(),
            ErrorMessage::WrongCredentials => "Email or password is incorrect".to_string(),
            ErrorMessage::EmailExist => "Email already exist".to_string(),
            ErrorMessage::UserNoLongerExist => "User no longer exist".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::PermissionDenied => "Permission denied".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: u16,
    pub message: String,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        HttpError {
            status,
            message: message.into(),
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            status: 500,
            message: message.into(),
        }
    }

    pub fn permission_denied(message: impl Into<String>) -> Self {
        HttpError {
            status: 403,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            status: 400,
            message: message.into(),
        }
    }
    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            status: 401,
            message: message.into(),
        }
    }

    pub fn uqique_constraint_voilation(message: impl Into<String>) -> Self {
        HttpError {
            status: 409,
            message: message.into(),
        }
    }

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            500 => HttpResponse::InternalServerError().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            403 => HttpResponse::Forbidden().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            400 => HttpResponse::BadRequest().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            401 => HttpResponse::Unauthorized().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            _ => {
                eprintln!(
                    "Warning: Missing pattern match. Converted status code {} to 500",
                    self.status
                );
                HttpResponse::InternalServerError().json(Response {
                    status: "error",
                    message: ErrorMessage::ServerError.into(),
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message {}, status {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}
