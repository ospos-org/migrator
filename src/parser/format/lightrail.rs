use crate::parser::ParseFailure;
use chrono::prelude::*;
use csv::Reader;
use open_stock::{
    Address, ContactInformation, Customer, Email, Kiosk, MobileNumber, Note, Product, Store,
    Transaction,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, str::FromStr};

use super::{Parsable, ParseType};

pub fn match_self(parse_type: ParseType) -> String {
    let matchable = match parse_type {
        ParseType::Store => "ZZZZ",
        ParseType::Kiosk => "ZZZZ",
        ParseType::Product => "ZZZZ",
        ParseType::Customer => "ZZZZ",
        ParseType::Transaction => "ZZZZ",
    };

    String::from_str(matchable).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreRecord {
    // Empty
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KioskRecord {
    // Empty
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductRecord {
    #[serde(rename = "Handle")]
    handle: String,

    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "Body (HTML)")]
    body: String,

    #[serde(rename = "Vendor")]
    vendor: String,

    #[serde(rename = "Product Category")]
    product_category: String,

    #[serde(rename = "Type")]
    prod_type: String,

    #[serde(rename = "Tags")]
    tags: String,

    #[serde(rename = "Published")]
    published: String,

    #[serde(rename = "Option1 Name")]
    option_1_name: String,

    #[serde(rename = "Option1 Value")]
    option_1_value: String,

    #[serde(rename = "Option2 Name")]
    option_2_name: String,

    #[serde(rename = "Option2 Value")]
    option_2_value: String,

    #[serde(rename = "Option3 Name")]
    option_3_name: String,

    #[serde(rename = "Option3 Value")]
    option_3_value: String,

    #[serde(rename = "Variant SKU")]
    sku: String,

    #[serde(rename = "Variant Grams")]
    weight_grams: String,

    #[serde(rename = "Variant Inventory Tracker")]
    vit: String,

    #[serde(rename = "Variant Inventory Policy")]
    vip: String,

    #[serde(rename = "Variant Fulfillment Service")]
    vfs: String,

    #[serde(rename = "Variant Price")]
    price: String,

    #[serde(rename = "Variant Compare At Price")]
    cat: String,

    #[serde(rename = "Variant Requires Shipping")]
    requires_shipping: String,

    #[serde(rename = "Variant Taxable")]
    taxable: String,

    #[serde(rename = "Variant Barcode")]
    barcode: String,

    #[serde(rename = "Image Src")]
    image_url: String,

    #[serde(rename = "Image Position")]
    image_pos: String,

    #[serde(rename = "Image Alt Text")]
    image_alt: String,

    #[serde(rename = "Gift Card")]
    is_gift_card: String,

    #[serde(rename = "SEO Title")]
    seo_title: String,

    #[serde(rename = "Variant Image")]
    variant_image: String,

    #[serde(rename = "Variant Weight Unit")]
    weight_unit: String,

    #[serde(rename = "Variant Tax Code")]
    tax_code: String,

    #[serde(rename = "Cost per item")]
    marginal_cost: String,

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "Google Shopping / Google Product Category")]
    google_product_category: String,

    #[serde(rename = "Google Shopping / Age Group")]
    google_age_group: String,

    #[serde(rename = "Google Shopping / MPN")]
    google_mpn: String,

    #[serde(rename = "Google Shopping / AdWords Grouping")]
    google_adwords: String,

    #[serde(rename = "Google Shopping / AdWords Labels")]
    google_adword_labels: String,

    #[serde(rename = "Google Shopping / Condition")]
    google_condition: String,

    #[serde(rename = "Google Shopping / Custom Product")]
    google_custom_product: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerRecord {
    #[serde(rename = "First Name")]
    first_name: String,

    #[serde(rename = "Last Name")]
    last_name: String,

    #[serde(rename = "Email")]
    email: String,

    #[serde(rename = "Accepts Email Marketing")]
    accepts_marketing: String,

    #[serde(rename = "Company")]
    company: String,

    #[serde(rename = "Address1")]
    address_street: String,

    #[serde(rename = "Address2")]
    address_suburb: String,

    #[serde(rename = "City")]
    address_city: String,

    #[serde(rename = "Province")]
    address_province: String,

    #[serde(rename = "Province Code")]
    address_province_code: String,

    #[serde(rename = "Country")]
    address_country: String,

    #[serde(rename = "Country Code")]
    address_country_code: String,

    #[serde(rename = "Zip")]
    address_zip: String,

    #[serde(rename = "Phone")]
    phone_number: String,

    #[serde(rename = "Accepts SMS Marketing")]
    accepts_sms: String,

    #[serde(rename = "Total Spent")]
    accrued_billing: String,

    #[serde(rename = "Total Orders")]
    order_total: String,

    #[serde(rename = "Tags")]
    tags: String,

    #[serde(rename = "Note")]
    note: String,

    #[serde(rename = "Tax Exempt")]
    tax_exempt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRecord {
    #[serde(rename = "Name")]
    order_name: String,

    #[serde(rename = "Email")]
    email: String,

    #[serde(rename = "Financial Status")]
    financial_status: String,

    #[serde(rename = "Paid at")]
    paid_at: String,

    #[serde(rename = "Fulfillment Status")]
    fulfillment_status: String,

    #[serde(rename = "Fulfilled at")]
    fulfilled_at: String,

    #[serde(rename = "Accepts Marketing")]
    accepts_marketing: String,

    #[serde(rename = "Currency")]
    currency: String,

    #[serde(rename = "Subtotal")]
    subtotal: String,

    #[serde(rename = "Shipping")]
    shipping_cost: String,

    #[serde(rename = "Taxes")]
    taxes: String,

    #[serde(rename = "Total")]
    total: String,

    #[serde(rename = "Discount Code")]
    discount_code: String,

    #[serde(rename = "Discount Amount")]
    discount_amount: String,

    #[serde(rename = "Shipping Method")]
    shipping_method: String,

    #[serde(rename = "Created at")]
    created_at: String,

    #[serde(rename = "Lineitem quantity")]
    lineitem_quantity: String,

    #[serde(rename = "Lineitem name")]
    lineitem_name: String,

    #[serde(rename = "Lineitem price")]
    lineitem_price: String,

    #[serde(rename = "Lineitem compare at price")]
    lineitem_cap: String,

    #[serde(rename = "Lineitem sku")]
    lineitem_sku: String,

    #[serde(rename = "Lineitem requires shipping")]
    lineitem_requires_shipping: String,

    #[serde(rename = "Lineitem taxable")]
    lineitem_taxable: String,

    #[serde(rename = "Lineitem filfillment status")]
    lineitem_fulfillment: String,

    #[serde(rename = "Billing Name")]
    billing_name: String,

    #[serde(rename = "Billing Street")]
    billing_street: String,

    #[serde(rename = "Billing Address1")]
    billing_address: String,

    #[serde(rename = "Billing Address2")]
    billing_address_2: String,

    #[serde(rename = "Billing Company")]
    billing_company: String,

    #[serde(rename = "Billing City")]
    billing_city: String,

    #[serde(rename = "Billing Zip")]
    billing_zip: String,

    #[serde(rename = "Billing Province")]
    billing_province: String,

    #[serde(rename = "Billing Country")]
    billing_country: String,

    #[serde(rename = "Billing Phone")]
    billing_phone: String,

    #[serde(rename = "Shipping Name")]
    shipping_name: String,

    #[serde(rename = "Shipping Street")]
    shipping_street: String,

    #[serde(rename = "Shipping Address1")]
    shipping_address: String,

    #[serde(rename = "Shipping Address2")]
    shipping_address2: String,

    #[serde(rename = "Shipping Company")]
    shipping_company: String,

    #[serde(rename = "Shipping Zip")]
    shipping_zip: String,

    #[serde(rename = "Shipping Province")]
    shipping_province: String,

    #[serde(rename = "Shipping Country")]
    shipping_country: String,

    #[serde(rename = "Shipping Phone")]
    shipping_phone: String,

    #[serde(rename = "Notes")]
    notes: String,

    #[serde(rename = "Note Attributes")]
    note_attributes: String,

    #[serde(rename = "Cancelled at")]
    cancelled_at: String,

    #[serde(rename = "Payment Method")]
    payment_method: String,

    #[serde(rename = "Payment Reference")]
    payment_reference: String,

    #[serde(rename = "Refunded Amount")]
    refunded_amount: String,

    #[serde(rename = "Vendor")]
    vendor: String,

    #[serde(rename = "Outstanding Balance")]
    outstanding: String,

    #[serde(rename = "Employee")]
    employee: String,

    #[serde(rename = "Location")]
    location: String,

    #[serde(rename = "Device ID")]
    device_id: String,

    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "Tags")]
    tags: String,

    #[serde(rename = "Risk Level")]
    risk_level: String,

    #[serde(rename = "Source")]
    source: String,

    #[serde(rename = "Lineitem discount")]
    lineitem_discount: String,

    #[serde(rename = "Tax 1 Name")]
    tax_1_name: String,

    #[serde(rename = "Tax 1 Value")]
    tax_1_value: String,

    #[serde(rename = "Tax 2 Name")]
    tax_2_name: String,

    #[serde(rename = "Tax 2 Value")]
    tax_2_value: String,

    #[serde(rename = "Tax 3 Name")]
    tax_3_name: String,

    #[serde(rename = "Tax 3 Value")]
    tax_3_value: String,

    #[serde(rename = "Tax 4 Name")]
    tax_4_name: String,

    #[serde(rename = "Tax 4 Value")]
    tax_4_value: String,

    #[serde(rename = "Tax 5 Name")]
    tax_5_name: String,

    #[serde(rename = "Tax 5 Value")]
    tax_5_value: String,

    #[serde(rename = "Phone")]
    phone: String,

    #[serde(rename = "Receipt Number")]
    receipt_number: String,

    #[serde(rename = "Duties")]
    duties: String,

    #[serde(rename = "Payment Terms Name")]
    payment_terms_name: String,

    #[serde(rename = "Next Payment Due At")]
    next_payment_due_at: String,
}

