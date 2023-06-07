use core::fmt;
use csv::Reader;
use open_stock::{Customer, Product, Transaction};
use std::fs::File;

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

use crate::parser::ParseType;

use super::{CUSTOMER_FORMATS, PRODUCT_FORMATS, TRANSACTION_FORMATS};

pub fn read_file(
    reader: Reader<File>,
    format: String,
    file_type: ParseType,
) -> (Products, Customers, Transactions) {
    let mut products: Products = Products(vec![]);
    let mut customers: Customers = Customers(vec![]);
    let mut transactions: Transactions = Transactions(vec![]);

    let db = (
        products.0.as_ref(),
        customers.0.as_ref(),
        transactions.0.as_ref(),
    );

    if format == "none" {
        return (products, customers, transactions);
    }

    match file_type {
        ParseType::Product => {
            match PRODUCT_FORMATS.get(&format) {
                Some(executor) => {
                    let result = executor(reader, db);

                    match result {
                        Ok(pdt) => products = Products(pdt),
                        Err(e) => {
                            // Handle error
                            println!("[err]: Failed to parse row of input, reason: {:?}", e);
                        }
                    }
                }
                None => {
                    panic!("No respective key exists, {}.", format)
                }
            }
        }
        ParseType::Customer => {
            match CUSTOMER_FORMATS.get(&format) {
                Some(executor) => {
                    let result = executor(reader, db);

                    match result {
                        Ok(custom) => customers = Customers(custom),
                        Err(e) => {
                            // Handle error
                            println!("[err]: Failed to parse row of input, reason: {:?}", e);
                        }
                    }
                }
                None => {
                    panic!("No respective key exists, {}.", format)
                }
            }
        }
        ParseType::Transaction => {
            match TRANSACTION_FORMATS.get(&format) {
                Some(executor) => {
                    let result = executor(reader, db);

                    match result {
                        Ok(trans) => transactions = Transactions(trans),
                        Err(e) => {
                            // Handle error
                            println!("[err]: Failed to parse row of input, reason: {:?}", e);
                        }
                    }
                }
                None => {
                    panic!("No respective key exists, {}.", format)
                }
            }
        }
    }

    (products, customers, transactions)
}
