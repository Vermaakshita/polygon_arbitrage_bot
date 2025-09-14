mod config;
mod dex;
mod arbitrage;
mod db;

use ethers::providers::{Provider, Http};
use std::sync::Arc;
use ethers::types::Address;

#[tokio::main]
async fn main() {
    // Load config
    let config = config::Config::from_file("config.toml");
    println!("Loaded config: {:?}", config);

    // Set up provider
    let provider = Arc::new(Provider::<Http>::try_from(config.rpc_url.as_str()).expect("Invalid RPC URL"));

    // Set up DEXes
    let dex_a = dex::Dex::new(&config.dex_a_router, provider.clone());
    let dex_b = dex::Dex::new(&config.dex_b_router, provider.clone());

    // Prepare path (token_a -> token_b)
    let path = vec![
        config.token_a.parse::<Address>().unwrap(),
        config.token_b.parse::<Address>().unwrap(),
    ];
    let amount_in = ethers::utils::parse_units(config.trade_amount, 18).unwrap();

    // Set up database (optional)
    let db = match db::Db::connect().await {
        Ok(db) => Some(db),
        Err(e) => {
            println!("[WARN] Could not connect to database: {e}");
            None
        }
    };

    // Fetch prices
    let price_a = dex_a.get_amount_out(amount_in.into(), path.clone()).await;
    let price_b = dex_b.get_amount_out(amount_in.into(), path.clone()).await;

    if let (Ok(out_a), Ok(out_b)) = (price_a, price_b) {
        // Convert U256 to f64 USDC (6 decimals)
        let price_a_f64 = out_a.as_u128() as f64 / 1_000_000.0;
        let price_b_f64 = out_b.as_u128() as f64 / 1_000_000.0;
        println!("DEX A: {} USDC, DEX B: {} USDC", price_a_f64, price_b_f64);

        // Check for arbitrage opportunity
        let arbitrage_result = arbitrage::detect_arbitrage(
            price_a_f64,
            price_b_f64,
            config.trade_amount,
            config.gas_cost_usdc,
            config.min_profit_usdc,
        );

        // Log market snapshot (always log for knowledge)
        if let Some(db) = &db {
            if let Err(e) = db.log_market_snapshot(
                "DEX_A", "DEX_B",
                &config.token_a, &config.token_b,
                price_a_f64, price_b_f64,
                config.trade_amount,
                config.gas_cost_usdc,
                arbitrage_result,
                arbitrage_result.is_some()
            ).await {
                println!("[WARN] Failed to log market snapshot: {e}");
            }
        }

        // Handle arbitrage opportunity
        if let Some(profit) = arbitrage_result {
            println!("Arbitrage opportunity detected! Simulated profit: ${:.2}", profit);
            if let Some(db) = &db {
                if let Err(e) = db.log_opportunity(
                    "DEX_A", "DEX_B",
                    &config.token_a, &config.token_b,
                    price_a_f64, price_b_f64, profit
                ).await {
                    println!("[WARN] Failed to log arbitrage opportunity: {e}");
                }
            }
        } else {
            println!("No arbitrage opportunity detected.");
        }
    } else {
        println!("Failed to fetch prices from one or both DEXes.");
    }
}
