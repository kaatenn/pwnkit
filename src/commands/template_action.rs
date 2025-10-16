use crate::data::template::Template;
use crate::error::PkError;
use clap::Subcommand;
use std::path::PathBuf;

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
    pub fn execute(&self) -> Result<(), PkError> {
        match self {
            TemplateAction::List => {
                Self::list_templates()?;
                Ok(())
            }
            TemplateAction::Add { path, name } => {
                Self::add_template(path, name)?;
                Ok(())
            }
            TemplateAction::Remove { name } => {
                Self::remove_template(name)?;
                Ok(())
            }
        }
    }

    fn add_template(path: &str, name: &Option<String>) -> Result<(), PkError> {
        let derived_name = match name {
            Some(n) => n.clone(),
            None => {
                let p: PathBuf = PathBuf::from(path);
                let stem = p
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("template")
                    .to_string();
                stem
            }
        };

        Template::add_templates(PathBuf::from(path), derived_name)?;
        Ok(())
    }

    fn list_templates() -> Result<(), PkError> {
        Template::list_templates()?;
        Ok(())
    }

    fn remove_template(name: &String) -> Result<(), PkError> {
        Template::remove_templates(name.clone())?;
        Ok(())
    }
}
