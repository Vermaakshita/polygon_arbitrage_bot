pub fn detect_arbitrage(
    price_a: f64,
    price_b: f64,
    trade_amount: f64,
    gas_cost: f64,
    min_profit: f64,
) -> Option<f64> {
    // Buy on A, sell on B
    let profit = (price_b - price_a) * trade_amount - gas_cost;
    if profit > min_profit {
        Some(profit)
    } else {
        None
    }
}
