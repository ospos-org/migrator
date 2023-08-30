use odm::{convert_from_directory, InlineDatabase};

use clap::{self, Command};

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
                .about("Parse a folder of exports into the OpenRetail format")
                .arg(
                    clap::arg!(<FOLDER> "Input folder")
                        .id("folder")
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .get_matches();

    match cmd.subcommand_name() {
        Some("parse") => {
            let folder: String = cmd
                .subcommand_matches("parse")
                .expect("?")
                .get_one::<String>("folder")
                .expect("Expected value 'folder'. ")
                .to_string();

            convert_from_directory(folder.to_string());
        }
        _ => unreachable!("This shouldn't happen, please file a bug report."),
    }
}
