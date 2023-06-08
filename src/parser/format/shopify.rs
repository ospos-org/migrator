use crate::parser::ParseFailure;
use chrono::prelude::*;
use csv::Reader;
use open_stock::{
    Address, ContactInformation, Customer, DiscountValue, Email, MobileNumber, Note, Product,
    StockInformation, Transaction, Variant, VariantCategory, VariantInformation,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, ops::Deref, str::FromStr};

use super::{Parsable, ParseType};

pub fn match_self(parse_type: ParseType) -> String {
    let matchable = match parse_type {
        ParseType::Product => "Handle,Title,Body (HTML),Vendor,Product Category,Type,Tags,Published,Option1 Name,Option1 Value,Option2 Name,Option2 Value,Option3 Name,Option3 Value,Variant SKU,Variant Grams,Variant Inventory Tracker,Variant Inventory Qty,Variant Inventory Policy,Variant Fulfillment Service,Variant Price,Variant Compare At Price,Variant Requires Shipping,Variant Taxable,Variant Barcode,Image Src,Image Position,Image Alt Text,Gift Card,SEO Title,SEO Description,Google Shopping / Google Product Category,Google Shopping / Gender,Google Shopping / Age Group,Google Shopping / MPN,Google Shopping / AdWords Grouping,Google Shopping / AdWords Labels,Google Shopping / Condition,Google Shopping / Custom Product,Google Shopping / Custom Label 0,Google Shopping / Custom Label 1,Google Shopping / Custom Label 2,Google Shopping / Custom Label 3,Google Shopping / Custom Label 4,Variant Image,Variant Weight Unit,Variant Tax Code,Cost per item,Included / New Zealand,Included / International,Price / International,Compare At Price / International,Status",
        ParseType::Customer => "First Name,Last Name,Email,Accepts Email Marketing,Company,Address1,Address2,City,Province,Province Code,Country,Country Code,Zip,Phone,Accepts SMS Marketing,Total Spent,Total Orders,Tags,Note,Tax Exempt",
        ParseType::Transaction => "Name,Email,Financial Status,Paid at,Fulfillment Status,Fulfilled at,Accepts Marketing,Currency,Subtotal,Shipping,Taxes,Total,Discount Code,Discount Amount,Shipping Method,Created at,Lineitem quantity,Lineitem name,Lineitem price,Lineitem compare at price,Lineitem sku,Lineitem requires shipping,Lineitem taxable,Lineitem fulfillment status,Billing Name,Billing Street,Billing Address1,Billing Address2,Billing Company,Billing City,Billing Zip,Billing Province,Billing Country,Billing Phone,Shipping Name,Shipping Street,Shipping Address1,Shipping Address2,Shipping Company,Shipping City,Shipping Zip,Shipping Province,Shipping Country,Shipping Phone,Notes,Note Attributes,Cancelled at,Payment Method,Payment Reference,Refunded Amount,Vendor,Outstanding Balance,Employee,Location,Device ID,Id,Tags,Risk Level,Source,Lineitem discount,Tax 1 Name,Tax 1 Value,Tax 2 Name,Tax 2 Value,Tax 3 Name,Tax 3 Value,Tax 4 Name,Tax 4 Value,Tax 5 Name,Tax 5 Value,Phone,Receipt Number,Duties,Billing Province Name,Shipping Province Name,Payment ID,Payment Terms Name,Next Payment Due At,Payment References"
    };

    String::from_str(matchable).unwrap()
}

#[derive(Debug, Clone)]
struct Options {
    option_1_name: String,
    option_2_name: String,
    option_3_name: String,
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
    db: &(Vec<Product>, Vec<Customer>, Vec<Transaction>),
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
        reader: &Vec<Result<CustomerRecord, csv::Error>>,
        line: &mut usize,
        _db: &(Vec<Product>, Vec<Customer>, Vec<Transaction>),
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
        reader: &Vec<Result<TransactionRecord, csv::Error>>,
        line: &mut usize,
        _db: &(Vec<Product>, Vec<Customer>, Vec<Transaction>),
    ) -> Result<Transaction, ParseFailure> {
        Err(ParseFailure::EOFException)
    }
}

