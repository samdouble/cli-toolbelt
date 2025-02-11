use clap::Parser;
use dirs;
use std::{error::Error, fs::read_to_string};

mod cli;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match &args.command {
        Commands::Add(args) => {
            println!("{}", args.file);
            println!("{}", args.num);

            println!("{}", dirs::home_dir().unwrap().display().to_string());
            println!("{}", dirs::config_dir().unwrap().display().to_string());

            read_to_string(&args.file)?
                .lines()
                .take(args.num)
                .for_each(|line| println!("{}", line));
            Ok(())
        }
    }
}
