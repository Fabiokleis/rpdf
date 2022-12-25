use clap::{arg, Command};

fn cli() -> Command {
    Command::new("rpdf")
        .about("A image to pdf converter")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("-c")
                .about("Convert image to pdf")
                .long_flag("convert")
                .arg(arg!(<PATH> "The local path of the image"))
                .arg_required_else_help(true)
        )
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("-c", sub_matches)) => {
            println!("converting {}", sub_matches.get_one::<String>("PATH").expect("required"));
        },
        _ => unreachable!(),
    }
    println!("Hello, world!");
}
