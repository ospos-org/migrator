use crate::InlineDatabase;
use crate::{
    parser::format, parser::lightrail::CustomerRecord as lCR,
    parser::lightrail::KioskRecord as lKR, parser::lightrail::ProductRecord as lPR,
    parser::lightrail::StoreRecord as lSR, parser::lightrail::TransactionRecord as lTR,
    parser::shopify::CustomerRecord as sCR, parser::shopify::KioskRecord as sKR,
    parser::shopify::ProductRecord as sPR, parser::shopify::StoreRecord as sSR,
    parser::shopify::TransactionRecord as sTR,
};

use crate::parser::ParseType;

use core::fmt;
use csv::Reader;
use open_stock::{Customer, Kiosk, Product, Store, Transaction};
use phf::{phf_map, Map};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::{
    fs::{DirEntry, File},
    io::{BufRead, BufReader},
    usize::MAX,
};
use strsim::levenshtein;

use strum::IntoEnumIterator;

#[derive(Debug)]
pub enum ParseFailure {
    ReadFailure(String),
    FormatFailure(String),
    EOFException,
}

// type Parser<T> = fn(Reader<File>) -> Result<Vec<T>, ParseFailure>;
type KioskParser = fn(Reader<File>, &mut InlineDatabase) -> Result<Vec<Kiosk>, ParseFailure>;
type StoreParser = fn(Reader<File>, &mut InlineDatabase) -> Result<Vec<Store>, ParseFailure>;
type ProductParser = fn(Reader<File>, &mut InlineDatabase) -> Result<Vec<Product>, ParseFailure>;
type CustomerParser = fn(Reader<File>, &mut InlineDatabase) -> Result<Vec<Customer>, ParseFailure>;
type TransactionParser =
    fn(Reader<File>, &mut InlineDatabase) -> Result<Vec<Transaction>, ParseFailure>;

pub static KIOSK_FORMATS: Map<&'static str, KioskParser> = phf_map! {
    "shopify" => format::shopify::parse_type::<open_stock::Kiosk, sKR>,
    "lightrail" => format::lightrail::parse_type::<open_stock::Kiosk, lKR>
};

pub static STORE_FORMATS: Map<&'static str, StoreParser> = phf_map! {
    "shopify" => format::shopify::parse_type::<open_stock::Store, sSR>,
    "lightrail" => format::lightrail::parse_type::<open_stock::Store, lSR>
};

pub static PRODUCT_FORMATS: phf::Map<&'static str, ProductParser> = phf_map! {
    "shopify" => format::shopify::parse_type::<open_stock::Product, sPR>,
    "lightrail" => format::lightrail::parse_type::<open_stock::Product, lPR>
};

pub static CUSTOMER_FORMATS: Map<&'static str, CustomerParser> = phf_map! {
    "shopify" => format::shopify::parse_type::<open_stock::Customer, sCR>,
    "lightrail" => format::lightrail::parse_type::<open_stock::Customer, lCR>,
};

pub static TRANSACTION_FORMATS: Map<&'static str, TransactionParser> = phf_map! {
    "shopify" => format::shopify::parse_type::<open_stock::Transaction, sTR>,
    "lightrail" => format::lightrail::parse_type::<open_stock::Transaction, lTR>
};

pub static HEADER_FORMAT_MATCHERS: phf::Map<&'static str, fn(ParseType) -> String> = phf_map! {
    "shopify" => format::shopify::match_self,
    "lightrail" => format::lightrail::match_self
};

#[derive(Debug)]
pub struct Classification {
    pub score: usize,
    pub path: PathBuf,
    pub branding: String,
    pub variant: ParseType,
}

impl Display for Classification {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]\t{} ({:08}) - {} ",
            self.variant,
            self.branding,
            self.score,
            self.path.file_name().unwrap().to_str().unwrap(),
        )
    }
}

pub fn classify_type(entry: &DirEntry) -> Classification {
    let path: std::path::PathBuf = entry.path();

    let open_file = File::open(path.clone()).unwrap();
    let reader = BufReader::new(open_file);
    let mut lines = reader.lines();

    let mut best_match = Classification {
        score: MAX,
        path: path.clone(),
        branding: "none".to_string(),
        variant: ParseType::Product,
    };

    if let Some(Ok(line)) = lines.next() {
        for (key, val) in HEADER_FORMAT_MATCHERS.into_iter() {
            for variant in ParseType::iter() {
                let comparative = val(variant);
                let score = levenshtein(line.as_str(), comparative.as_str());

                if score < best_match.score {
                    best_match = Classification {
                        branding: key.to_string(),
                        score,
                        path: path.clone(),
                        variant,
                    }
                }
            }
        }
    }

    best_match
}
