use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;
    
    println!("Connected to database successfully!");
    
    // Read and execute migration
    let migration_sql = fs::read_to_string("migrations/001_create_arbitrage_opportunities.sql")?;
    
    println!("Running migration...");
    sqlx::query(&migration_sql)
        .execute(&pool)
        .await?;
    
    println!("Database setup completed successfully!");
    println!("The 'arbitrage_opportunities' table has been created.");
    
    Ok(())
}
