use sea_orm::{DbErr, TransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Database error: {0}")]
    DbError(#[from] DbErr),

    #[error("Transaction Error: {0}")]
    TransactionError(#[from] Box<TransactionError<ServiceError>>),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found error: {0}")]
    NotFoundError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("Conflict error: {0}")]
    ConflictError(String),

    #[error("Biz error: {0}")]
    BizError(String),

    #[error("System error: {0}")]
    SystemError(String),
}

pub trait ErrorCode {
    fn error_code(&self) -> u32;
}

impl ErrorCode for ServiceError {
    fn error_code(&self) -> u32 {
        match self {
            ServiceError::Unauthorized(_) => 401,
            ServiceError::DbError(_) => 500,
            ServiceError::TransactionError(_) => 500,
            ServiceError::ValidationError(_) => 400,
            ServiceError::NotFoundError(_) => 404,
            ServiceError::UnknownError(_) => 520,
            ServiceError::ConflictError(_) => 409,
            ServiceError::BizError(_) => 422,
            ServiceError::SystemError(_) => 500,
        }
    }
}

#[derive(Error, Debug)]
pub enum UserFriendlyError {
    #[error("用户已存在")]
    ConflictError { err_code: u32 },

    #[error("系统错误，请稍后重试")]
    SystemError { err_code: u32 },

    #[error("其他错误: {message}")]
    OtherError { err_code: u32, message: String },
}

impl UserFriendlyError {
    pub fn err_code(&self) -> u32 {
        match self {
            UserFriendlyError::ConflictError { err_code } => *err_code,
            UserFriendlyError::SystemError { err_code } => *err_code,
            UserFriendlyError::OtherError { err_code, .. } => *err_code,
        }
    }
}

impl ServiceError {
    /// 将 ServiceError 转换为 UserFriendlyError
    pub fn to_user_friendly_error(&self) -> UserFriendlyError {
        match self {
            ServiceError::ConflictError(_) => UserFriendlyError::ConflictError { err_code: 409 },
            ServiceError::SystemError(_) => UserFriendlyError::SystemError { err_code: 500 },
            _ => UserFriendlyError::OtherError {
                err_code: 520,
                message: self.to_string(),
            },
        }
    }
}