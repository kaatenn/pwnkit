use crate::database::Database;
use crate::error::PkError;
use chrono::Local;
use colored::Colorize;
use rusqlite::Connection;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

pub struct Competition {
    pub name: String,
    pub date: String,
}

impl Competition {
    pub fn new(name: String) -> Self {
        Self {
            name,
            date: Local::now().to_rfc3339(),
        }
    }

    pub fn from_row(name: String, date: String) -> Self {
        Self { name, date }
    }

    pub fn list_competitions() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Database::init_db()?;
        let mut stmt = conn.prepare("SELECT name FROM competitions ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

        println!("Competition list:");
        for row in rows {
            println!("- {}", row?);
        }
        Ok(())
    }

    pub fn add_competitions(self: &Self) -> Result<(), PkError> {
        let conn = Database::init_db()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM competitions WHERE name = (?1)")?;
        let count: i64 = stmt.query_row([&self.name], |row| row.get(0))?;

        if count > 0 {
            self.deal_repeat_comp(&conn)?;
            println!("Add competition {}", self.name);
        } else {
            let comp_dir = self.competition_dir();
            conn.execute(
                "INSERT OR IGNORE INTO competitions (name, date) VALUES (?1, ?2)",
                [Some(&self.name), Some(&self.date)],
            )?;
            std::fs::create_dir_all(&comp_dir)?;
            println!("Added competition {}", self.name);
        }
        Ok(())
    }

    fn competition_dir(self: &Self) -> std::path::PathBuf {
        std::path::PathBuf::from(".pwnkit").join(&self.name)
    }

    fn deal_repeat_comp(self: &Self, conn: &Connection) -> Result<(), PkError> {
        eprintln!("{}", "WARNING: Competition already exists!".yellow().bold());
        eprintln!(
            "   Competition '{}' was previously added.",
            self.name.bright_yellow()
        );
        eprint!(
            "{} ",
            "It will cover the current data, Continue anyway? (y/N):".bright_cyan()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let comp_dir = self.competition_dir();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                conn.execute("DELETE FROM competitions WHERE name = (?1)", [&self.name])?;

                conn.execute(
                    "INSERT INTO competitions (name, date) VALUES (?1, ?2)",
                    [&self.name, &self.date],
                )?;
                println!(
                    "{} {}",
                    "Updated competition".green(),
                    self.name.bright_white()
                );
                if comp_dir.exists() {
                    std::fs::remove_dir_all(&comp_dir)?;
                }
                std::fs::create_dir_all(&comp_dir)?;
                Ok(())
            }

            _ => {
                println!("{}", "Operation cancelled.".bright_red());
                Ok(())
            }
        }
    }

    pub fn remove_competition(self: Self) -> Result<(), PkError> {
        let conn = Database::init_db()?;

        let comp_dir = std::path::PathBuf::from(".pwnkit").join(&self.name);
        conn.execute("DELETE FROM competitions WHERE name = (?1)", [&self.name])?;
        if comp_dir.exists() {
            std::fs::remove_dir_all(&comp_dir)?;
        }

        Ok(())
    }
}

impl Display for Competition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.date)
    }
}

impl PartialEq for Competition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
