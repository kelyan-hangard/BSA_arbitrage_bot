use reqwest;
use serde_json::Value;
use std::error::Error;
use crate::utils::split_symbol_okx;

pub async fn get_okx_data(imported_symbol: &str, limit: usize) -> Result<Value, Box<dyn Error>> {

    let split_index = 3;
    let symbol = split_symbol_okx(imported_symbol, split_index);

    let url = format!(
        "https://www.okx.com/api/v5/market/trades?instId={}&limit={}",
        symbol, limit
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