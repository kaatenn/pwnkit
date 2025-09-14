use crate::data::competition::Competition;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use rusqlite::Connection;
use crate::error::ConfigError;

#[derive(Serialize, Deserialize, Default)]
pub struct Database {
    competitions: Vec<Competition>,
}

impl Database {
    pub fn config_path() -> PathBuf {
        /* dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".pwnkit/config.json") */
        PathBuf::from(".pwnkit/pwnkit.db")
    }

    fn init_db() -> Result<Connection, ConfigError> {
        let conn = Connection::open(Self::config_path())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS competitions (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                date TEXT NOT NULL)",
            []
        )?;
        Ok(conn)
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }
    pub fn add_competitions(comp: Competition) -> Result<(), ConfigError> {
        let conn = Self::init_db()?;
        conn.execute(
            "INSERT OR IGNORE INTO competitions (name, date) VALUES (?1, ?2)",
            [&comp.name, &comp.date]
        )?;
        println!("Added competition {}", comp.name);
        Ok(())
    }
    pub fn list_competitions() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Self::init_db()?;
        let mut stmt = conn.prepare("SELECT name FROM competitions ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        println!("Competition list:");
        for row in rows {
            println!("- {}", row?);
        }
        Ok(())
    }
}
