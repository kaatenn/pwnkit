use crate::data::template::Template;
use crate::error::PkError;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TemplateAction {
    List,
    Add {
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        name: Option<String>,
    },
    Remove {
        #[arg(short, long)]
        name: String,
    }
}

impl TemplateAction {
    fn execute(&self) -> Result<(), PkError> {
        match self {
            TemplateAction::List => {
                Self::list_templates()?;
                Ok(())
            }
            TemplateAction::Add { .. } => {
                todo!()
            }
            TemplateAction::Remove { .. } => {
                todo!()
            }
        }
    }

    fn list_templates() -> Result<(), PkError> {
        Template::list_templates()?;
        Ok(())
    }
}