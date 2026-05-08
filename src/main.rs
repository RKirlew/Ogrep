mod cli;
mod file;
mod ogrep;
use clap::Parser;
use cli::Args;
use colored::Colorize;
fn main() {
    let args = Args::parse();

    match file::read_to_string(&args.file) {
        Ok(contents) => {
            let results = ogrep::search(&contents, &args.query, args.ignore_case, args.whole_word);
            let scoff = "Count:";
            if args.count {
                println!("{} {}", scoff.red(), results.len());
            } else {
                for line in results {
                    println!("{}", line.green().bold());
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
