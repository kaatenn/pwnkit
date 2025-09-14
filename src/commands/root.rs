use crate::commands::comp_action::CompAction;
use crate::commands::ques_action::QuesAction;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pk")]
#[command(about = "A powerful pwn command line tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long)]
    pub allow_wsl: bool,
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
        #[arg(short, long)]
        name: String,
    },
    #[command(name = "uninstall")]
    UninstallCommand,
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
            Commands::UninstallCommand => {
                todo!()
            }
            Commands::TemplateCommand { .. } => todo!(),
        }
        Ok(())
    }
}
