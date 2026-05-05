mod cli;
mod file;
mod ogrep;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    match file::read_to_string(&args.file) {
        Ok(contents) => {
            let results = ogrep::search(&contents, &args.query);
            for v in results {
                println!("{}", v);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