pub fn parse_type<T: Parsable<R>, R: for<'de> serde::Deserialize<'de>>(
    mut reader: Reader<File>,
    db: &mut (
        Vec<Product>,
        Vec<Customer>,
        Vec<Transaction>,
        Vec<Store>,
        Vec<Kiosk>,
    ),
) -> Result<Vec<T>, ParseFailure> {
    let collected: Vec<Result<R, csv::Error>> = reader.deserialize().collect();
    let mut iterator: usize = 0;
    let mut items: Vec<T> = vec![];

    loop {
        match T::parse_individual(&collected, &mut iterator, db) {
            Ok(i) => items.push(i),
            Err(err) => match err {
                ParseFailure::EOFException => break,
                error => {
                    println!("[warn]: Parser Warning: {:?}", error);
                }
            },
        }
    }

    Ok(items)
}

impl Parsable<CustomerRecord> for Customer {
    fn parse_individual(
        reader: &[Result<CustomerRecord, csv::Error>],
        line: &mut usize,
        _db: &mut (
            Vec<Product>,
            Vec<Customer>,
            Vec<Transaction>,
            Vec<Store>,
            Vec<Kiosk>,
        ),
    ) -> Result<Customer, ParseFailure> {
        let customer: Customer = {
            let line_value = match reader.get(*line) {
                Some(value) => value,
                None => return Err(ParseFailure::EOFException),
            };

            let cloned = (*line_value).as_ref().unwrap();
            let name = format!("{} {}", cloned.first_name, cloned.last_name);

            Customer {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.clone(),
                contact: ContactInformation {
                    name,
                    mobile: MobileNumber::from(cloned.phone_number.clone()),
                    email: Email::from(cloned.email.clone()),
                    landline: cloned.phone_number.clone(),
                    address: Address {
                        street: cloned.address_street.clone(),
                        street2: cloned.address_suburb.clone(),
                        city: cloned.address_city.clone(),
                        country: cloned.address_country.clone(),
                        po_code: cloned.address_zip.clone(),
                        lat: 0.0,
                        lon: 0.0,
                    },
                },
                customer_notes: if cloned.note.is_empty() {
                    vec![]
                } else {
                    vec![Note {
                        message: cloned.note.clone(),
                        author: "SHOPIFY-IMPORT".to_string(),
                        timestamp: Utc::now(),
                    }]
                },
                balance: 0.0,
                special_pricing: if cloned.tax_exempt == "yes" {
                    "TAX-EXEMPT".to_string()
                } else {
                    "".to_string()
                },
                accepts_marketing: cloned.accepts_marketing == "yes",
            }
        };

        *line += 1;

        Ok(customer)
    }
}

