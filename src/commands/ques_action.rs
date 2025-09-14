use clap::Subcommand;

#[derive(Subcommand)]
pub enum QuesAction {
    List,
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        competition: String,
        #[arg(short, long)]
        tags: Option<Vec<String>>,
    }
}