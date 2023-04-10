use clap::{self, Arg, Command, ArgAction};
use crate::parser::read_file;

mod parser;
fn main() {
    let cmd = clap::Command::new("odm")
        .about("OpenRetail Data Format Migrator")
        .version("0.0.1")
        .bin_name("odm")
        .arg_required_else_help(true)
        .author("OpenRetail Development Team")
        .subcommand_required(true)
        .subcommand(
            Command::new("parse")
                .short_flag('p')
                .long_flag("parse")
                .about("Parse an input file into the OpenRetail format")
                .arg(
                    clap::arg!(-i --input <FILE> "Sets input file")
                        .id("input")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                )
                .arg(
                    clap::arg!(-f --format <FORMAT> "Sets input file format")
                        .id("format")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                )
        )
        .get_matches();

    match cmd.subcommand_name() {
        Some("parse") => {
            let file: String = cmd.subcommand_matches("parse").expect("?").get_one::<String>("input").expect("Expected value 'input'. ").to_string();
            let format: String = cmd.subcommand_matches("parse").expect("?").get_one::<String>("format").expect("Expected value 'format'. ").to_string();

            let rdr = csv::Reader::from_path(file).unwrap();
            let products = read_file(rdr, format);

            println!("{}", products);
        },
        _ => unreachable!("This shouldn't happen, please file a bug report."),
    }
}
