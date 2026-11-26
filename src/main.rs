mod frontend;
use crate::frontend::lexer::Lexer;
use clap::{CommandFactory, Parser, error::ErrorKind};
use std::fs;
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
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => match e.kind() {
            ErrorKind::MissingRequiredArgument => {
                eprintln!("ERROR: no input files given\n");
                eprintln!("{}", Args::command().render_help());
                std::process::exit(2);
            }
            _ => e.exit(),
        },
    };
    if args.zen {
        println!("Zen Of Zara");
        println!("- If dogma doesn't guide us, what will?");
        println!("- S2l0dWt1IFN1Y2tz");
        return ExitCode::SUCCESS;
    }
    if args.lex {
        for file_path in args.files {
            let ext = file_path.extension();
            if ext.is_none() || ext.unwrap() != "zara" {
                eprintln!(
                    "ERROR: file {} does not have .zara extension",
                    file_path.display()
                );
                std::process::exit(2);
            }
            let file = match fs::read_to_string(&file_path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("ERROR: failed to read file {}: {err}", file_path.display());
                    std::process::exit(2);
                }
            };
            println!(
                "===========Lexical Analysis({})==============",
                file_path.display()
            );
            let name = file_path.display().to_string();
            let lexer = Lexer::new(&name, &file);
            for token in lexer {
                println!("{:?}", token);
            }
        }
        return ExitCode::SUCCESS;
    }
    ExitCode::SUCCESS
}
