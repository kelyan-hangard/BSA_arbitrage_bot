/// Splits a symbol at a specified index
/// For example, `split_and_reverse_symbol("LDOUSDT", 3)` will return "LDO-USDT".
/// Fit for the OKX standard representation of a symbol.
pub fn split_symbol_okx(symbol: &str, split_index: usize) -> String {
    let (base, quote) = symbol.split_at(split_index);
    format!("{}-{}", base, quote)
}

/// Splits a symbol at a specified index and reverses the order.
/// For example, `split_and_reverse_symbol("LDOUSDT", 3)` will return "USDT_LDO".
/// Fit for the Poloniex standard of a symbol.
pub fn split_and_reverse_symbol_poloniex(symbol: &str, split_index: usize) -> String {
    if symbol.len() > split_index {
        let (base, quote) = symbol.split_at(split_index);
        format!("{}_{}", quote, base)
    } else {
        symbol.to_string()
    }
}