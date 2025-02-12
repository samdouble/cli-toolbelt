use clap::Parser;
use std::{error::Error, fs::read_to_string};

mod cli;
use cli::{Cli, Commands};
mod config;
use config::read_toml;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match &args.command {
        Commands::Add(args) => {
            println!("{}", args.file);
            println!("{}", args.num);

            read_toml("example.toml");

            read_to_string(&args.file)?
                .lines()
                .take(args.num)
                .for_each(|line| println!("{}", line));
            Ok(())
        }
    }
}
