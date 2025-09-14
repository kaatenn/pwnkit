use crate::data::competition::Competition;
use crate::database::Database;
use crate::error::PkError;
use clap::Subcommand;

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

    fn add_competitions(comp: Competition) -> Result<(), PkError> {
        comp.add_competitions()?;
        Ok(())
    }

    fn list_competitions() -> Result<(), Box<dyn std::error::Error>> {
        let conn = Database::init_db()?;
        let mut stmt = conn.prepare("SELECT name FROM competitions ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;

        println!("Competition list:");
        for row in rows {
            println!("- {}", row?);
        }
        Ok(())
    }
}