impl Parsable<ProductRecord> for Product {
    fn parse_individual(
        reader: &Vec<Result<ProductRecord, csv::Error>>,
        line: &mut usize,
        _db: &(Vec<Product>, Vec<Customer>, Vec<Transaction>),
    ) -> Result<Product, ParseFailure> {
        let init_line = line.clone();
        let mut options: Option<Options> = None;

        // Shopify will not provide any information like this,
        // so we must freshly generate it.
        let generated_sku = uuid::Uuid::new_v4().to_string();
        let pdt_ident = open_stock::ProductIdentification {
            sku: generated_sku.clone(),
            ean: String::new(),
            hs_code: String::new(),
            article_code: String::new(),
            isbn: String::new(),
        };

        let mut product: Product = {
            // Generate Variant Groups
            let mut vcs = vec![];

            let val = match reader.get(*line) {
                Some(v) => v,
                None => return Err(ParseFailure::EOFException),
            };

            let cloned = (*val).as_ref().unwrap();

            if cloned.title.is_empty() {
                *line += 1;
                return Err(ParseFailure::ReadFailure("Empty Field".to_owned()));
            }

            if !cloned.option_1_name.is_empty() {
                let vc: VariantCategory = VariantCategory {
                    category: (*cloned.option_1_name.clone()).to_string(),
                    variants: vec![],
                };

                vcs.push(vc);
            }

            if !cloned.option_2_name.is_empty() {
                let vc: VariantCategory = VariantCategory {
                    category: (*cloned.option_2_name.clone()).to_string(),
                    variants: vec![],
                };

                vcs.push(vc);
            }

            if !cloned.option_3_name.is_empty() {
                let vc: VariantCategory = VariantCategory {
                    category: (*cloned.option_3_name.clone()).to_string(),
                    variants: vec![],
                };

                vcs.push(vc);
            }

            options = Some(Options {
                option_1_name: (*cloned.option_1_name.clone()).to_string(),
                option_2_name: (*cloned.option_2_name.clone()).to_string(),
                option_3_name: (*cloned.option_3_name.clone()).to_string(),
            });

            Product {
                name: (*cloned.title.clone()).to_string(),
                company: (*cloned.vendor.clone()).to_string(),
                variant_groups: vcs,
                variants: vec![],
                sku: generated_sku.clone(),
                images: vec![(*cloned.image_url.clone()).to_string()],
                tags: vec![(*cloned.tags.clone()).to_string()],
                description: (*cloned.body.clone()).to_string(),
                specifications: vec![],
                name_long: (*cloned.title.clone()).to_string(),
                identification: pdt_ident.clone(),
                description_long: (*cloned.body.clone()).to_string(),
                visible: open_stock::ProductVisibility::ShowWhenInStock,
            }
        };

        // Keep parsing till reached.
        while let Some(val) = reader.get(*line) {
            let cloned = (*val).as_ref().unwrap();

            if (*cloned.title.clone()).to_string() != "" && *line.deref() != init_line
                || (*cloned.title.clone()).to_string() == "" && cloned.price.is_empty()
            {
                // End of valid product range
                break;
            }

            let mut actual_title = format!(
                "{} {} {}",
                &(*cloned.option_1_value.clone()),
                &(*cloned.option_2_value.clone()),
                &(*cloned.option_3_value.clone())
            )
            .trim()
            .to_string();
            if actual_title == "Default Title" {
                actual_title = product.name.clone();
            }

            let price = match cloned.price.parse::<f32>() {
                Ok(p) => p,
                Err(err) => return Err(ParseFailure::FormatFailure(err.to_string())),
            };

            let variant = VariantInformation {
                name: actual_title,
                stock: vec![], // Stock must be loaded from a stock CSV in shopify
                images: vec![(*cloned.variant_image.clone()).to_string()],
                retail_price: price,
                marginal_price: cloned.marginal_cost.parse::<f32>().unwrap_or(price),
                loyalty_discount: DiscountValue::Absolute(0),
                variant_code: vec![(*cloned.sku.clone()).to_string()],
                order_history: vec![],
                stock_information: StockInformation {
                    stock_group: (*cloned.prod_type.clone()).to_string(),
                    sales_group: (*cloned.product_category.clone()).to_string(),
                    value_stream: String::new(),
                    brand: (*cloned.vendor.clone()).to_string(),
                    tax_code: (*cloned.tax_code.clone()).to_string(),
                    weight: (*cloned.weight_grams.clone()).to_string(),
                    volume: "0.00".to_string(),
                    max_volume: "0.00".to_string(),
                    back_order: false,
                    discontinued: (cloned.status.clone()) == "active",
                    non_diminishing: false,
                    shippable: (cloned.requires_shipping.clone()) == "TRUE",
                    size_override_unit: (*cloned.weight_unit.clone()).to_string(),
                    size_x_unit: (*cloned.weight_unit.clone()).to_string(),
                    size_y_unit: (*cloned.weight_unit.clone()).to_string(),
                    size_z_unit: (*cloned.weight_unit.clone()).to_string(),
                    size_x: 0.0,
                    size_y: 0.0,
                    size_z: 0.0,
                    min_stock_before_alert: 0.0,
                    min_stock_level: 0.0,
                    colli: String::new(),
                },
                barcode: (*cloned.barcode.clone()).to_string(),
                id: uuid::Uuid::new_v4().to_string(),
                buy_max: -1.0,
                // Considers if the quantity is a decimal,
                // otherwise would take value `1.0`.
                buy_min: 0.0,
                identification: pdt_ident.clone(),
                stock_tracking: true,
            };

            let options = options.clone();

            if !cloned.option_1_value.is_empty() {
                let vc: Variant = Variant {
                    name: (*cloned.option_1_value.clone()).to_string(),
                    images: vec![(*cloned.variant_image.clone()).to_string()],
                    marginal_price: 0.00,
                    variant_code: format!(
                        "{}-{}",
                        options.clone().expect("").option_1_name,
                        &(*cloned.option_1_value.clone())
                    ),
                    order_history: vec![],
                };

                let existing_index = product
                    .variant_groups
                    .iter()
                    .position(|x| x.category == options.clone().expect("").option_1_name);

                match existing_index {
                    Some(val) => {
                        let existing_index_2 = product
                            .variant_groups
                            .get_mut(val)
                            .expect("")
                            .variants
                            .iter()
                            .position(|x| x.name == (*cloned.option_1_value.clone()));

                        match existing_index_2 {
                            Some(_) => {}
                            None => {
                                product
                                    .variant_groups
                                    .get_mut(val)
                                    .expect("")
                                    .variants
                                    .push(vc);
                            }
                        }
                    }
                    None => println!(
                        "[err]: Failed trying to place variant {} in group {}.",
                        &(*cloned.option_1_value.clone()),
                        options.clone().expect("").option_1_name
                    ),
                }
            }

            if !cloned.option_2_value.is_empty() {
                let vc: Variant = Variant {
                    name: (*cloned.option_2_value.clone()).to_string(),
                    images: vec![(*cloned.variant_image.clone()).to_string()],
                    marginal_price: 0.00,
                    variant_code: format!(
                        "{}-{}",
                        options.clone().expect("").option_2_name,
                        &(*cloned.option_2_value.clone())
                    ),
                    order_history: vec![],
                };

                let existing_index = product
                    .variant_groups
                    .iter()
                    .position(|x| x.category == options.clone().expect("").option_2_name);

                match existing_index {
                    Some(val) => {
                        let existing_index_2 = product
                            .variant_groups
                            .get_mut(val)
                            .expect("")
                            .variants
                            .iter()
                            .position(|x| x.name == (*cloned.option_2_value.clone()));

                        match existing_index_2 {
                            Some(_) => {}
                            None => {
                                product
                                    .variant_groups
                                    .get_mut(val)
                                    .expect("")
                                    .variants
                                    .push(vc);
                            }
                        }
                    }
                    None => println!(
                        "[err]: Failed trying to place variant {} in group {}.",
                        &(*cloned.option_2_value.clone()),
                        options.clone().expect("").option_2_name
                    ),
                }
            }

            if !cloned.option_3_value.is_empty() {
                let vc: Variant = Variant {
                    name: (*cloned.option_3_value.clone()).to_string(),
                    images: vec![(*cloned.variant_image.clone()).to_string()],
                    marginal_price: 0.00,
                    variant_code: format!(
                        "{}-{}",
                        options.clone().expect("").option_3_name,
                        &(*cloned.option_3_value.clone())
                    ),
                    order_history: vec![],
                };

                let existing_index = product
                    .variant_groups
                    .iter()
                    .position(|x| x.category == options.clone().expect("").option_3_name);

                match existing_index {
                    Some(val) => {
                        let existing_index_2 = product
                            .variant_groups
                            .get_mut(val)
                            .expect("")
                            .variants
                            .iter()
                            .position(|x| x.name == (*cloned.option_3_value.clone()));

                        match existing_index_2 {
                            Some(_) => {}
                            None => {
                                product
                                    .variant_groups
                                    .get_mut(val)
                                    .expect("")
                                    .variants
                                    .push(vc);
                            }
                        }
                    }
                    None => println!(
                        "[err]: Failed trying to place variant {} in group {}.",
                        &(*cloned.option_3_value.clone()),
                        options.expect("").option_3_name
                    ),
                }
            }

            product.variants.push(variant);
            *line += 1;
        }

        Ok(product)
    }
}
