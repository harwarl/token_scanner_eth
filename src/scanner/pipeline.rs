use teloxide::Bot;

use crate::{etherscan::client::EtherscanClient, types::TokenInfo};

pub async fn run_pipeline(
    token_info: TokenInfo,
    bot: &Bot,
    etherscanClient: EtherscanClient
) {

}