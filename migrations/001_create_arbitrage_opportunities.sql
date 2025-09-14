-- Create arbitrage_opportunities table
CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
    id SERIAL PRIMARY KEY,
    dex_a VARCHAR(50) NOT NULL,
    dex_b VARCHAR(50) NOT NULL,
    token_a VARCHAR(42) NOT NULL, -- Ethereum address format
    token_b VARCHAR(42) NOT NULL, -- Ethereum address format
    price_a DECIMAL(20, 8) NOT NULL,
    price_b DECIMAL(20, 8) NOT NULL,
    profit DECIMAL(20, 8) NOT NULL,
    detected_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index for faster queries on detection time
CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_detected_at 
ON arbitrage_opportunities(detected_at);

-- Create index for queries by token pair
CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_tokens 
ON arbitrage_opportunities(token_a, token_b);
