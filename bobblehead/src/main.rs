use anyhow::Result;
use clap::Parser;
use common::open;
use figleter::FIGfont;
use std::io::BufRead;
use std::process;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// A bobblehead version of head
struct Args {
    /// file - it accepts only one file
    #[arg(value_name = "FILE", default_value = "-")]
    file: String,
    /// number of lines
    #[arg(
        short('n'),
        long,
        value_name = "LINES",
        default_value = "10",
        value_parser = clap::value_parser!(u32).range(1..)
     )]
    lines: u32,
    /// number of bytes
    #[arg(
        short('c'),
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u32).range(1..)
    )]
    bytes: Option<u32>,
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let standard_font = FIGfont::standard()?;
    match open(&args.file) {
        Err(err) => eprintln!("Failed to open {0}: {err}", args.file),
        Ok(file) => {
            for line in file.lines() {
                let l = line?;
                for w in l.split_whitespace() {
                    let figure = standard_font.convert(w);
                    match figure {
                        Some(f) => print!("{f}"),
                        None => println!("So trash bug"),
                    }
                }
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
