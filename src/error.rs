use thiserror::Error;

#[derive(Debug, Error)]
pub enum PkError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
