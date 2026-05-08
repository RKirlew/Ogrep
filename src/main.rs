mod cli;
mod file;
mod ogrep;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    match file::read_to_string(&args.file) {
        Ok(contents) => {
            let results = ogrep::search(&contents, &args.query, args.ignore_case, args.whole_word);
            if args.count {
                println!("{}", results.len());
            } else {
                for line in results {
                    println!("{}", line);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
