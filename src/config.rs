use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub dex_a_router: String,
    pub dex_b_router: String,
    pub token_a: String,
    pub token_b: String,
    pub min_profit_usdc: f64,
    pub trade_amount: f64,
    pub gas_cost_usdc: f64,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse config file")
    }
}
