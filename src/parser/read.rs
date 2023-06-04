
use core::fmt;
use std::fs::File;
use csv::Reader;
use open_stock::{Product, Customer, Transaction};

pub struct Products(pub Vec<Product>);

impl fmt::Display for Products {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, product| {
            result.and_then(|_| writeln!(f, "{}", product))
        })
    }
}

pub struct Customers(pub Vec<Customer>);

impl fmt::Display for Customers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, customer| {
            result.and_then(|_| writeln!(f, "{}", customer))
        })
    }
}

pub struct Transactions(pub Vec<Transaction>);

impl fmt::Display for Transactions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, transaction| {
            result.and_then(|_| writeln!(f, "{}", transaction))
        })
    }
}

use super::{PRODUCT_FORMATS, CUSTOMER_FORMATS, TRANSACTION_FORMATS};

pub fn read_file(reader: Reader<File>, format: String, file_type: String) -> (Products, Customers, Transactions) {
    let mut products: Products = Products(vec![]);
    let mut customers: Customers = Customers(vec![]);
    let mut transactions: Transactions = Transactions(vec![]);

    match file_type.as_str() {
        "product" | "products" => {
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
        },
        "customer" | "customers" => {
            match CUSTOMER_FORMATS.get(&format) {
                Some(executor) => {
                    let result = executor(reader);
        
                    match result {
                        Ok(custom) => customers = Customers(custom),
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
        },
        "order" | "orders" | "transaction" | "transactions" => {
            match TRANSACTION_FORMATS.get(&format) {
                Some(executor) => {
                    let result = executor(reader);
        
                    match result {
                        Ok(trans) => transactions = Transactions(trans),
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
        },
        _ => {
            println!("File type \"{}\" unknown. Valid types are: product, customer and order.", file_type)
        }
    }

    (products, customers, transactions)
}
