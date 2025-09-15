use crate::data::question::Question;
use crate::error::PkError;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum QuesAction {
    List {
        #[arg(short, long)]
        competition: Option<String>,
        #[arg(short, long)]
        tags: Option<Vec<String>>,
    },
    Add {
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        from_wsl: bool,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        competition: String,
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    Remove {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        competition: String,
    },
}

impl QuesAction {
    pub fn execute(&self) -> Result<(), PkError> {
        match self {
            QuesAction::List { competition, tags } => {
                Self::list_questions(competition, tags)?;
                Ok(())
            }
            QuesAction::Add { from_wsl, name, competition, tags } => {
                Self::add_question(from_wsl, name, competition, tags)?;
                Ok(())
            },
            QuesAction::Remove { name, competition } => {
                Self::remove_question(name, competition)?;
                Ok(())
            }
        }
    }

    fn list_questions(
        competition: &Option<String>,
        tags: &Option<Vec<String>>,
    ) -> Result<(), PkError> {
        Question::list_questions(competition, tags)?;
        Ok(())
    }

    fn add_question(from_wsl: &bool, name: &String, competition: &String, tags: &Option<Vec<String>>) -> Result<(), PkError> {
        let question = Question::new(name.clone(), competition.clone(), tags.clone());

        question.add_question(*from_wsl)?;

        Ok(())
    }

    fn remove_question(name: &String, competition: &String) -> Result<(), PkError> {
        let question = Question::new(name.clone(), competition.clone(), None);
        question.remove_ques()?;
        Ok(())
    }
}
