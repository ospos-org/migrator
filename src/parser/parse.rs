use std::fs::File;
use csv::Reader;
use crate::parser::format;
use open_stock::{Product, Customer, Transaction};
use phf::{Map, phf_map};

#[derive(Debug)]
pub enum ParseFailure {
    ReadFailure(String),
    FormatFailure(String),
    EOFException
}

// type Parser<T> = fn(Reader<File>) -> Result<Vec<T>, ParseFailure>;
type ProductParser = fn(Reader<File>) -> Result<Vec<Product>, ParseFailure>;
type CustomerParser = fn(Reader<File>) -> Result<Vec<Customer>, ParseFailure>;
type TransactionParser = fn(Reader<File>) -> Result<Vec<Transaction>, ParseFailure>;

pub static PRODUCT_FORMATS: phf::Map<&'static str, ProductParser> = phf_map! {
    "shopify" => format::shopify::parse_type
};

pub static CUSTOMER_FORMATS: Map<&'static str, CustomerParser> = phf_map! {
    "shopify" => format::shopify::parse_type
};

pub static TRANSACTION_FORMATS: Map<&'static str, TransactionParser> = phf_map! {
    "shopify" => format::shopify::parse_type
};