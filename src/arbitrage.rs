use serde_json::Value;
use std::collections::VecDeque;
use csv::Writer;
use std::time::Instant;
use crate::api_clients::binance;
use crate::api_clients::okx;
use crate::utils;


pub fn historic_arbitrage(binance_data: &Value, okx_data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let binance_fees: f64 = 0.001; //  Binance typically charges a standard trading fee of 0.1% for regular users.
    let okx_fees: f64 = 0.001; // OKX charges 0.08% for makers and 0.10% for takers in spot trading fees.
    let mut arbitrage_opportunities = VecDeque::new();
    let mut wtr = Writer::from_path("historic_arbitrage.csv")?;
    wtr.write_record(&["LDO OKX Price in UDDT", "OKX Timestamp (ms)", "LDO Binance Price in USDT", "Binance Timestamp (ms)", "Time Difference (ms)", "Price Difference (USDT)", "Net Arbitrage Profit (USDT)"])?;

    if let Some(binance_trades) = binance_data.as_array() {
        if let Some(okx_trades) = okx_data.as_array() {
            for o_trade in okx_trades {
                if let Ok(o_time) = utils::parse_timestamp(o_trade, "ts") {
                    let o_price: f64 = o_trade["px"].as_str()
                        .ok_or("Price field missing for OKX trade")?
                        .parse()?;
                    
                    if let Some(closest_b_trade_timestamp) = binance_trades.iter()
                    .filter_map(|b_trade| utils::parse_timestamp(b_trade, "time").ok())
                    .min_by_key(|&b_time| (o_time - b_time).abs())
                    {
                        if let Some(closest_b_trade) = binance_trades.iter().find(|&b_trade| {
                            match utils::parse_timestamp(b_trade, "time") {
                                Ok(b_time) => b_time == closest_b_trade_timestamp,
                                Err(_) => false,
                            }
                        }) {
                            let b_time: i64 = utils::parse_timestamp(closest_b_trade, "time").unwrap_or(i64::MAX);
                            let time_diff = (o_time - b_time).abs();
                            let b_price: f64 = closest_b_trade["price"].as_str().unwrap().parse()?;
        
                            if time_diff <= 5 {
                                
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
                                    let real_price_diff = price_diff - total_fees;
                                    wtr.write_record(&[
                                        o_price.to_string(), 
                                        o_time.to_string(), 
                                        b_price.to_string(), 
                                        b_time.to_string(),
                                        time_diff.to_string(),
                                        price_diff.to_string(),
                                        real_price_diff.to_string()
                                    ])?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Number of Arbitrage Opportunities: {}", arbitrage_opportunities.len());
    Ok(())
}

pub async fn live_arbitrage(symbol: &str) {
    let binance_rate_limit = 1;
    let okx_rate_limit = 1;
    let binance_fees: f64 = 0.001; // Binance typically charges 0.1%
    let okx_fees: f64 = 0.001; // OKX charges 0.1% for takers

    loop {
        let start_time = Instant::now();

        let binance_data = binance::get_binance_data(symbol, binance_rate_limit).await.unwrap();
        let okx_data = okx::get_okx_data(symbol, okx_rate_limit).await.unwrap();

        let api_call_duration = start_time.elapsed();

        let binance_latest_trade = binance_data.as_array().unwrap().first().unwrap();
        let okx_latest_trade = okx_data.as_array().unwrap().first().unwrap();

        let b_price: f64 = binance_latest_trade["price"].as_str().unwrap().parse().unwrap();
        let o_price: f64 = okx_latest_trade["px"].as_str().unwrap().parse().unwrap();
        let b_time: i64 = utils::parse_timestamp(binance_latest_trade, "time").unwrap();
        let o_time: i64 = utils::parse_timestamp(okx_latest_trade, "ts").unwrap();

        let logic_execution_start_time = Instant::now();
        if (b_price - o_price).abs() > (b_price * binance_fees / 100.0 + o_price * okx_fees / 100.0) {
            println!("\n\n");
            let (buy_exchange, buy_price, sell_exchange, sell_price) = if b_price < o_price {
                ("Binance", b_price, "OKX", o_price)
            } else {
                ("OKX", o_price, "Binance", b_price)
            };
            let net_profit = (sell_price - buy_price) - (sell_price * binance_fees / 100.0 + buy_price * okx_fees / 100.0);

            let logic_execution_duration = logic_execution_start_time.elapsed();
            println!("Arbitrage opportunity detected at timestamp {}.\nBuy LDO on {} at {} USDT and sell on {} at {} USDT.\nNet profit: {} USDT.\nAPI Call Duration: {:?}, Logic Execution Duration: {:?}",
                     std::cmp::max(b_time, o_time), buy_exchange, buy_price, sell_exchange, sell_price, net_profit,
                     api_call_duration, logic_execution_duration);
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
