use reqwest;
use serde_json::Value;
use std::error::Error;
use chrono::{Duration, Utc};
use crate::utils::split_symbol_okx;

pub async fn get_okx_data(imported_symbol: &str, interval: &str, time_back: i64, limit: usize) -> Result<Value, Box<dyn Error>> {
    let start_time = (Utc::now() - Duration::days(time_back)).timestamp_millis();
    let bar = match interval {
        "1d" => "1D",
        "1h" => "1H",
        "5M" => "5m",
        // add other relevant intervals
        _ => interval,
    };
    let split_index = 3;
    let symbol = split_symbol_okx(imported_symbol, split_index);

    let url = format!(
        "https://www.okx.com/api/v5/market/history-candles?instId={}&bar={}&after={}&limit={}",
        symbol, bar, start_time, limit
    );

    let resp = reqwest::get(&url).await?;
    if !resp.status().is_success() {
        return Err(Box::new(resp.error_for_status().err().unwrap()));
    }
    
    let body: serde_json::Value = resp.json().await?;
    if body["code"] == "0" {
        Ok(body["data"].clone())
    }  else {
        println!("The error code is: {}", body["code"]);
        Err("Error".into()) 
    }
}