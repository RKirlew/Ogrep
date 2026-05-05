use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ogrep")]
#[command(about = "Implementing grep in Rust")]
pub struct Args {
    #[arg(short, long)]
    pub file: String,

    #[arg(short, long)]
    pub query: String,
}
