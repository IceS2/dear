use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Save {
        #[clap(long, short)]
        title: String,
        #[clap(long, short)]
        description: Option<String>
    },
    List
}
