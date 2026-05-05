use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(name = "ogrep")]
#[command(about = "Implementing grep in Rust")]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    query: String,
}

fn open_file(file_to_open: String) -> std::io::Result<String> {
    let mut file = File::open(file_to_open)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() {
    let args = Args::parse();
    match open_file(args.file) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}
