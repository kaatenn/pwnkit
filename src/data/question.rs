use crate::database::Database;
use crate::error::PkError::ConfigError;
use crate::error::PkError;
use crate::utils;
use colored::Colorize;
use rusqlite::ToSql;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::PathBuf;

pub struct Question {
    pub name: String,
    pub competition: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(name: String, competition: String, tags: Option<Vec<String>>) -> Self {
        Self {
            name,
            competition,
            tags,
        }
    }

    pub fn from_row(name: String, competition: String, tags: Option<String>) -> Self {
        let tags_vec = tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());
        Self {
            name,
            competition,
            tags: tags_vec,
        }
    }

    pub fn get_question_path(self: &Self) -> PathBuf {
        // dirs::home_dir()
        //     .unwrap_or_else(|| std::path::PathBuf::from("."))
        //     .join(".pwnkit")
        //     .join(competition)
        //     .join(name)
        PathBuf::from(".pwnkit")
            .join(self.competition.clone())
            .join(self.name.clone())
    }

    pub fn add_question(self: &Self, from_wsl: bool) -> Result<(), PkError> {
        let conn = utils::connect()?;
        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM questions WHERE name = (?1) AND competition = (?2)")?;
        let count: i64 = stmt
            .query_row(&[&self.name, &self.competition], |row| row.get(0))?;

        if count > 0 {
            self.deal_repeat_question(from_wsl)?;
        } else {
            self.create_new_question(from_wsl)?;
            println!("Added question {}", self.name);
        }

        Ok(())
    }

    fn create_new_question(self: &Self, from_wsl: bool) -> Result<(), PkError> {
        let conn = utils::connect()?;

        let question_dir = self.get_question_path();
        let tags_str = self.tags_str();

        conn.execute(
            "INSERT OR IGNORE INTO questions (name, competition, tags) VALUES (?1, ?2, ?3)",
            &[
                &Some(self.name.clone()),
                &Some(self.competition.clone()),
                &Some(tags_str),
            ],
        )?;

        std::fs::create_dir_all(&question_dir)?;
        if from_wsl {
            self.copy_from_windows_downloads()?;
        }
        Ok(())
    }

    fn tags_str(self: &Self) -> String {
        let tags_str = self.tags.clone().map_or("".to_string(), |t| t.join(","));
        tags_str
    }

    fn deal_repeat_question(self: &Self, from_wsl: bool) -> Result<(), PkError> {
        eprintln!("{}", "WARNING: Question already exists!".yellow().bold());
        eprintln!(
            "   Question '{}' in competition '{}' was previously added.",
            self.name.bright_yellow(),
            self.competition.bright_yellow()
        );
        eprintln!(
            "{} ",
            "It will cover the current data, Continue anyway? (y/N):".bright_cyan()
        );

        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                self.remove_ques()?;

                self.create_new_question(from_wsl)?;
                println!(
                    "{} {}",
                    " Overwritten question:".green(),
                    self.name.bright_white()
                );
                Ok(())
            }

            _ => {
                println!("{}", "Operation cancelled.".bright_red());
                Ok(())
            }
        }
    }

    pub fn remove_ques(self: &Self) -> Result<(), PkError> {
        let conn = Database::init_db()?;
        conn.execute(
            "DELETE FROM questions WHERE name = (?1) AND competition = (?2)",
            &[&self.name, &self.competition],
        )?;
        let dir = self.get_question_path();
        if dir.exists() {
            std::fs::remove_dir_all(dir)?;
        }
        println!("Removed question {}", self.name);
        Ok(())
    }

    pub fn list_questions(
        competition: &Option<String>,
        tags: &Option<Vec<String>>,
    ) -> Result<(), PkError> {
        let mut sql = "SELECT name, competition, tags FROM questions WHERE 1=1 ".to_string();
        let mut params: Vec<String> = Vec::new();

        if let Some(competition) = competition {
            sql.push_str(" AND competition = ? ");
            params.push(competition.clone());
        }

        if let Some(tags) = tags {
            for (i, tag) in tags.iter().enumerate() {
                if i == 0 {
                    sql.push_str(" AND (tag LIKE ?");
                } else {
                    sql.push_str(" OR tag LIKE ? ");
                }
                params.push(format!("%{}%", tag));
            }
            if !tags.is_empty() {
                sql.push(')');
            }
        }

        let conn = Database::init_db()?;
        let mut stmt = conn
            .prepare(&sql)?;
        let param_refs: Vec<&dyn ToSql> = params.iter().map(|i| i as &dyn ToSql).collect();
        let rows = stmt
            .query_map(&param_refs[..], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?;
        println!("Questions:");
        for row in rows {
            let (name, comp, tags) = row.map_err(|e| PkError::FmtError(e.to_string()))?;
            println!("- {} ({}) [{}]", name, comp, tags);
        }
        Ok(())
    }

    pub fn copy_from_windows_downloads(self: &Self) -> Result<(), PkError> {
        let downloads_path = Self::get_windows_downloads_path()?;

        let first_file = std::fs::read_dir(&downloads_path)?
            .filter_map(|entry| entry.ok())
            .find(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false));

        if let Some(file_entry) = first_file {
            let source_path = file_entry.path();
            let question_dir = self.get_question_path();
            let target_path = question_dir.join("pwn");
            std::fs::copy(&source_path, &target_path)?;

            println!(
                "{} '{}' to '{}'",
                "Copied file from Windows Downloads:".green(),
                source_path.display(),
                target_path.display()
            );
            std::fs::remove_file(&source_path)?;
        } else {
            return Err(ConfigError(
                "No files found in Downloads folder".to_string(),
            ));
        }
        Ok(())
    }

    fn get_windows_downloads_path() -> Result<PathBuf, PkError> {
        if let Some(username) = Database::get_config("windows_username")? {
            // let path = std::path::Path::new("/mnt/c/Users")
            let path = std::path::Path::new("C:/Users")
                .join("administrators")
                .join("Downloads");
            if path.exists() {
                return Ok(path);
            }
        }

        Err(ConfigError(
            "Cannot determine Windows Downloads path. Please set correct 'windows_username' in config."
                .red()
                .to_string(),
        ))
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- {} ({}) [{}]",
            self.name,
            self.competition,
            self.tags_str()
        )
    }
}
