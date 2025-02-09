use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long, default_value_t = 1)]
    pub num: usize,
}
