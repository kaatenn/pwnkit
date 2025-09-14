use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Database error: {0}")]
    DatabaseInitError(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
