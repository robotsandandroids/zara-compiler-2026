mod frontend;
use clap::Parser;
use std::path::PathBuf;

/// Zara Compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Files to compile that end with .zara
    #[arg(required_unless_present = "zen")]
    files: Vec<PathBuf>,
    /// Lexically analyze a file
    #[arg(short, long)]
    lex: bool,
    /// Print the zen of zara then exit
    #[arg(short, long)]
    zen: bool,
}

fn main() {
    let args = Args::parse();
}
