use reqwest;
use serde_json::Value;
use std::error::Error;
use chrono::{Duration, Utc};

//should it be pub?
pub async fn get_binance_data(symbol: &str, interval: &str, time_back: i64, limit: usize) -> Result<Value, Box<dyn Error>> {
    // Define your time range (example: last month)
    // NOTE: Replace these with actual timestamps in milliseconds for the past month.

    let now = Utc::now();
    let end_time = now.timestamp_millis();
    let start_time = (now - Duration::days(time_back)).timestamp_millis();

    // Create the URL with query parameters
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit={}&startTime={}&endTime={}",
        symbol, interval, limit, start_time, end_time
    );

    // Send the GET request
    let resp = reqwest::get(&url).await?;
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