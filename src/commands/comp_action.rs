use crate::data::competition::Competition;
use crate::database::Database;
use crate::error::PkError;
use clap::Subcommand;
use colored::Colorize;
use rusqlite::Connection;
use std::io;
use std::io::Write;

#[derive(Subcommand)]
pub enum CompAction {
    List,
    Add { name: String },
}

impl CompAction {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = match self {
            CompAction::List => Self::list_competitions()?,
            CompAction::Add { name } => {
                Self::add_competitions(Competition::new(name.clone()))?;
            }
        };
        Ok(())
    }

    pub fn add_competitions(comp: Competition) -> Result<(), PkError> {
        let conn = Database::init_db()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM competitions WHERE name = (?1)")?;
        let count: i64 = stmt.query_row([&comp.name], |row| row.get(0))?;

        if count > 0 {
            Self::deal_repeat_comp(&comp, &conn)?;
        } else {
            conn.execute(
                "INSERT OR IGNORE INTO competitions (id, name, date) VALUES (?1, ?2, ?3)",
                [comp.id.as_ref(), Some(&comp.name), Some(&comp.date)],
            )?;
            println!("Added competition {}", comp.name);
        }
        Ok(())
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

    fn deal_repeat_comp(comp: &Competition, conn: &Connection) -> Result<(), PkError> {
        eprintln!("{}", "WARNING: Competition already exists!".yellow().bold());
        eprintln!(
            "   Competition '{}' was previously added.",
            comp.name.bright_yellow()
        );
        eprint!(
            "{} ",
            "It will cover the current data, Continue anyway? (y/N):".bright_cyan()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                conn.execute(
                    "UPDATE competitions SET date = (?1) WHERE name = (?2)",
                    [Some(&comp.date), Some(&comp.name)],
                )?;
                println!(
                    "{} {}",
                    "Updated competition".green(),
                    comp.name.bright_white()
                );
                todo!("添加题目删除逻辑");
                Ok(())
            }

            _ => {
                println!("{}", "Operation cancelled.".bright_red());
                Ok(())
            }
        }
    }
}
