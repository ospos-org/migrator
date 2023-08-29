#![allow(ambiguous_glob_reexports)]

pub mod lightrail;
pub mod shopify;

use open_stock::{Customer, Kiosk, Product, Store, Transaction};

pub use lightrail::*;
pub use shopify::*;

use crate::parser::ParseFailure;

use strum_macros::{Display, EnumIter};

/// **Linking Hierarchy**
///
/// A store must be present for a product (stock information)
/// For a transaction to take place, there must be customers
/// to link to, hence the following hierarchy.
///
#[derive(Debug, EnumIter, Copy, Clone, Display)]
pub enum ParseType {
    Store = 0,
    Kiosk = 1,
    Product = 2,
    Customer = 3,
    Transaction = 4,
}

pub trait Parsable<R> {
    fn parse_individual(
        reader: &[Result<R, csv::Error>],
        line: &mut usize,
        db: &mut (
            Vec<Product>,
            Vec<Customer>,
            Vec<Transaction>,
            Vec<Store>,
            Vec<Kiosk>,
        ),
    ) -> Result<Self, ParseFailure>
    where
        Self: Sized;
}
