use clap::Command;

fn main() {
    let _matches = Command::new("foo")
        .version("0.1.0")
        .author("helio frota")
        .about("the description")
        .get_matches(); // tells the 'Command' to parse the arguments.

    println!("Hello, world!");
}
