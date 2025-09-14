use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::contract::abigen;
use std::sync::Arc;

abigen!(
    UniswapV2Router,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts)
    ]"#
);

pub struct Dex {
    pub router: UniswapV2Router<Provider<Http>>,
}

impl Dex {
    pub fn new(router_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let router = UniswapV2Router::new(router_address.parse::<Address>().unwrap(), provider);
        Self { router }
    }

    pub async fn get_amount_out(&self, amount_in: U256, path: Vec<Address>) -> anyhow::Result<U256> {
        let amounts = self.router.get_amounts_out(amount_in, path).call().await?;
        Ok(*amounts.last().unwrap())
    }
}
