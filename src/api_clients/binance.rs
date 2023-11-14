use reqwest;
use serde_json::Value;
use std::error::Error;

//should it be pub?
pub async fn get_binance_data(symbol: &str, limit: usize) -> Result<Value, Box<dyn Error>> {

    // Create the URL with query parameters
    let url = format!(
        "https://api.binance.com/api/v3/trades?symbol={}&limit={}",
        symbol, limit
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