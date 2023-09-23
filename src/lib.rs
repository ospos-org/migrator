pub mod parser;
use std::{
    ffi::{c_char, CStr, CString},
    fs::{self, DirEntry},
    io,
    path::Path,
};

use open_stock::{Customer, Kiosk, Product, Store, Transaction};
pub use parser::*;

pub type InlineDatabase = (
    Vec<Product>,
    Vec<Customer>,
    Vec<Transaction>,
    Vec<Store>,
    Vec<Kiosk>,
);

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

#[no_mangle]
pub extern "C" fn c_convert_from_directory(input: *mut c_char) {
    let pth = unsafe { CStr::from_ptr(input) }
        .to_string_lossy()
        .into_owned();
    let path = Path::new(&pth);

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

/// 🪵 Lays the [wasm] file log into a wasmfs.
#[no_mangle]
pub extern "C" fn lay_file(
    file_id_str: *const c_char,
    file_content_str: *const c_char,
) -> *mut c_char {
    let file_id = unsafe { CStr::from_ptr(file_id_str) }
        .to_string_lossy()
        .into_owned();
    let file_content = unsafe { CStr::from_ptr(file_content_str) }
        .to_string_lossy()
        .into_owned();

    let raw_path = format!("/{}", file_id);
    let path = Path::new(raw_path.as_str());

    match fs::write(path, file_content) {
        Ok(_) => CString::new("Written File.")
            .expect("CString Conversion Failure")
            .into_raw(),
        Err(reason) => CString::new(format!("Failed to write file. Reason: {}", reason))
            .expect("CString Conversion Failure")
            .into_raw(),
    }
}

/// 🥬 Leaks the viewable [wasm] directory for debugging purposes.
#[no_mangle]
pub extern "C" fn leek_directory(dir_str: *const c_char) -> *mut c_char {
    let dir = unsafe { CStr::from_ptr(dir_str) }
        .to_string_lossy()
        .into_owned();

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

    let value: String = classifications
        .into_iter()
        .map(|classification| classification.to_string())
        .collect();

    CString::new(value)
        .expect("CString Conversion Failure")
        .into_raw()
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
