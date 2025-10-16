use crate::config;
use crate::database::Database;
use crate::error::PkError;
use colored::Colorize;
use rusqlite::{params, Connection};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct Template {
    pub name: String,
}

impl Template {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_path(name: String) -> PathBuf {
        config::root().join(format!("{}.py", name))
    }
    
    pub fn list_templates() -> Result<(), PkError> {
        let conn = Database::init_db()?;
        let mut stmt = conn.prepare("SELECT name FROM templates ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

        println!("Template list:");
        for row in rows {
            println!("- {}", row?);
        }
        Ok(())
    }

    pub fn add_templates(path: PathBuf, name: String) -> Result<(), PkError> {
        let conn = Database::init_db()?;

        if Self::template_exists(&conn, &name)? {
            Self::handle_duplicate_template(&conn, &name, &path)?;
        } else {
            Self::insert_and_copy(&conn, &name, &path)?;
            println!("Added template {}", name);
        }

        Ok(())
    }

    fn template_exists(conn: &Connection, name: &str) -> Result<bool, PkError> {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM templates WHERE name = ?")?;
        let count: i64 = stmt.query_row([name], |row| row.get(0))?;
        Ok(count > 0)
    }

    fn insert_and_copy(conn: &Connection, name: &str, src: &Path) -> Result<(), PkError> {
        conn.execute(
            "INSERT INTO templates (name) VALUES ($1)",
            params![name],
        )?;

        let target_path = Self::get_path(String::from(name));
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if target_path.exists() {
            fs::remove_file(&target_path)?;
        }
        fs::copy(&src, &target_path)?;
        Ok(())
    }

    fn handle_duplicate_template(
        conn: &Connection,
        name: &str,
        src: &Path,
    ) -> Result<(), PkError> {
        eprintln!("{}", "WARNING: Template already exists!".yellow().bold());
        eprintln!(
            "   Template '{}' was previously added.",
            name.bright_yellow()
        );
        eprint!(
            "{} ",
            "It will overwrite the current data, Continue anyway? (y/N):".bright_cyan()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                // Remove existing DB record and file
                conn.execute("DELETE FROM templates WHERE name = $1", params![name])?;
                let existing = Self::get_path(String::from(name));
                if existing.exists() {
                    fs::remove_file(existing)?;
                }

                // Re-insert and copy new file
                Self::insert_and_copy(conn, name, src)?;
                println!(
                    "{} {}",
                    "Updated template".green(),
                    name.bright_white()
                );
                Ok(())
            }
            _ => {
                println!("{}", "Operation cancelled.".bright_red());
                Ok(())
            }
        }
    }

    pub fn remove_templates(name: String) -> Result<(), PkError> {
        let conn = Database::init_db()?;
        conn.execute("DELETE FROM templates WHERE name = $1", params![name])?;
        let path = Self::get_path(name);
        fs::remove_file(path)?;
        Ok(())
    }
}
