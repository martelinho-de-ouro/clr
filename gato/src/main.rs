use anyhow::Result;
use clap::Parser;
use common::open;
use std::io::BufRead;
use std::process;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Weirdo version of cat command
struct Args {
    /// file - it accepts only one file
    #[arg(value_name = "FILE", default_value = "-")]
    file: String,
}

fn cat_being_weird(line: &str) -> String {
    let mut new_line = String::new();
    for w in line.split_whitespace() {
        new_line.push(' ');
        if let Some(emoji) = emojis::get_by_shortcode(w) {
            new_line.push_str(emoji.as_str());
        } else {
            new_line.push_str(w);
        }
    }

    new_line.to_string()
}

fn run(args: Args) -> Result<()> {
    match open(&args.file) {
        Err(err) => eprintln!("Failed to open {0}: {err}", args.file),
        Ok(file) => {
            for line in file.lines() {
                let l = line?;
                let new_line = cat_being_weird(&l);
                println!("{new_line}");
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
