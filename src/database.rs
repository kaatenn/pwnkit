use crate::error::PkError;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS questions (
              id TEXT PRIMARY KEY,
              name TEXT NOT NULL,
              competition TEXT NOT NULL,
              tags TEXT,
              FOREIGN KEY (competition) REFERENCES competitions(name)
          )",
            [],
        )?;
        Ok(conn)
    }
}
