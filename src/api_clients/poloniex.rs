use reqwest;
use serde_json::Value;
use std::error::Error;
use chrono::{Duration, Utc};
use crate::utils::split_and_reverse_symbol_poloniex;

pub async fn get_poloniex_data(imported_symbol: &str, interval: &str, time_back: i64) -> Result<Value, Box<dyn Error>> {
    let now = Utc::now();
    let end_time = now.timestamp_millis();
    let start_time = (now - Duration::days(time_back)).timestamp_millis();
    let bar = match interval {
        "1d" => 86400,
        "4h" => 14400,
        "2h" => 7200,
        "30M" => 1800,
        "15M" => 900,
        "5M" => 300,
        // add other relevant intervals
        _ => 300,
    };
    let split_index = 3;
    let symbol = split_and_reverse_symbol_poloniex(imported_symbol, split_index);

    let url = format!(
        "https://poloniex.com/public?command=returnChartData&currencyPair={}&start={}&end={}&period={}",
        symbol, start_time, end_time, bar
    );

    // Send the GET request
    let client = reqwest::Client::new();
    let resp = client.get(&url)
                    .header("User-Agent", "Your User Agent")
                    .send()
                    .await?;

    // Check if our request was successful
    if resp.status().is_success() {
        let data: Value = resp.json().await?;
        // Return the data
        Ok(data)
    } else {
        // have better error handling
        Err(Box::new(resp.error_for_status().err().unwrap()))
    }
}