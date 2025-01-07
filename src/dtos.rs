use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

use crate::models::User;

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default, ToSchema)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(
        email(message = "Invalid email"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,

    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.is_empty() {
        return Err(ValidationError::new("Password is required"));
    }
    if password.len() < 6 {
        return Err(ValidationError::new(
            "Password must be at least 6 characters",
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default, ToSchema)]
pub struct LoginUserDto {
    #[validate(
        email(message = "Invalid email"),
        length(min = 1, message = "Email is required")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, IntoParams)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            role: user.role.to_str().to_string(),
            photo: user.photo.to_owned(),
            verified: user.verified,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(users: Vec<User>) -> Vec<Self> {
        users.iter().map(Self::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResonseDto {
    pub status: String,
    pub data: UserDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListResonseDto {
    pub status: String,
    pub data: Vec<FilterUserDto>,
    result: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserLoginResonseDto {
    pub status: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
