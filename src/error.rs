use rusqlite::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PkError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] Error),
    
    #[error("fmt error: {0}")]
    FmtError(String),
    
    #[error("Config error: {0}")]
    ConfigError(String),
}
