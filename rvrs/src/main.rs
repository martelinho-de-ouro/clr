use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("rvrs")
        .version("0.0.1")
        .author("Helio Frota")
        .about("Reverse echo")
        .arg(
            Arg::new("text")
                .action(ArgAction::Append)
                .value_name("TEXT")
                .help("Input text")
                .required(true),
        )
        .arg(
            Arg::new("no_newline")
                .short('n')
                .help("Do not print newline"),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    println!("{:?}", &text.join(" "));
}
