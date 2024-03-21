use clap::Parser;

mod loader;
mod reader;
mod transformer;

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

        let json_array = transformer::tomls_two_json_array(tomls);
        for _j in json_array {
            // println!("--");
            // println!("{}", j);
            // println!("--");
        }
        print!("ok");
    } else {
        // TODO: fix this duplication - learn how to use pwd with clap
        // let tomls = loader::load_cargo_tomls(".")
        //     .expect("Error getting Cargo.toml files.");

        // let json_array = transformer::tomls_two_json_array(tomls);
        // for j in json_array {
        //     println!("{}", j);
        // }
    }
}
