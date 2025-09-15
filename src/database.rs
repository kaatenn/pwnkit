use crate::error::PkError::ConfigError;
use crate::error::{DatabaseError, PkError};
use colored::Colorize;
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

        let conn = Connection::open(Self::database_path())
            .map_err(|e| DatabaseError::SqliteError(e.to_string()))?;

        Self::init_competition_table(&conn)?;
        Self::init_question_table(&conn)?;
        Self::init_config_table(&conn)?;

        Ok(conn)
    }

    fn init_competition_table(conn: &Connection) -> Result<(), DatabaseError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS competitions (
                name TEXT PRIMARY KEY,
                date TEXT NOT NULL)",
            [],
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(())
    }

    fn init_question_table(conn: &Connection) -> Result<(), DatabaseError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS questions (
                name TEXT NOT NULL,
                competition TEXT NOT NULL,
                tags TEXT,
                PRIMARY KEY (name, competition),
                FOREIGN KEY (competition) REFERENCES competitions(name) ON DELETE CASCADE
            )",
            [],
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(())
    }

    fn init_config_table(conn: &Connection) -> Result<(), DatabaseError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(())
    }

    pub fn get_config(key: &str) -> Result<Option<String>, PkError> {
        let conn = Self::init_db()?;
        let mut stmt = conn
            .prepare("SELECT value FROM config WHERE key = ?1")
            .map_err(|e| DatabaseError::SqliteError(e.to_string()))?;
        let mut rows = stmt
            .query(&[key])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
        {
            let value = row
                .get::<_, String>(0)
                .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
            Ok(Some(value))
        } else {
            Err(ConfigError(
                format!("{} is not an valid property!", key)
                    .bright_red()
                    .to_string(),
            ))
        }
    }

    pub fn set_config(key: &str, value: &str) -> Result<(), PkError> {
        let conn = Self::init_db()?;
        let now = chrono::Local::now().to_rfc3339();
        conn.execute(
            "
        INSERT OR REPLACE INTO config (key, value, created_at, updated_at) 
        VALUES 
        (?1, ?2, COALESCE((SELECT created_at FROM config WHERE key = ?1), ?3), ?3)",
            [key, value, &now],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(())
    }
}
