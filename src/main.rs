mod api_clients;
use api_clients::binance;
use api_clients::okx;
mod utils;
mod arbitrage;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "LDOUSDT";
    let binance_rate_limit = 1000; // Up to 1000 for binance and poloniex, 500 for OKX.
    let okx_rate_limit = 500;

    let binance_data = binance::get_binance_data(symbol, binance_rate_limit).await?;
    let okx_data = okx::get_okx_data(symbol, okx_rate_limit).await?;

    arbitrage::historic_arbitrage(&binance_data, &okx_data)?;
    println!("You can see historic arbitrage opportunitiy details in historic_arbitrage.csv");

    println!("________________________________________________________________________________");
    println!("Starting live arbitrage monitoring...\n");

    arbitrage::live_arbitrage(symbol).await;    
    Ok(())
}


