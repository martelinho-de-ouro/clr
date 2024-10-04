use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Weirdo version of cat command
struct Args {
    /// file - it accepts only one file
    #[arg(value_name = "FILE", default_value = "-")]
    files: String,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}