impl Parsable<TransactionRecord> for Transaction {
    fn parse_individual(
        _reader: &[Result<TransactionRecord, csv::Error>],
        _line: &mut usize,
        _db: &mut (
            Vec<Product>,
            Vec<Customer>,
            Vec<Transaction>,
            Vec<Store>,
            Vec<Kiosk>,
        ),
    ) -> Result<Transaction, ParseFailure> {
        Err(ParseFailure::EOFException)
    }
}

impl Parsable<ProductRecord> for Product {
    fn parse_individual(
        _reader: &[Result<ProductRecord, csv::Error>],
        _line: &mut usize,
        _db: &mut (
            Vec<Product>,
            Vec<Customer>,
            Vec<Transaction>,
            Vec<Store>,
            Vec<Kiosk>,
        ),
    ) -> Result<Product, ParseFailure> {
        Err(ParseFailure::EOFException)
    }
}

impl Parsable<KioskRecord> for Kiosk {
    fn parse_individual(
        _reader: &[Result<KioskRecord, csv::Error>],
        _line: &mut usize,
        _db: &mut (
            Vec<Product>,
            Vec<Customer>,
            Vec<Transaction>,
            Vec<Store>,
            Vec<Kiosk>,
        ),
    ) -> Result<Self, ParseFailure>
    where
        Self: Sized,
    {
        Err(ParseFailure::EOFException)
    }
}

impl Parsable<StoreRecord> for Store {
    fn parse_individual(
        _reader: &[Result<StoreRecord, csv::Error>],
        _line: &mut usize,
        _db: &mut (
            Vec<Product>,
            Vec<Customer>,
            Vec<Transaction>,
            Vec<Store>,
            Vec<Kiosk>,
        ),
    ) -> Result<Self, ParseFailure>
    where
        Self: Sized,
    {
        Err(ParseFailure::EOFException)
    }
}
