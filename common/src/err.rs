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