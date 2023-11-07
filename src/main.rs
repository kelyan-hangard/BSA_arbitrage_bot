use reqwest;
use serde_json::Value;
use std::error::Error;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let symbol = "LDOUSDT";
    let interval = "1d"; // You can use different intervals like "1h", "1d", etc.
    let limit = 1000; // You can fetch up to 1000 entries, the maximum allowed by the API.

    // Define your time range (example: last month)
    // NOTE: Replace these with actual timestamps in milliseconds for the past month.

    let now = Utc::now();
    let end_time = now.timestamp_millis();
    let start_time = (now - Duration::days(30)).timestamp_millis();

    // let start_time: i64 = 1633046400000;
    // let end_time: i64 = 1635724800000;    

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
        // Print out the kline data
        println!("{:#?}", data);
    } else {
        // If the request was not successful, print out the error code
        println!("Error: {}", resp.status());
    }

    Ok(())
}
