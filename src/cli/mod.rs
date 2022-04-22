use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Save {
        #[clap(long)]
        title: String,

        #[clap(long, short)]
        description: Option<String>,

        #[clap(long, short, multiple_occurrences(true))]
        tags: Option<Vec<String>>,
    },
    List,
}
