use thiserror::Error;
use uuid::Uuid;

/// Domain errors for the item service.
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("item not found: {0}")]
    NotFound(Uuid),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("item already exists: {0}")]
    Conflict(Uuid),

    #[error("storage error: {0}")]
    Storage(#[from] anyhow::Error),
}

/// Convenience alias used throughout the crate.
pub type ServiceResult<T> = Result<T, ServiceError>;
