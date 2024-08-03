mod brainfuck;

use std::{fmt::{Display, Formatter}, path::PathBuf};
use clap::Parser;
use brainfuck::BrainfuckCode;

#[derive(Parser)]
#[command(version, about)]
struct BrainAssemblyCli {
    /// The BrainF*** source file
    source_file: PathBuf
}

#[derive(Debug)]
enum BrainAssemblyError {
    IoError(std::io::Error),
    StringError(String)
}

impl Display for BrainAssemblyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            BrainAssemblyError::IoError(e) => write!(f, "IO Error: {}", e),
            BrainAssemblyError::StringError(s) => write!(f, "Error: {}", s)
        }
    }
}

impl std::error::Error for BrainAssemblyError {}

impl From<std::io::Error> for BrainAssemblyError {
    fn from(e: std::io::Error) -> Self {
        BrainAssemblyError::IoError(e)
    }
}

impl From<String> for BrainAssemblyError {
    fn from(s: String) -> Self {
        BrainAssemblyError::StringError(s)
    }
}

fn main() {
    let cli = BrainAssemblyCli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: BrainAssemblyCli) -> Result<(), BrainAssemblyError> {
    let source = std::fs::read_to_string(cli.source_file)?.trim().to_string();
    let code = BrainfuckCode::new_from_code(source.as_str())?.optimize_better();
    println!("{}", code);
    Ok(())
}