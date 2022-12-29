extern crate clap;
use clap::{arg, Command};

#[allow(dead_code)]
use convert::{Convert, conf::Conf};

fn cli() -> Command {
    Command::new("rpdf")
        .about("A image to pdf converter.")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("-c")
                .about("Convert image to pdf")
                .long_flag("convert")
                .arg(arg!([PATH] "The local path of the images.")
                     .num_args(1..))
                .arg(arg!(<OUTPUT_PATH> "The local path of the output pdf.").last(true))
                .arg_required_else_help(true)
        )
}

#[allow(dead_code, unused_assignments)]
fn main() -> Result<(), String> {
    let mut config = Conf::default();
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("-c", sub_matches)) => {
            let imgs = sub_matches.get_many::<String>("PATH")
                .expect("required argument")
                .map(|path| path.to_string()).collect::<Vec<String>>();
            let out: String = sub_matches.get_one::<String>("OUTPUT_PATH")
                .expect("required argument")
                .to_owned();
            config = Conf::from_images(imgs, out);
        },
        _ => unreachable!(),
    }
    let cvrt = Convert::new(config);
    println!("{:#?}", cvrt);
    cvrt.save_to_pdf()?;
    Ok(())
}
