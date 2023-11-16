use serde_json::Value;
use std::error::Error;
use std::collections::VecDeque;
use std::num::ParseIntError;
use csv::Writer;

pub fn arbitrage(binance_data: &Value, okx_data: &Value) -> Result<VecDeque<(String, String, f64)>, Box<dyn Error>> {
    let binance_fees: f64 = 0.0; // Example fee percentage for Binance
    let okx_fees: f64 = 0.0; // Example fee percentage for OKX
    let mut arbitrage_opportunities = VecDeque::new();
    let mut wtr = Writer::from_path("matching_trades_3.csv")?;
    wtr.write_record(&["OKX Price", "OKX Timestamp", "Binance Price", "Binance Timestamp"])?;

    if let Some(binance_trades) = binance_data.as_array() {
        if let Some(okx_trades) = okx_data.as_array() {
            for o_trade in okx_trades {
                if let Ok(o_time) = parse_timestamp(o_trade, "ts") {
                    let o_price: f64 = o_trade["px"].as_str()
                        .ok_or("Price field missing for OKX trade")?
                        .parse()?;
                    
                    println!("OKX trade: time = {}, price = {}", o_time, o_price);
                    let mut min_diff = i64::MAX;
                    let mut closest_b_trade_timestamp: Option<i64> = None;


                    if let Some(closest_b_trade_timestamp) = binance_trades.iter()
                    .filter_map(|b_trade| parse_timestamp(b_trade, "time").ok())
                    .min_by_key(|&b_time| (o_time - b_time).abs())
                    {
                        if let Some(closest_b_trade) = binance_trades.iter().find(|&b_trade| {
                            match parse_timestamp(b_trade, "time") {
                                Ok(b_time) => b_time == closest_b_trade_timestamp,
                                Err(_) => false,
                            }
                        }) {
                            // closest_b_trade is now a serde_json::Value
                            let b_time: i64 = parse_timestamp(closest_b_trade, "time").unwrap_or(i64::MAX);
                            let time_diff = (o_time - b_time).abs();
                            let b_price: f64 = closest_b_trade["price"].as_str().unwrap().parse()?;
                            wtr.write_record(&[
                                o_price.to_string(), 
                                o_time.to_string(), 
                                b_price.to_string(), 
                                b_time.to_string()
                            ])?;
        
                            if time_diff <= 100 {
                                
                                let price_diff = (b_price - o_price).abs();
                                
                                // Calculate the fees
                                let total_fees = (b_price * binance_fees / 100.0) + (o_price * okx_fees / 100.0);

                                if price_diff > total_fees {
                                    // Arbitrage opportunity found
                                    arbitrage_opportunities.push_back((
                                        closest_b_trade["time"].to_string(),
                                        o_trade["ts"].to_string(),
                                        price_diff
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Analyze the arbitrage opportunities
    analyze_arbitrage_opportunities(&arbitrage_opportunities);

    Ok(arbitrage_opportunities)
}

fn analyze_arbitrage_opportunities(arbitrage_opportunities: &VecDeque<(String, String, f64)>) {
    // Implement your analysis logic here.
    // For example, calculate the number of opportunities and their average duration.
    println!("Number of Arbitrage Opportunities: {}", arbitrage_opportunities.len());
    // Further analysis can be added as needed
}

// Helper function to parse timestamp
fn parse_timestamp(trade: &Value, field: &str) -> Result<i64, Box<dyn std::error::Error>> {
    match trade.get(field) {
        Some(Value::String(ts_str)) => ts_str.parse::<i64>().map_err(|e| e.into()),
        Some(Value::Number(num)) => num.as_i64().ok_or_else(|| "Invalid number format".into()),
        _ => Err(format!("Timestamp field '{}' missing or not a string/number", field).into()),
    }
}

