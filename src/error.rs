use thiserror::Error;

#[derive(Debug, Error)]
pub enum PkError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    
    #[error("fmt error: {0}")]
    FmtError(String),
    
    #[error("Config error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    #[error("Query execution error: {0}")]
    QueryError(String),

    #[error("SQLite error: {0}")]
    SqliteError(String),
}
