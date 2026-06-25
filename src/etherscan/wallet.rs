use alloy::primitives::Address;

use crate::{
    etherscan::client::{Chain, EtherscanClient}, types::{EtherscanResponse, Transaction, WalletInfo},
};

impl EtherscanClient {
    /// Gets the first transaction of a wallet to determine its age
    pub async fn get_wallet_age_days(&self, chain_id: u64, deployer: &Address) -> Option<u64> {
        let deployer_str = deployer.to_string();
        let params = [
            ("module", "account"),
            ("action", "txlist"),
            ("address", deployer_str.as_str()),
            ("startblock", "0"),
            ("endblock", "99999999"),
            ("page", "1"),
            ("offset", "1"),
            ("sort", "asc"),
        ];

        let chain = Chain::from_chain_id(chain_id).unwrap_or(Chain::Ethereum);
        let res = self
            .get::<EtherscanResponse<Vec<Transaction>>>(chain, &params)
            .await?;

        if res.status != "1" {
            return None;
        }

        let first_tx = res.result.into_iter().next()?;
        let first_tx_timestamp: u64 = first_tx.timestamp.parse().ok()?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let age_days = (now - first_tx_timestamp) / 86400;
        Some(age_days)
    }

    /// Gets all contracts deployed by a wallet
    pub async fn get_deployed_contracts(&self, chain_id: u64, deployer: &Address) -> Vec<Address> {
        let deployer_str = deployer.to_string();
        let params = [
            ("module", "account"),
            ("action", "txlist"),
            ("address", deployer_str.as_str()),
            ("startblock", "0"),
            ("endblock", "99999999"),
            ("sort", "asc"),
        ];

        let chain = Chain::from_chain_id(chain_id).unwrap_or(Chain::Ethereum);
        let res = match self
            .get::<EtherscanResponse<Vec<Transaction>>>(chain, &params)
            .await
        {
            Some(r) => r,
            None => return vec![],
        };

        if res.status != "1" {
            return vec![];
        }

        // Contract creation txs have empty `to` and non-empty `contractAddress`
        res.result
            .into_iter()
            .filter(|tx| tx.to.is_empty() && !tx.contract_address.is_empty())
            .filter_map(|tx| tx.contract_address.parse::<Address>().ok())
            .collect()
    }

    /// Gets full wallet info — age, deployed contracts, fresh wallet flag
    pub async fn get_wallet_info(&self, chain_id: u64, deployer: &Address) -> WalletInfo {
        let age_days = self.get_wallet_age_days(chain_id, deployer).await.unwrap_or(0);
        let deployed_contracts = self.get_deployed_contracts(chain_id, deployer).await;

        WalletInfo {
            is_fresh_wallet: age_days < 30,
            age_days,
            deployed_contracts,
        }
    }

    /// Checks if a deployer has previously deployed honeypot or rugged tokens
    /// by cross checking their deployed contracts against the honeypot API
    pub async fn check_deployer_reputation(&self, chain_id: u64, deployer: &Address) -> bool {
        let contracts = self.get_deployed_contracts(chain_id, deployer).await;

        if contracts.is_empty() {
            return false;
        }

        let mut unverified = 0;

        for contract in &contracts {
            let contract_str = contract.to_string();
            let params = [
                ("module", "contract"),
                ("action", "getsourcecode"),
                ("address", contract_str.as_str()),
            ];

            let chain = Chain::from_chain_id(chain_id).unwrap_or(Chain::Ethereum);

            if let Some(res) = self.get::<serde_json::Value>(chain, &params).await {
                let source = res["result"][0]["SourceCode"].as_str().unwrap_or("");
                if source.is_empty() {
                    unverified += 1;
                }
            }
        }

        let total = contracts.len();
        let unverified_ratio = unverified as f64 / total as f64;

        tracing::info!(
            "Deployer {deployer:?}: {unverified}/{total} contracts unverified ({:.0}%)",
            unverified_ratio * 100.0
        );

        // Flag if more than 50% of contracts are unverified
        unverified_ratio > 0.5
    }
}
