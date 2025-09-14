use clap::Parser;
use pwnkit::commands::{Cli, Commands};
use pwnkit::config::Config;
use pwnkit::data::competition::Competition;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::load();

    match cli.command {
        Commands::CompCommand { list, name} => {
            if list {
                println!("Competition list:");
                for comp in &config.competitions {
                    println!("- {}", comp);
                }
            }
            else if let Some(name) = name {
                config.add_competitions(Competition::new(name));
                config.save()?;
            };
            Ok(())
        }
        Commands::QuesCommand { .. } => {
            todo!()
        }
        Commands::UninstallCommand => {
            todo!()
        },
        Commands::TemplateCommand { .. } => todo!()
    }
}