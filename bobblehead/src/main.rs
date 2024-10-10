use anyhow::Result;
use clap::Parser;
use common::open;
use figleter::*;
use std::io::BufRead;
use std::process;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// A bobblehead version of head
struct Args {
    /// file - it accepts only one file
    #[arg(value_name = "FILE", default_value = "-")]
    file: String,
}

// the first 5 words are transformed into figlets and the next
// line is normal text. contains bugs and not 100%
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let standard_font = FIGfont::standard()?;
    match open(&args.file) {
        Ok(file) => {
            let mut count = 0;
            let mut already_figleted = vec![];
            for line in file.lines() {
                let l = line?;
                for w in l.split_whitespace() {
                    if count == 5 {
                        break;
                    }
                    if let Some(figure) = standard_font.convert(w) {
                        print!("{figure}");
                        already_figleted.push(w.to_string());
                        count += 1;
                    };
                }
                if count == 5 {
                    let remaining: String = l
                        .split_whitespace()
                        .filter(|w| !already_figleted.contains(&w.to_string()))
                        .collect::<Vec<&str>>()
                        .join(" ");
                    println!("\n{remaining}");
                    break;
                }
            }
        }
        Err(err) => eprintln!("Failed to open {0}: {err}", args.file),
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}
