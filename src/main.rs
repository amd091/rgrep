use clap::Parser;
use rgrep::search_directories;
use std::env;
/// Simple grep like CLI
#[derive(Parser, Debug)]
#[command(name = "rgrep")]
#[command(author = "Alan ")]
#[command(version = "1.0")]
#[command(about = "a grep like solution to search code through the codebase", long_about = None)]
struct Cli {
    /// The search term for the CLI
    #[arg(short, long)]
    search_term: String,

    ///The directory to search
    #[arg(short, long, default_value = ".")]
    directory: Option<String>,

    /// The type of file (txt,log,out... etc)
    #[arg(short, long, default_value = "")]
    file_type: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
    let search_term = cli.search_term;
    let directory = cli.directory.unwrap_or_else(|| current_dir);
    let file_type = cli.file_type.unwrap_or_else(|| ".*".to_string());

    search_directories(&search_term, directory, &file_type)
}
