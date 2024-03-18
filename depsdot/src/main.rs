use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    dir: Option<String>,
}

/// This function gets all the Cargo.toml files from a directory, and returns a
/// vector with all the Cargo.toml's file paths.
pub fn get_cargo_tomls(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    // Creates an empty vector.
    let mut cargo_tomls = Vec::new();
    // Gets all the entries from 'current|argument' dir.
    let entries = fs::read_dir(dir)?;

    for e in entries {
        let entry = e?;
        let path = entry.path();

        if path.is_dir() {
            // Ignoring nonsense directories.
            if let Some(name) = path.file_name() {
                if name == "target"
                    || name == ".git"
                    || name == ".github"
                    || name == "tests"
                {
                    continue;
                }
            }

            // take a look at sub dir
            let temp = get_cargo_tomls(&path.to_string_lossy())?;
            // Cargo.toml found? Then add to the vector.
            cargo_tomls.extend(temp);
        } else if let Some(file_name) = path.file_name() {
            if file_name == "Cargo.toml" {
                cargo_tomls.push(path);
            }
        }
    }

    Ok(cargo_tomls)
}

fn main() {
    let cli = Cli::parse();

    if let Some(dir) = cli.dir.as_deref() {
        println!("{:?}", get_cargo_tomls(dir).unwrap());
    } else {
        println!("{:?}", get_cargo_tomls(".").unwrap());
    }
}
