use std::collections::HashSet;

use alloy::primitives::{Address, address};

use crate::etherscan::client::EtherscanClient;

pub struct BadActorDB {
    pub deployers: HashSet<Address>,
    pub funding_sources: HashSet<Address>,
}

impl BadActorDB {
    pub fn new() -> Self {
        let mut deployers = HashSet::new();
        let mut funding_sources = HashSet::new();

        // seed known bad actors
        deployers.insert(address!("0x99BE975616016A55F4A164d743370b6bf98c1b38"));
        funding_sources.insert(address!("0xc43f317Ed4d81cbbFe2c9C98b4cC6F303519f078"));

        Self {
            deployers,
            funding_sources,
        }
    }

    pub async fn is_bad_actor(&mut self, etherscan: &EtherscanClient, deployer: &Address) -> bool {
        // check deployer
        if self.deployers.contains(deployer) {
            return true;
        }

        // get the funder
        if let Some(funder) = etherscan.get_funding_source(deployer).await {
            if self.funding_sources.contains(&funder) {
                self.deployers.insert(funder);
                return true;
            }
        }
        false
    }
}
