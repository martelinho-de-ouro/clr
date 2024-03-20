use serde::Deserialize;
use std::path::PathBuf;

use crate::reader;

#[derive(Deserialize)]
struct Package {
    name: String,
}

#[derive(Deserialize)]
struct Krate {
    package: Package,
    dependencies: String,
}

pub fn parse_tomls(toml_files: Vec<PathBuf>) {
    for tf in toml_files {
        println!("{}", tf.to_string_lossy());
        let toml_content =
            reader::read_content(tf).expect("Error getting file content.");
        if !toml_content.contains("workspace") {
            // https://docs.rs/toml/0.8.12/toml/#parsing-toml
            let krate: Krate = toml::from_str(&toml_content).expect("foo");
            println!("package name: {}", krate.package.name);
            println!("package name: {:?}", krate.dependencies);
        }
    }
}
