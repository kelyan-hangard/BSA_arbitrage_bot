mod api_clients;
use api_clients::binance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "LDOUSDT";
    let interval = "1h"; // You can use different intervals like "1h", "1d", etc.
    let limit = 800; // You can fetch up to 1000 entries, the maximum allowed by the API.

    let data = binance::get_binance_data(symbol, interval, limit).await?;
    println!("{:#?}", data);

    Ok(())
}