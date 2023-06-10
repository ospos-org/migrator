pub mod lightrail;
pub mod shopify;

use open_stock::{Customer, Product, Transaction};

pub use lightrail::*;
pub use shopify::*;

use crate::parser::ParseFailure;

use strum_macros::{Display, EnumIter};

#[derive(Debug, EnumIter, Copy, Clone, Display)]
pub enum ParseType {
    Product = 0,
    Customer = 1,
    Transaction = 2,
}

pub trait Parsable<R> {
    fn parse_individual(
        reader: &Vec<Result<R, csv::Error>>,
        line: &mut usize,
        db: &mut (Vec<Product>, Vec<Customer>, Vec<Transaction>),
    ) -> Result<Self, ParseFailure>
    where
        Self: Sized;
}
