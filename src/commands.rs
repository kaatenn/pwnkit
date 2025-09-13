use clap::{Parser, Subcommand};

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
        #[arg(short, long)]
        list: bool,
        #[arg(short, long)]
        name: String,
    },
    #[command(name = "q")]
    QuesCommand {
        #[arg(short, long)]
        list: bool,
        #[arg(short, long)]
        competition: String,
        #[arg(short, long)]
        name: String,
    },
    #[command(name = "uninstall")]
    UninstallCommand,
}