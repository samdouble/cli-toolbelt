use clap::Parser;
use std::{error::Error, fs::read_to_string};

mod cli;
use cli::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    read_to_string(&args.file)?
        .lines()
        .take(args.num)
        .for_each(|line| println!("{}", line));
    Ok(())
}
