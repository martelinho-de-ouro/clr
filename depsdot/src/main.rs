use clap::Parser;

mod loader;
mod parser;
mod reader;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    dir: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // (as_deref) -> https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
    if let Some(dir) = cli.dir.as_deref() {
        let tomls = loader::load_cargo_tomls(dir)
            .expect("Error getting Cargo.toml files.");
        parser::parse_tomls(tomls);
    } else {
        println!(
            "{:?}",
            loader::load_cargo_tomls(".")
                .expect("Error getting Cargo.toml files.")
        );
    }
}
