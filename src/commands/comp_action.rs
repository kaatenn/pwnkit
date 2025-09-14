use crate::config::Database;
use crate::data::competition::Competition;
use clap::Subcommand;
use serde_json::error::Category::Data;

#[derive(Subcommand)]
pub enum CompAction {
    List,
    Add { name: String },
}

impl CompAction {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = match self {
            CompAction::List => {
                Database::list_competitions()?
            }
            CompAction::Add { name } => {
                Database::add_competitions(Competition {name: name.clone(), date: chrono::Local::now().to_rfc3339()})?
            }
        };
        Ok(())
    }
}
