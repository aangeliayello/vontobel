use anyhow::Result;
use chrono::{NaiveDate, Datelike};
use reqwest::get;

use crate::models::{PriceHistory, DailyReturnHistory, YtdReturn};
use crate::response::Response;

pub async fn fetch_price_history(id: &str) -> Result<PriceHistory> {
    let api_url = format!("http://localhost:8080/price_history/{}", id); // TODO: Remove hardcoded local host
    let response = get(&api_url).await?.json::<Response<PriceHistory>>().await?;
    response.data.ok_or_else(|| anyhow::anyhow!("No data found in response"))
}

pub async fn calculate_daily_returns(id: &str) -> Result<DailyReturnHistory> {
    let price_history = fetch_price_history(id).await?;

    if price_history.prices.len() < 2 {
        return Err(anyhow::anyhow!("Not enough data to calculate daily returns"));
    }

    let mut return_values = Vec::new();
    let mut dates = Vec::new();

    for i in 1..price_history.prices.len() {
        let return_value = (price_history.prices[i] - price_history.prices[i - 1]) / price_history.prices[i - 1];
        return_values.push(return_value);
        dates.push(price_history.dates[i]);
    }

    Ok(DailyReturnHistory { id: id.to_string(), return_values, dates })
}

pub async fn calculate_ytd_return(id: &str, as_of_date: NaiveDate) -> Result<YtdReturn> {
    let daily_returns_history = calculate_daily_returns(id).await?;

    let start_of_year = NaiveDate::from_ymd_opt(as_of_date.year(), 1, 1)
        .ok_or_else(|| anyhow::anyhow!("Error creating start of year date"))?;

    let mut ytd_return = 1.0;
    
    for i in 0..daily_returns_history.dates.len() {
        if daily_returns_history.dates[i] >= start_of_year {    
            if daily_returns_history.dates[i] >= as_of_date {
                break;
            }
            ytd_return *= 1.0 + daily_returns_history.return_values[i];
            println!("{}", daily_returns_history.return_values[i]);
        }
    }

    Ok(YtdReturn { id: id.to_string(), ytd_return_value: ytd_return - 1.0, as_of_date: as_of_date })
}