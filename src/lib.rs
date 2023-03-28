use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Arguments {
    pub input: String,
    pub target_format: String,
}