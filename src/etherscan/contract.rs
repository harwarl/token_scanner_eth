use alloy::primitives::Address;

use crate::{
    etherscan::client::EtherscanClient,
    types::{ContractCreation, ContractInfo, EtherscanResponse, SourceCode},
};

impl EtherscanClient {
    /// Gets contract creation details including deployer and tx hash
    pub async fn get_contract_creation(&self, token: &Address) -> Option<ContractCreation> {
        let token_str = token.to_string();
        let params = [
            ("module", "contract"),
            ("action", "getcontractcreation"),
            ("contractaddresses", token_str.as_str()),
        ];

        let res = self
            .get::<EtherscanResponse<Vec<ContractCreation>>>(&params)
            .await?;

        if res.status != "1" {
            return None;
        }

        res.result.into_iter().next()
    }

    /// Checks if a contract is verified and returns source code info
    pub async fn get_source_code(&self, token: &Address) -> Option<SourceCode> {
        let token_str = token.to_string();
        let params = [
            ("module", "contract"),
            ("action", "getsourcecode"),
            ("address", token_str.as_str()),
        ];

        let res = self
            .get::<EtherscanResponse<Vec<SourceCode>>>(&params)
            .await?;

        if res.status != "1" {
            return None;
        }

        res.result.into_iter().next()
    }

    /// Returns combined contract info — verified status, deployer, tx hash
    pub async fn get_contract_info(&self, token: &Address) -> ContractInfo {
        let source = self.get_source_code(token).await;
        let creation = self.get_contract_creation(token).await;

        let verified = source
            .as_ref()
            .map(|s| !s.source_code.is_empty())
            .unwrap_or(false);

        let contract_name = source
            .map(|s| s.contract_name)
            .unwrap_or_else(|| "Unknown".to_string());

        let deployer = creation
            .as_ref()
            .and_then(|c| c.contract_creator.parse::<Address>().ok())
            .unwrap_or(Address::ZERO);

        let tx_hash = creation
            .map(|c| c.tx_hash)
            .unwrap_or_else(|| "Unknown".to_string());

        ContractInfo {
            verified,
            contract_name,
            deployer,
            tx_hash,
        }
    }

    // Returns the number of token holders
    // pub async fn get_holder_count(&self, token: &Address) -> u64 {
    //     let token_str = token.to_string();
    //     let params = [
    //         ("module", "token"),
    //         ("action", "tokeninfo"),
    //         ("contractaddress", token_str.as_str()),
    //     ];

    //     let res = self
    //         .get::<EtherscanResponse<Vec<serde_json::Value>>>(&params)
    //         .await;

    //     println!("Res: {res:?}");
    //     res.and_then(|r| {
    //         if r.status != "1" {
    //             return None;
    //         }
    //         r.result.into_iter().next()?["holdersCount"]
    //             .as_str()
    //             .and_then(|s| s.parse().ok())
    //     })
    //     .unwrap_or(0)
    // }
}
