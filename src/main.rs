use std::{
    fs::{self, DirEntry},
    io::{self},
    path::Path,
};

use crate::parser::classify_type;
use crate::parser::read_file;

use crate::parser::Classification;
use clap::{self, Command};
use open_stock::{Customer, Kiosk, Product, Store, Transaction};

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

            let path = Path::new(folder.as_str());
            let classifications = match traverse_directories(path, &classify_type) {
                Ok(mut v) => {
                    v.sort_by(|a, b| (a.variant as u32).cmp(&(b.variant as u32)));
                    v
                }
                Err(err) => {
                    panic!(
                        "[err]: Execution error in parsing files in provided directory, {}",
                        err
                    );
                }
            };

            let mut db: (
                Vec<Product>,
                Vec<Customer>,
                Vec<Transaction>,
                Vec<Store>,
                Vec<Kiosk>,
            ) = (vec![], vec![], vec![], vec![], vec![]);

            for c in classifications {
                println!("{}", c);

                match csv::Reader::from_path(c.path) {
                    Ok(rdr) => {
                        read_file(rdr, c.branding, c.variant, &mut db);
                    }
                    Err(error) => {
                        println!("{:?}", error)
                    }
                }
            }

            match serde_json::to_string(&db) {
                Ok(string_value) => {
                    // We're all good!
                    match fs::write("output.os", string_value) {
                        Ok(_) => {
                            println!("Converted all data. Thank you for using OpenPOS!")
                        }
                        Err(error) => {
                            println!("Failed to save data to file, {:?}", error)
                        }
                    }
                }
                Err(error) => {
                    println!("Failed to stringify data, {:?}", error)
                }
            }
        }
        _ => unreachable!("This shouldn't happen, please file a bug report."),
    }
}

fn traverse_directories(
    dir: &Path,
    cb: &dyn Fn(&DirEntry) -> Classification,
) -> Result<Vec<Classification>, io::Error> {
    let mut classifications = vec![];

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                traverse_directories(&path, cb)?;
            } else {
                classifications.push(cb(&entry))
            }
        }
    }

    Ok(classifications)
}
