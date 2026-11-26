mod frontend;
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

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

fn main() -> ExitCode {
    let args = Args::parse();
    if args.zen {
        println!("Zen Of Zara");
        println!("- If dogma doesn't guide us, what will?");
        println!("- S2l0dWt1IFN1Y2tz");
        return ExitCode::SUCCESS;
    }
    ExitCode::SUCCESS
}
