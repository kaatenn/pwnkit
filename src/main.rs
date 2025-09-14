use clap::Parser;
use pwnkit::commands::root::Cli;
use pwnkit::config::Database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    cli.command.execute()
}
