use std::{fs::File, ops::Deref};
use csv::Reader;
use open_stock::{Product, VariantInformation, DiscountValue, StockInformation, VariantCategory, Variant, Customer, Transaction};
use serde::{Serialize, Deserialize};
use crate::{parser::ParseFailure};

#[derive(Debug, Clone)]
struct Options {
    option_1_name: String,
    option_2_name: String,
    option_3_name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
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

pub fn parse_customers(mut reader: Reader<File>) -> Result<Vec<Customer>, ParseFailure> {
    Err(ParseFailure::EOFException)
}

pub fn parse_transactions(mut reader: Reader<File>) -> Result<Vec<Transaction>, ParseFailure> {
    Err(ParseFailure::EOFException)
}

pub fn parse_products(mut reader: Reader<File>) -> Result<Vec<Product>, ParseFailure> {
    let collected: Vec<Result<Record, csv::Error>> = reader.deserialize().collect();
    let mut iterator: usize = 0;
    let mut products: Vec<Product> = vec![];

    loop {
        // Run following till EOF
        match parse_individual_product(&collected, &mut iterator) {
            Ok(pdt) => products.push(pdt),
            Err(err) => {
                match err {
                    ParseFailure::EOFException => break,
                    error => {
                        println!("[warn]: Parser Warning: {:?}", error);
                    }
                }
            },
        }
    }

    Ok(products)
}

fn parse_individual_product(reader: &Vec<Result<Record, csv::Error>>, line: &mut usize) -> Result<Product, ParseFailure> {
    let init_line = line.clone();
    let mut options: Option<Options> = None;
    
    let mut product: Product = {
        // Generate Variant Groups
        let mut vcs = vec![];

        let val = match reader.get(*line) {
            Some(v) => v,
            None => return Err(ParseFailure::EOFException)
        };

        let cloned = (*val.clone()).as_ref().unwrap();

        if cloned.title == "" {
            *line += 1;
            return Err(ParseFailure::ReadFailure("Empty Field".to_owned()))
        }

        if cloned.option_1_name != "" { 
            let vc: VariantCategory = VariantCategory {
                category: (*cloned.option_1_name.clone()).to_string(),
                variants: vec![]
            };

            vcs.push(vc);
        }

        if cloned.option_2_name != "" { 
            let vc: VariantCategory = VariantCategory {
                category: (*cloned.option_2_name.clone()).to_string(),
                variants: vec![]
            };

            vcs.push(vc);
        }

        if cloned.option_3_name != "" { 
            let vc: VariantCategory = VariantCategory {
                category: (*cloned.option_3_name.clone()).to_string(),
                variants: vec![]
            };

            vcs.push(vc); 
        }

        options = Some(Options {
            option_1_name: (*cloned.option_1_name.clone()).to_string(),
            option_2_name: (*cloned.option_2_name.clone()).to_string(),
            option_3_name: (*cloned.option_3_name.clone()).to_string()
        });

        Product {
            name: (*cloned.title.clone()).to_string(),
            company: (*cloned.vendor.clone()).to_string(),
            variant_groups: vcs,
            variants: vec![],
            sku: uuid::Uuid::new_v4().to_string(),
            images: vec![(*cloned.image_url.clone()).to_string()],
            tags: vec![(*cloned.tags.clone()).to_string()],
            description: (*cloned.body.clone()).to_string(),
            specifications: vec![],
        }
    };

    // Keep parsing till reached.
    loop {
        let val = match reader.get(*line) {
            Some(v) => v,
            None => {
                break
            }
        };

        let cloned = (*val.clone()).as_ref().unwrap();

        if (*cloned.title.clone()).to_string() != "" && *line.deref() != init_line {
            // Should skip line, is a new product
            break;
        }else if (*cloned.title.clone()).to_string() == "" && cloned.price == "" {
            // End of valid product range
            break;
        }

        let mut actual_title = format!("{} {} {}", (*cloned.option_1_value.clone()).to_string(), (*cloned.option_2_value.clone()).to_string(), (*cloned.option_3_value.clone()).to_string()).trim().to_string();
        if actual_title == "Default Title" {
            actual_title = product.name.clone();
        }

        let variant = VariantInformation {
            name: actual_title,
            stock: vec![], // Stock must be loaded from a stock CSV in shopify
            images: vec![(*cloned.variant_image.clone()).to_string()],
            retail_price: cloned.price.parse::<f32>().unwrap(),
            marginal_price: cloned.marginal_cost.parse::<f32>().unwrap_or(cloned.price.parse::<f32>().unwrap()),
            loyalty_discount: DiscountValue::Absolute(0),
            variant_code: vec![(*cloned.sku.clone()).to_string()],
            order_history: vec![],
            stock_information: StockInformation {
                stock_group: (*cloned.prod_type.clone()).to_string(),
                sales_group: (*cloned.product_category.clone()).to_string(),
                value_stream: format!(""),
                brand: (*cloned.vendor.clone()).to_string(),
                unit: (*cloned.weight_unit.clone()).to_string(),
                tax_code: (*cloned.tax_code.clone()).to_string(),
                weight: (*cloned.weight_grams.clone()).to_string(),
                volume: format!("0.00"),
                max_volume: format!("0.00"),
                back_order: false,
                discontinued: (cloned.status.clone()) == "active",
                non_diminishing: false,
                shippable: (cloned.requires_shipping.clone()) == "TRUE",
            },
            barcode: (*cloned.barcode.clone()).to_string(),
            id: uuid::Uuid::new_v4().to_string(),
        };

        let options = options.clone();

        if cloned.option_1_value != "" { 
            let vc: Variant = Variant { 
                name: (*cloned.option_1_value.clone()).to_string(), 
                images: vec![(*cloned.variant_image.clone()).to_string()], 
                marginal_price: 0.00, 
                variant_code: format!("{}-{}", options.clone().expect("").option_1_name, (*cloned.option_1_value.clone()).to_string()), 
                order_history: vec![]
            };

            let existing_index = product.variant_groups.iter().position(|x| {
                x.category == options.clone().expect("").option_1_name
            });
            
            match existing_index {
                Some(val) => { 
                    let existing_index_2 = product.variant_groups
                        .get_mut(val)
                        .expect("")
                        .variants.iter()
                        .position(|x| {
                            x.name == (*cloned.option_1_value.clone()).to_string()
                        });

                    match existing_index_2 {
                        Some(_) => {},
                        None => {
                            product.variant_groups
                                .get_mut(val)
                                .expect("")
                                .variants
                                .push(vc);
                        }
                    } 
                },
                None => println!("[err]: Failed trying to place variant {} in group {}.", (*cloned.option_1_value.clone()).to_string(), options.clone().expect("").option_1_name),
            }
        }

        if cloned.option_2_value != "" { 
            let vc: Variant = Variant { 
                name: (*cloned.option_2_value.clone()).to_string(), 
                images: vec![(*cloned.variant_image.clone()).to_string()], 
                marginal_price: 0.00, 
                variant_code: format!("{}-{}", options.clone().expect("").option_2_name, (*cloned.option_2_value.clone()).to_string()), 
                order_history: vec![]
            };

            let existing_index = product.variant_groups.iter().position(|x| {
                x.category == options.clone().expect("").option_2_name
            });

            match existing_index {
                Some(val) => {
                    let existing_index_2 = product.variant_groups
                        .get_mut(val)
                        .expect("")
                        .variants.iter()
                        .position(|x| {
                            x.name == (*cloned.option_2_value.clone()).to_string()
                        });

                    match existing_index_2 {
                        Some(_) => {},
                        None => {
                            product.variant_groups
                                .get_mut(val)
                                .expect("")
                                .variants
                                .push(vc);
                        }
                    } 
                },
                None => println!("[err]: Failed trying to place variant {} in group {}.", (*cloned.option_2_value.clone()).to_string(), options.clone().expect("").option_2_name),
            }
        }

        if cloned.option_3_value != "" { 
            let vc: Variant = Variant { 
                name: (*cloned.option_3_value.clone()).to_string(), 
                images: vec![(*cloned.variant_image.clone()).to_string()], 
                marginal_price: 0.00, 
                variant_code: format!("{}-{}", options.clone().expect("").option_3_name, (*cloned.option_3_value.clone()).to_string()), 
                order_history: vec![]
            };

            let existing_index = product.variant_groups.iter().position(|x| {
                x.category == options.clone().expect("").option_3_name
            });

            match existing_index {
                Some(val) => {
                    let existing_index_2 = product.variant_groups
                        .get_mut(val)
                        .expect("")
                        .variants.iter()
                        .position(|x| {
                            x.name == (*cloned.option_3_value.clone()).to_string()
                        });

                    match existing_index_2 {
                        Some(_) => {},
                        None => {
                            product.variant_groups
                                .get_mut(val)
                                .expect("")
                                .variants
                                .push(vc);
                        }
                    } 
                },
                None => println!("[err]: Failed trying to place variant {} in group {}.", (*cloned.option_3_value.clone()).to_string(), options.expect("").option_3_name),
            }
        }
        
        product.variants.push(variant);
        *line += 1;
    }

    Ok(product)
}