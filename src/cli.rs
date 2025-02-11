use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(Add),
}

#[derive(Args)]
pub struct Add {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long, default_value_t = 1)]
    pub num: usize,
}
