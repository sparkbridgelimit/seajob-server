use sea_orm::{DbErr, TransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
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
}
