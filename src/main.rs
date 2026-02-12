mod cli;
mod engine;
mod parsers;
mod reporters;
mod rules;
mod scanner;
mod tokens;
mod types;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { ref path, .. } => {
            let files = scanner::scan_files(path);
            eprintln!("Found {} file(s) to scan", files.len());
            for f in &files {
                eprintln!("  {}", f.display());
            }
        }
    }
}
