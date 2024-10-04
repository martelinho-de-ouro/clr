use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Weirdo version of cat command
struct Args {
    /// file - it accepts only one file
    #[arg(value_name = "FILE", default_value = "~/.gitconfig")]
    file: String,
}

// dyn keyword here says that the return type's trait is dynamically
// dispatched allowing to abstract the idea of input source in case
// the source implements the trait BufRead.
// The return needs to be boxed because the compiler has no idea
// about the size of of the thing that will be read at compile time.
fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

fn run(args: Args) -> Result<()> {
    match open(&args.file) {
        Err(err) => eprintln!("Failed to open {0}: {err}", args.file),
        Ok(file) => {
            for line in file.lines() {
                let l = line?;
                println!("☺️ {l}");
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}
