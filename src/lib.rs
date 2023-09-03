pub mod parser;
use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

use vfs::MemoryFS;
use wasm_bindgen::prelude::*;

use open_stock::{Customer, Kiosk, Product, Store, Transaction};
pub use parser::*;

pub type InlineDatabase = (
    Vec<Product>,
    Vec<Customer>,
    Vec<Transaction>,
    Vec<Store>,
    Vec<Kiosk>,
);

#[wasm_bindgen]
pub fn convert_from_directory(folder: String) {
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

#[wasm_bindgen]
/// 🪵 Lays the [wasm] file log into a wasmfs.
pub fn lay_file(file_id: String, file_content: String) -> String {
    let raw_path = format!("/{}", file_id);
    let path = Path::new(raw_path.as_str());

    match fs::write(path, file_content) {
        Ok(_) => format!("Written File."),
        Err(reason) => {
            format!("Failed to write file. Reason: {}", reason.to_string())
        }
    }
}

#[wasm_bindgen]
/// 🥬 Leaks the viewable [wasm] directory for debugging purposes.
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
