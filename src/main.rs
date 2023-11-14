mod api_clients;
use api_clients::binance;
use api_clients::okx;
use api_clients::poloniex;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "LDOUSDT";
    let interval = "5m"; // You can use different intervals like "1h", "1d", etc.
    let time_back = 3; // times the unit interval
    let rate_limit = 500; // Up to 1000 for binance and poloniex, 500 for OKX.

    let binance_data = binance::get_binance_data(symbol, interval, time_back, rate_limit).await?;
    println!("{:#?}", binance_data);
    println!("-----------------------------------------");
    println!("-----------------------------------------");
    println!("-----------------------------------------");

    let okx_data = okx::get_okx_data(symbol, interval, time_back, rate_limit).await?;
    println!("{:#?}", okx_data);
    // println!("-----------------------------------------");
    // println!("-----------------------------------------");
    // println!("-----------------------------------------");

    // let poloniex_data = poloniex::get_poloniex_data(symbol, interval, time_back).await?;
    // println!("{:#?}", poloniex_data);

    Ok(())
}