use crate::data::competition::Competition;
use crate::error::PkError;
use colored::Colorize;
use rusqlite::Connection;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Default)]
pub struct Database;

impl Database {
    pub fn database_path() -> PathBuf {
        /* dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".pwnkit/config.json") */
        PathBuf::from(".pwnkit/pwnkit.db")
    }

    pub fn init_db() -> Result<Connection, PkError> {
        let db_path = Self::database_path();

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(Self::database_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS competitions (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                date TEXT NOT NULL)",
            [],
        )?;
        Ok(conn)
    }

    
}
