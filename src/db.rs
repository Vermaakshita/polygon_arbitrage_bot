use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

pub struct Db {
    pub pool: PgPool,
}

impl Db {
    pub async fn connect() -> anyhow::Result<Self> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        
        // Create arbitrage_opportunities table
        let create_arbitrage_table_sql = r#"
            CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                id SERIAL PRIMARY KEY,
                dex_a VARCHAR(50) NOT NULL,
                dex_b VARCHAR(50) NOT NULL,
                token_a VARCHAR(42) NOT NULL,
                token_b VARCHAR(42) NOT NULL,
                price_a DECIMAL(20, 8) NOT NULL,
                price_b DECIMAL(20, 8) NOT NULL,
                profit DECIMAL(20, 8) NOT NULL,
                detected_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#;
        
        // Create market_snapshots table for all price checks
        let create_snapshots_table_sql = r#"
            CREATE TABLE IF NOT EXISTS market_snapshots (
                id SERIAL PRIMARY KEY,
                dex_a VARCHAR(50) NOT NULL,
                dex_b VARCHAR(50) NOT NULL,
                token_a VARCHAR(42) NOT NULL,
                token_b VARCHAR(42) NOT NULL,
                dex_a_price DECIMAL(20, 8) NOT NULL,
                dex_b_price DECIMAL(20, 8) NOT NULL,
                price_difference DECIMAL(20, 8) NOT NULL,
                price_difference_percent DECIMAL(10, 4) NOT NULL,
                trade_amount DECIMAL(20, 8) NOT NULL,
                gas_cost DECIMAL(20, 8) NOT NULL,
                potential_profit DECIMAL(20, 8),
                is_arbitrage BOOLEAN NOT NULL DEFAULT FALSE,
                snapshot_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#;
        
        sqlx::query(create_arbitrage_table_sql)
            .execute(&pool)
            .await?;
            
        sqlx::query(create_snapshots_table_sql)
            .execute(&pool)
            .await?;
        
        println!("Database tables 'arbitrage_opportunities' and 'market_snapshots' are ready!");
        Ok(Self { pool })
    }

    pub async fn log_opportunity(&self, dex_a: &str, dex_b: &str, token_a: &str, token_b: &str, price_a: f64, price_b: f64, profit: f64) -> anyhow::Result<()> {
        let insert_sql = r#"
            INSERT INTO arbitrage_opportunities (dex_a, dex_b, token_a, token_b, price_a, price_b, profit, detected_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
        "#;
        
        sqlx::query(insert_sql)
            .bind(dex_a)
            .bind(dex_b)
            .bind(token_a)
            .bind(token_b)
            .bind(price_a)
            .bind(price_b)
            .bind(profit)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn log_market_snapshot(
        &self, 
        dex_a: &str, 
        dex_b: &str, 
        token_a: &str, 
        token_b: &str, 
        price_a: f64, 
        price_b: f64, 
        trade_amount: f64,
        gas_cost: f64,
        potential_profit: Option<f64>,
        is_arbitrage: bool
    ) -> anyhow::Result<()> {
        let price_difference = (price_a - price_b).abs();
        let price_difference_percent = if price_a > 0.0 {
            (price_difference / price_a) * 100.0
        } else {
            0.0
        };

        let insert_sql = r#"
            INSERT INTO market_snapshots (
                dex_a, dex_b, token_a, token_b, 
                dex_a_price, dex_b_price, price_difference, price_difference_percent,
                trade_amount, gas_cost, potential_profit, is_arbitrage, snapshot_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW())
        "#;
        
        sqlx::query(insert_sql)
            .bind(dex_a)
            .bind(dex_b)
            .bind(token_a)
            .bind(token_b)
            .bind(price_a)
            .bind(price_b)
            .bind(price_difference)
            .bind(price_difference_percent)
            .bind(trade_amount)
            .bind(gas_cost)
            .bind(potential_profit)
            .bind(is_arbitrage)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
