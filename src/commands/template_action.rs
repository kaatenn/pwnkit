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