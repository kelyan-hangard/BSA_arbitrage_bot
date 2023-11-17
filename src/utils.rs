use serde_json::Value;

/// Splits a symbol at a specified index
/// For example, `split_and_reverse_symbol("LDOUSDT", 3)` will return "LDO-USDT".
/// Fit for the OKX standard representation of a symbol.
pub fn split_symbol_okx(symbol: &str, split_index: usize) -> String {
    let (base, quote) = symbol.split_at(split_index);
    format!("{}-{}", base, quote)
}

// Helper function to parse timestamp
pub fn parse_timestamp(trade: &Value, field: &str) -> Result<i64, Box<dyn std::error::Error>> {
    match trade.get(field) {
        Some(Value::String(ts_str)) => ts_str.parse::<i64>().map_err(|e| e.into()),
        Some(Value::Number(num)) => num.as_i64().ok_or_else(|| "Invalid number format".into()),
        _ => Err(format!("Timestamp field '{}' missing or not a string/number", field).into()),
    }
}