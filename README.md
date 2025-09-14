# Polygon Arbitrage Opportunity Detector Bot

## 🚀 Quick Start (For Beginners)
1. **Install Rust** from [rustup.rs](https://rustup.rs/)
2. **Download this project** to your computer
3. **Open a terminal** in the project folder
4. **Run:** `cargo run`
5. **Watch** as it checks for profitable trading opportunities!

That's it! The bot will tell you if it finds any ways to make money by trading between exchanges.

---

## What Is This Project?
This project is a simple computer program ("bot") written in the Rust programming language. Its job is to look for special trading opportunities called "arbitrage" on the Polygon blockchain. It checks the prices of a cryptocurrency (like WETH or USDC) on two different online exchanges and tells you if you could make a profit by buying on one and selling on the other.

---

## Saving Data to a Database (Optional)
The bot can save information to a database so you can track opportunities over time. This is completely optional - the bot works fine without it.

### What Does the Database Do?
Think of the database like a notebook that keeps track of:
- **Profitable opportunities** - When the bot finds ways to make money
- **All price checks** - Every time it looks at prices (even when no profit is found)

This helps you:
- See patterns in the market over time
- Track how often profitable opportunities appear
- Learn about price differences between exchanges

### How to Set Up Database Logging (Simple Version)
1. **Get a free database** from services like [Neon](https://neon.tech) or [Supabase](https://supabase.com)
2. **Set up your database URL** using one of these methods:

#### Option A: Create a .env file (Recommended)
Create a file called `.env` in the project folder with your database connection info:
```
DATABASE_URL=your_database_connection_string_here
```

#### Option B: Set environment variable in terminal
**For Windows (Command Prompt):**
```cmd
set DATABASE_URL=your_database_connection_string_here
cargo run
```

**For Windows (PowerShell):**
```powershell
$env:DATABASE_URL="your_database_connection_string_here"
cargo run
```

**For Mac/Linux (Terminal/Bash):**
```bash
export DATABASE_URL="your_database_connection_string_here"
cargo run
```

3. **Run the bot** - it will automatically create the database tables for you!

### For Advanced Users: Database Details
<details>
<summary>Click to see technical database information</summary>

The bot creates two tables:

**1. arbitrage_opportunities** - Stores only profitable opportunities:
```sql
CREATE TABLE arbitrage_opportunities (
    id SERIAL PRIMARY KEY,
    dex_a VARCHAR(50) NOT NULL,           -- Name of first DEX
    dex_b VARCHAR(50) NOT NULL,           -- Name of second DEX
    token_a VARCHAR(42) NOT NULL,         -- Token A address
    token_b VARCHAR(42) NOT NULL,         -- Token B address
    price_a DECIMAL(20, 8) NOT NULL,      -- Price on DEX A
    price_b DECIMAL(20, 8) NOT NULL,      -- Price on DEX B
    profit DECIMAL(20, 8) NOT NULL,       -- Calculated profit
    detected_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

**2. market_snapshots** - Records every price check:
```sql
CREATE TABLE market_snapshots (
    id SERIAL PRIMARY KEY,
    dex_a VARCHAR(50) NOT NULL,                    -- Name of first DEX
    dex_b VARCHAR(50) NOT NULL,                    -- Name of second DEX
    token_a VARCHAR(42) NOT NULL,                  -- Token A address
    token_b VARCHAR(42) NOT NULL,                  -- Token B address
    dex_a_price DECIMAL(20, 8) NOT NULL,           -- Price on DEX A
    dex_b_price DECIMAL(20, 8) NOT NULL,           -- Price on DEX B
    price_difference DECIMAL(20, 8) NOT NULL,      -- Absolute price difference
    price_difference_percent DECIMAL(10, 4) NOT NULL, -- Price difference percentage
    trade_amount DECIMAL(20, 8) NOT NULL,          -- Trade amount used
    gas_cost DECIMAL(20, 8) NOT NULL,              -- Gas cost estimate
    potential_profit DECIMAL(20, 8),               -- Calculated profit (NULL if no arbitrage)
    is_arbitrage BOOLEAN NOT NULL DEFAULT FALSE,   -- Whether arbitrage was detected
    snapshot_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

**Setup Commands:**
```bash
# Run the bot (creates tables automatically)
cargo run

# Or run database setup separately
cargo run --bin setup_db
```
</details>

---

## What Is Arbitrage?
**Arbitrage** is when you buy something for a low price in one place and sell it for a higher price in another place, making a profit from the difference. In cryptocurrency, this means buying a token (like WETH) on one exchange where it's cheap, and selling it on another exchange where it's more expensive.

---

## What Is Polygon?
**Polygon** is a blockchain network, similar to Ethereum, but it's faster and cheaper to use. People use Polygon to trade cryptocurrencies and run programs called "smart contracts."

---

## What Is a DEX?
A **DEX** (Decentralized Exchange) is a website or app where you can trade cryptocurrencies directly with other people, without a central company in charge. Examples include SushiSwap and QuickSwap. These DEXes run on the Polygon blockchain.

---

## What Is a Smart Contract?
A **smart contract** is a program that lives on a blockchain. It can hold money, send money, and do other things automatically. DEXes are made up of smart contracts that handle all the trading.

---

## What Is an RPC?
An **RPC** (Remote Procedure Call) is a way for your computer to talk to the Polygon blockchain. You use an RPC URL (like `https://polygon-rpc.com`) to connect your program to the blockchain and ask it for information, like token prices.

---

## How Does This Bot Work?
1. **Connects to Polygon:** The bot uses the RPC URL to connect to the Polygon blockchain.
2. **Checks Prices:** It asks two DEXes (like SushiSwap and QuickSwap) what the current price is for trading one token for another (for example, WETH for USDC).
3. **Looks for Arbitrage:** It compares the prices. If you could buy a token cheaply on one DEX and sell it for more on the other, the bot calculates how much profit you could make (after subtracting a small fee for transaction costs, called "gas").
4. **Tells You:** If the profit is big enough (more than a minimum you set), the bot prints a message saying there's an arbitrage opportunity.

---

## Why Is This Useful?
- **Find Profits:** It helps you spot ways to make money by trading between exchanges.
- **Saves Time:** It checks prices automatically, so you don't have to do it by hand.
- **Safe to Try:** It only simulates trades (doesn't actually buy or sell), so you can learn and experiment without risking real money.

---

## What Do I Need to Use This?
- A computer with [Rust](https://rustup.rs/) installed (Rust is a programming language, and you can install it by following the instructions on that website).
- Internet access.
- (Optional) Your own Polygon RPC URL (the default public one is fine for learning).

---

## How Do I Set It Up?

### 1. Download or Clone the Project
If you know how to use Git:
```
git clone <your-repo-url>
cd polygon-arb-bot
```
Or just download the folder and open it on your computer.

### 2. Edit the Configuration File
Open the file called `config.toml` in a text editor. It looks like this:
```toml
rpc_url = "https://polygon-rpc.com"                # How the bot connects to Polygon

dex_a_router = "0x1b02da8cb0d097eb8d57a175b88c7d8b47997506" # Address of SushiSwap's smart contract

dex_b_router = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff" # Address of QuickSwap's smart contract

token_a = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"      # Address of WETH token

token_b = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"      # Address of USDC token

min_profit_usdc = 1.0      # Minimum profit (in USDC) to tell you about an opportunity
trade_amount = 0.1         # How much WETH to pretend to trade

# This is a guess of how much it costs to do a trade ("gas cost"), in USDC
# It's just for simulation, not a real fee.
gas_cost_usdc = 0.2
```
**You can change these values to look for different tokens, DEXes, or profit amounts.**

### 3. Build the Bot
Open a terminal (Command Prompt, PowerShell, or a terminal in VSCode) in the `polygon-arb-bot` folder and run:
```
cargo build
```
This will download everything the bot needs and check for errors.

### 4. Run the Bot
In the same terminal, run:
```
cargo run
```
You should see output like this:
```
Loaded config: Config { ... }
DEX A: 3010.23 USDC, DEX B: 3022.15 USDC
Arbitrage opportunity detected! Simulated profit: $10.95
```
If there is no opportunity, it will say so.

---

## What Do the Files Do?
**Main Files (you'll use these):**
- **config.toml**: Where you change settings like which tokens to check
- **README.md**: This help file

**Code Files (the bot's "brain"):**
- **src/main.rs**: The main program that runs everything
- **src/config.rs**: Reads your settings from config.toml
- **src/dex.rs**: Gets prices from the exchanges
- **src/arbitrage.rs**: Calculates if there's a profit opportunity
- **src/db.rs**: Saves data to database (if you set one up)

**Database Files (only if you use database logging):**
- **setup_db.rs**: Helps set up the database
- **migrations/**: Database setup files

**System Files:**
- **Cargo.toml**: Tells the computer what extra tools the bot needs

---

## What If I Want to Change Something?
- **Different Tokens:** Change `token_a` and `token_b` in `config.toml` to the addresses of the tokens you want to check.
- **Different DEXes:** Change `dex_a_router` and `dex_b_router` to the addresses of other DEXes' router contracts.
- **Different Amounts:** Change `trade_amount` to try different trade sizes.
- **Different Minimum Profit:** Change `min_profit_usdc` to only see bigger or smaller opportunities.

---

## Can I Lose Money With This Bot?
**No!** This bot only simulates trades. It does not actually buy or sell anything. It is safe to use for learning and testing. If you want to make real trades, you would need to add more code to connect a wallet and send transactions (not recommended for beginners).

---

## What If I Get an Error?
- Make sure you have Rust installed.
- Make sure you are in the `polygon-arb-bot` folder in your terminal.
- Make sure your `config.toml` is filled out correctly.
- Make sure you have internet access.
- If you see a message about "failed to fetch prices," the DEX or Polygon network might be busy or down. Try again later.

---

## Where Can I Learn More?
- [What is Arbitrage? (Investopedia)](https://www.investopedia.com/terms/a/arbitrage.asp)
- [What is Polygon? (Polygon Technology)](https://polygon.technology/)
- [What is a DEX? (Coinbase)](https://www.coinbase.com/learn/crypto-basics/what-is-a-dex)
- [Learn Rust](https://www.rust-lang.org/learn)

---

## Final Notes
- This bot is for **learning and research only**. It does not make real trades.
- You can use it to understand how arbitrage works and how to interact with blockchains and smart contracts.
- If you want to build more advanced bots, learn more about Rust, blockchains, and smart contracts first.

---

**Happy learning and exploring the world of crypto arbitrage!**
