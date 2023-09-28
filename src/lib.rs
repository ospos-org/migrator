pub mod parser;
use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};
use std::fs::{File, OpenOptions};
use std::io::Write;

use open_stock::{Customer, Kiosk, Product, Store, Transaction};
pub use parser::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

pub type InlineDatabase = (
    Vec<Product>,
    Vec<Customer>,
    Vec<Transaction>,
    Vec<Store>,
    Vec<Kiosk>,
);

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn convert_from_directory(input: String) {
    let path = Path::new(&input);

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

    let mut db: InlineDatabase = (vec![], vec![], vec![], vec![], vec![]);

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

#[cfg(not(feature = "wasm"))]
pub fn convert_from_directory(input: String) -> String {
    let path = Path::new(&input);
    println!("Traversing {}", path.clone().to_str().unwrap_or_default());

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

    println!("Yielded Following Classifications: {:?}", classifications);

    let mut db: InlineDatabase = (vec![], vec![], vec![], vec![], vec![]);

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
            let to_write_path = path.join("output.os");

            match File::create(to_write_path.clone()) {
                Ok(mut f) => {
                    match f.write_all(string_value.as_bytes()) {
                        Ok(_) =>{
                            if let Err(err) = f.flush() {
                                println!("Encountered error flushing file, {}", err);
                            } else {
                                println!(
                                    "Wrote some bytes to {} and Converted all data. Thank you for using OpenPOS!",
                                    to_write_path.to_str().unwrap(),
                                )
                            }
                        },
                        Err(error) => println!(
                            "Failed to write data to file. Path given was: {} {:?}",
                            to_write_path.to_str().unwrap_or_default(), error
                        )
                    }
                }
                Err(error) => {
                    println!(
                        "Failed to create data file. Path given was: {} {:?}",
                        to_write_path.to_str().unwrap_or_default(), error
                    )
                }
            };

            string_value
        }
        Err(error) => {
            println!("Failed to stringify data, {:?}", error);

            format!("Failed Convert.")
        }
    }
}

/// 🪵 Lays the [wasm] file log into a wasmfs.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn lay_file(file_id: String, file_content: String) -> String {
    let raw_path = format!("/{}", file_id);
    let path = Path::new(raw_path.as_str());

    match fs::write(path, file_content) {
        Ok(_) => "Written File.".to_string(),
        Err(reason) => format!("Failed to write file. Reason: {}", reason),
    }
}

/// 🥬 Leaks the viewable [wasm] directory for debugging purposes.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn leek_directory(dir: String) -> String {
    let path = Path::new(dir.as_str());

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

    classifications
        .into_iter()
        .map(|classification| classification.to_string())
        .collect()
}

pub fn traverse_directories(
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
