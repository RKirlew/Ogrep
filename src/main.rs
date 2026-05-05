mod cli;
mod file;
mod ogrep;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    match file::read_to_string(&args.file) {
        Ok(contents) => {
            if args.ignore_case {
                let results = ogrep::search_ignore_case(&contents, &args.query); //Ignore case hello=Hello
                for v in results {
                    println!("{}", v);
                }
            } else {
                let results = ogrep::search_match_case(&contents, &args.query); // Base case Hello==Hello
                for v in results {
                    println!("{}", v);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
