use alloy::primitives::Address;

use crate::{
    etherscan::client::EtherscanClient,
    types::{EtherscanResponse, Transaction},
};

impl EtherscanClient {
    pub async fn get_funding_source(&self, deployer: &Address) -> Option<Address> {
        // Get the first normal transaction to the deployer wallet
        let deployer_str = deployer.to_string();

        let params = vec![
            ("module", "account"),
            ("action", "txlist"),
            ("address", &deployer_str),
            ("startblock", "0"),
            ("endblock", "99999999"),
            ("page", "1"),
            ("offset", "1"),
            ("sort", "asc"),
        ];

        let res = self
            .get::<EtherscanResponse<Vec<Transaction>>>(&params)
            .await?;

        let first_tx = res.result.first()?;
        let funder: Address = first_tx.from.parse().ok()?;
        Some(funder)
    }
}
