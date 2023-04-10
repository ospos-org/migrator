
use core::fmt;
use std::fs::File;
use csv::Reader;
use open_stock::Product;

pub struct Products(pub Vec<Product>);

impl fmt::Display for Products {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, product| {
            result.and_then(|_| writeln!(f, "{}", product))
        })
    }
}

use super::PRODUCT_FORMATS;

pub fn read_file(reader: Reader<File>, format: String) -> Products {
    let mut products: Products = Products(vec![]);

    match PRODUCT_FORMATS.get(&format) {
        Some(executor) => {
            let result = executor(reader);

            match result {
                Ok(pdt) => products = Products(pdt),
                Err(e) => {
                    // Handle error
                    println!("[err]: Failed to parse row of input, reason: {:?}", e);
                },
            }
        },
        None => {
            panic!("No respective key exists, {}.", format)
        }
    }

    // Have products...
    products
}
