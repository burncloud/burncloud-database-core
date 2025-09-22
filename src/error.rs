use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Not found")]
    NotFound,

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Database configuration error: {0}")]
    ConfigurationError(String),

    #[error("Internal database error: {0}")]
    InternalError(String),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;