use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::models::PriceHistory;

#[derive(Debug, Deserialize)]
struct PriceRow {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Adj Close")]
    price: f32,
}

#[derive(Debug, Deserialize)]
struct SymbolRow {
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "ETF")]
    is_etf: String,
}

pub fn fetch_prices_by_id(id: &str) -> Result<PriceHistory> {
    let asset_type = determine_asset_type(id)?;

    let file_path = PathBuf::from("data")
        .join(asset_type)
        .join(format!("{}.csv", id));

    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut prices = Vec::new();
    let mut dates = Vec::new();

    for result in reader.deserialize::<PriceRow>() { 
        let record = result.context("Error deserializing row")?;
        prices.push(record.price);
        let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d")
            .context("Error parsing date")?;
        dates.push(date);
    }

    Ok(PriceHistory { id: id.to_string(), prices, dates })
}

pub fn determine_asset_type(id: &str) -> Result<String> {
    let symbols_file_path = PathBuf::from("data")
        .join("symbols_valid_meta.csv");

    let file = File::open(symbols_file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));

    let mut symbol_map = HashMap::new();
    for result in reader.deserialize::<SymbolRow>() {
        let row = result?;
        symbol_map.insert(row.symbol, row.is_etf); 
    }

    match symbol_map.get(id) {
        Some(is_etf) if is_etf == "Y" => Ok("etfs".to_string()),
        Some(_) => Ok("stocks".to_string()),
        None => Err(anyhow::anyhow!("Symbol '{}' not found", id)),
    }
}