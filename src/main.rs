mod cli;
mod file;
mod ogrep;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    match file::read_to_string(&args.file) {
        Ok(contents) => {
            // Call the unified search function with all flags
            let results = ogrep::search(&contents, &args.query, args.ignore_case, args.whole_word);

            for line in results {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
