use crate::database::Database;
use crate::error::PkError;
use colored::Colorize;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Question {
    pub id: Option<String>,
    pub name: String,
    pub competition: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    pub fn new(name: String, competition: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id: Some(Uuid::new_v4().to_string()),
            name,
            competition,
            tags,
        }
    }

    pub fn from_row(id: String, name: String, competition: String, tags: Option<String>) -> Self {
        let tags_vec = tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());
        Self {
            id: Some(id),
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

    pub fn add_question(self: Self) -> Result<(), PkError> {
        let conn = Database::init_db()?;
        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM questions WHERE name = (?1) AND competition = (?2)")?;
        let count: i64 = stmt.query_row(&[&self.name, &self.competition], |row| row.get(0))?;

        if count > 0 {
            self.deal_repeat_question()?;
        } else {
            self.create_new_question()?;
        }

        Ok(())
    }

    fn create_new_question(self: &Self) -> Result<(), PkError> {
        let conn = Database::init_db()?;

        let question_dir = self.get_question_path();
        let tags_str = self.tags_str();

        conn.execute(
            "INSERT OR IGNORE INTO questions (id, name, competition, tags) VALUES (?1, ?2, ?3, ?4)",
            &[
                &self.id,
                &Some(self.name.clone()),
                &Some(self.competition.clone()),
                &Some(tags_str),
            ],
        )?;

        std::fs::create_dir_all(&question_dir)?;

        println!("Added question {}", self.name);
        Ok(())
    }

    fn tags_str(self: &Self) -> String {
        let tags_str = self.tags.clone().map_or("".to_string(), |t| t.join(","));
        tags_str
    }

    fn deal_repeat_question(self: &Self) -> Result<(), PkError> {
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

                self.create_new_question()?;
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
        Ok(())
    }
}


impl Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} ({}) [{}]", self.name, self.competition, self.tags_str())
    }
}