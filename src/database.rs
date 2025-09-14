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

        Self::init_competition_table(&conn)?;
        Self::init_question_table(&conn)?;

        Ok(conn)
    }

    fn init_competition_table(conn: &Connection) -> Result<(), PkError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS competitions (
                name TEXT PRIMARY KEY,
                date TEXT NOT NULL)",
            [],
        )?;
        Ok(())
    }

    fn init_question_table(conn: &Connection) -> Result<(), PkError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS questions (
                name TEXT NOT NULL,
                competition TEXT NOT NULL,
                tags TEXT,
                PRIMARY KEY (name, competition),
                FOREIGN KEY (competition) REFERENCES competitions(name) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(())
    }
}
