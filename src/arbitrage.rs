use serde_json::Value;
use std::error::Error;
pub fn arbitrage(binance_data: &Value, okx_data: &Value) -> Result<Value, Box<dyn Error>> {
    Ok(Value::Null)
}