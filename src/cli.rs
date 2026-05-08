use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ogrep")]
#[command(about = "Implementing grep in Rust")]
pub struct Args {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long)]
    pub query: String,

    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(short = 'w', long = "whole-word")]
    pub whole_word: bool,
    #[arg(short = 'c', long = "count")]
    pub count: bool,
}
