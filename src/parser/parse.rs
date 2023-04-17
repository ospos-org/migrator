use std::fs::File;
use csv::Reader;
use crate::parser::format;
use open_stock::Product;
use phf::{Map, phf_map};

#[derive(Debug)]
pub enum ParseFailure {
    ReadFailure(String),
    FormatFailure(String)
}

type ProductParser = fn(Reader<File>, String) -> Result<Vec<Product>, ParseFailure>;

pub static PRODUCT_FORMATS: Map<&'static str, ProductParser> = phf_map! {
    "shopify" => format::shopify::parse_product,
};