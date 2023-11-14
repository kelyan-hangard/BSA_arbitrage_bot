mod api_clients;
use api_clients::binance;
use api_clients::okx;
//use api_clients::poloniex;
use std::collections::VecDeque;
use serde_json::Value;
use std::num::ParseIntError;
mod utils;
mod arbitrage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "LDOUSDT";
    let binance_rate_limit = 1000; // Up to 1000 for binance and poloniex, 500 for OKX.
    let okx_rate_limit = 500;

    let binance_data = binance::get_binance_data(symbol, binance_rate_limit).await?;
    //println!("{:#?}", binance_data);
    println!("-----------------------------------------");
    println!("-----------------------------------------");
    println!("-----------------------------------------");

    let okx_data = okx::get_okx_data(symbol, okx_rate_limit).await?;
    //println!("{:#?}", okx_data);
    // println!("-----------------------------------------");
    // println!("-----------------------------------------");
    // println!("-----------------------------------------");

    // let poloniex_data = poloniex::get_poloniex_data(symbol, interval, time_back).await?;
    // println!("{:#?}", poloniex_data);
    let binance_fees: f64 = 0.0; // Example fee percentage for Binance
    let okx_fees: f64 = 0.0; // Example fee percentage for OKX

    let mut arbitrage_opportunities = VecDeque::new();

    if let Some(binance_trades) = binance_data.as_array() {
        if let Some(okx_trades) = okx_data.as_array() {
            for o_trade in okx_trades {
                if let Ok(o_time) = parse_timestamp(o_trade, "ts") {
                    let o_price: f64 = o_trade["px"].as_str()
                        .ok_or("Price field missing for OKX trade")?
                        .parse()?;
    
                    if let Some(closest_b_trade) = binance_trades.iter()
                        .min_by_key(|&b_trade| {
                            let b_time = parse_timestamp(b_trade, "time").unwrap_or(i64::MAX);
                            (o_time - b_time).abs()
                        })
                    {
                        // closest_b_trade is now a serde_json::Value
                        let b_time: i64 = parse_timestamp(closest_b_trade, "time").unwrap_or(i64::MAX);
                        let time_diff = (o_time - b_time).abs();
    
                        if time_diff <= 100 {
                            let b_price: f64 = closest_b_trade["price"].as_str().unwrap().parse()?;
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

    // Analyze the arbitrage opportunities
    analyze_arbitrage_opportunities(arbitrage_opportunities);

    Ok(())
}

fn analyze_arbitrage_opportunities(arbitrage_opportunities: VecDeque<(String, String, f64)>) {
    // Implement your analysis logic here.
    // For example, calculate the number of opportunities and their average duration.
    println!("Number of Arbitrage Opportunities: {}", arbitrage_opportunities.len());
    // Further analysis can be added as needed
}

// Helper function to parse timestamp
fn parse_timestamp(trade: &Value, field: &str) -> Result<i64, Box<dyn std::error::Error>> {
    trade[field].as_str()
        .ok_or("Timestamp field missing")?
        .parse::<i64>()
        .map_err(|e: ParseIntError| e.into())
}