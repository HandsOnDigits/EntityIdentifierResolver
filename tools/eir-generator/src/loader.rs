use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub aliases: Vec<String>,
    pub company: u64,
    pub category: u64,
    pub country: u64,
    pub tags: Vec<String>,
}

pub fn load_products(path: &str) -> Result<Vec<Product>> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}
