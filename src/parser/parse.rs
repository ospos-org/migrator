use crate::parser::ParseType;
use crate::{InlineDatabase};
use crate::{
    parser::format, parser::lightrail::CustomerRecord as lCR,
    parser::lightrail::KioskRecord as lKR, parser::lightrail::ProductRecord as lPR,
    parser::lightrail::StoreRecord as lSR, parser::lightrail::TransactionRecord as lTR,
    parser::shopify::CustomerRecord as sCR, parser::shopify::KioskRecord as sKR,
    parser::shopify::ProductRecord as sPR, parser::shopify::StoreRecord as sSR,
    parser::shopify::TransactionRecord as sTR,
};
use core::fmt;
use csv::Reader;
use open_stock::{Customer, Kiosk, Product, Store, Transaction};
use phf::{phf_map, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::path::PathBuf;
use std::{fs::{DirEntry, File}, fs, io::{BufRead, BufReader, Lines}, io, usize::MAX};
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

pub fn classify_type(entry: &PathBuf) -> Classification {
    let path: std::path::PathBuf = entry.clone();
    let ospos_file = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .ends_with(".os");

    if ospos_file {
        return Classification {
            score: 0,
            path: path.clone(),
            branding: "ospos".to_string(),
            variant: ParseType::Invalid,
        };
    }

    let open_file = File::open(path.clone()).unwrap();
    let reader = BufReader::new(open_file);
    let lines = reader.lines();

    classify_from_value(path, lines)
}

pub fn classify_by_path(path: &Path) -> Result<Vec<Classification>, std::io::Error> {
    println!("Traversing {}", path.to_str().unwrap_or_default());

    traverse_directories(path, &classify_type).map(|mut v| {
        v.sort_by(|a, b| (a.variant as u32).cmp(&(b.variant as u32)));
        v
    })
}

pub fn classify_from_value(path: PathBuf, mut lines: Lines<BufReader<File>>) -> Classification {
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

pub fn traverse_directories(
    dir: &Path,
    cb: &dyn Fn(&PathBuf) -> Classification,
) -> Result<Vec<Classification>, std::io::Error> {
    let mut classifications = vec![];

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                traverse_directories(&path, cb)?;
            } else {
                classifications.push(cb(&entry.path()));
            }
        }
    }

    Ok(classifications)
}