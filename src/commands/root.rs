use crate::commands::comp_action::CompAction;
use crate::commands::ques_action::QuesAction;
use crate::commands::template_action::TemplateAction;
use clap::{Parser, Subcommand};
use crate::data::competition::Competition;
use crate::error::PkError;
use crate::config;

#[derive(Parser)]
#[command(name = "pk")]
#[command(about = "A powerful pwn command line tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "c")]
    CompCommand {
        #[command(subcommand)]
        action: CompAction,
    },
    #[command(name = "q")]
    QuesCommand {
        #[command(subcommand)]
        action: QuesAction,
    },
    #[command(name = "t")]
    TemplateCommand {
        #[command(subcommand)]
        action: TemplateAction,
    },
    #[command(name = "clear")]
    ClearCommand,
}

impl Commands {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Commands::CompCommand {action} => {
                action.execute()?
            }
            Commands::QuesCommand { action } => {
                action.execute()?
            }
            Commands::ClearCommand => {
                Self::clear()?
            }
            Commands::TemplateCommand { action} => {
                action.execute()?
            },
        }
        Ok(())
    }

    fn clear() -> Result<(), PkError> {
        println!("Do you want to clear all user data?(Y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                // Competition::remove_all()?;
                let data_root = config::root();
                std::fs::remove_dir_all(&data_root)?;
            }
            _ => {}
        }
        Ok(())
    }
}
