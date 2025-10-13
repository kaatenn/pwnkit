use crate::data::competition::Competition;
use crate::error::PkError;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum CompAction {
    List,
    Add {
        #[arg(short, long)]
        name: String,
    },
    Remove {
        #[arg(short, long)]
        name: String,
    }
}

impl CompAction {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = match self {
            CompAction::List => Self::list_competitions()?,
            CompAction::Add { name } => {
                Self::add_competitions(Competition::new(name.clone()))?;
            },
            CompAction::Remove { name } => {
                Self::remove_competitions(name.clone())?;
            }
        };
        Ok(())
    }

    fn add_competitions(comp: Competition) -> Result<(), PkError> {
        comp.add_competitions()?;

        Ok(())
    }

    fn list_competitions() -> Result<(), PkError> {
        Competition::list_competitions()?;
        Ok(())
    }

    fn remove_competitions(name: String) -> Result<(), PkError> {
        let comp = Competition::new(name.clone());
        comp.remove_competition()?;
        println!("Removed competition {}", name);
        Ok(())
    }
}
