use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PriceHistory {
    pub id: String,
    pub prices: Vec<f32>,
    pub dates: Vec<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DailyReturnHistory {
    pub id: String,
    pub return_values: Vec<f32>,
    pub dates: Vec<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YtdReturn {
    pub id: String,
    pub ytd_return_value: f32,
    pub as_of_date: NaiveDate,
}