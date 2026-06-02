use alloy::primitives::{Address, B256, keccak256};

// pub fn get_univ2_pair_address(token_a: &Address, token_b: &Address) -> Address {
//     let (token0, token1) = if token_a < token_b {
//         (token_a, token_b)
//     } else {
//         (token_b, token_a)
//     };

//     println!("token0, {token0}, token1: {token1}");

//     // Uniswap V2 Pair Address Calculation
//     let factory: Address = "0x5C69bEe701ef814a2B6a3EcC1B0CEB0Ce2383b98"
//         .parse()
//         .unwrap();

//     let init_code_hash: B256 =
//     "0x96e8ac4277198ff8ba78caa5194fbbd3f0f89f5d1d6a8462fd5b0fce0c27a3a0"
//         .parse()
//         .unwrap();

//     let salt = keccak256(&[token0.as_slice(), token1.as_slice()].concat());

    // let pair = keccak256(
    //     &[
    //         &[0xff],
    //         factory.as_slice(),
    //         salt.as_slice(),
    //         init_code_hash.as_slice(),
    //     ]
    //     .concat(),
    // );

    // Address::from_slice(&pair[12..])

//     factory.create2(salt, init_code_hash)
// }

use hex::decode;

use crate::utils::constant::UNISWAP_FACTORY;

pub fn get_univ2_pair_address(token_a: &Address, token_b: &Address) -> Address {
    let (token0, token1) = if token_a < token_b { (token_a, token_b) } else { (token_b, token_a) };

    let mut salt_data = Vec::new();
    salt_data.extend_from_slice(token0.as_slice());
    salt_data.extend_from_slice(token1.as_slice());
    let salt: B256 = keccak256(&salt_data);

    // Init Code Hash (Standard Ethereum Uniswap V2 value)
    let init_code_hash_hex = "96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f";
    let init_code_hash = B256::from_slice(&decode(init_code_hash_hex).unwrap());

    // ABI Encode Packed the CREATE2 parameters
    let mut encoded = Vec::new();
    encoded.push(0xff); // 0xff prefix
    encoded.extend_from_slice(UNISWAP_FACTORY.as_slice());
    encoded.extend_from_slice(salt.as_slice());
    encoded.extend_from_slice(init_code_hash.as_slice());

    // Compute final pair address
    let hash = keccak256(&encoded);
    let mut pair_address_bytes = [0u8; 20];
    pair_address_bytes.copy_from_slice(&hash[12..32]); // Address is the last 20 bytes of the hash
    let pair_address = Address::from(pair_address_bytes);

    pair_address
}


pub fn address_match(token_a: Address, token_b: Address) -> bool {
    token_a.to_checksum(None).to_lowercase() == token_b.to_checksum(None).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_univ2_weth_usdc_pair() {
        let weth: Address = "0xC02aaA39b223FE8D0A0E5C4F27eAD9083C756Cc2"
            .parse()
            .unwrap();

        let usdc: Address = "0xA0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
            .parse()
            .unwrap();

        let expected: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc"
            .parse()
            .unwrap();

        let calculated = get_univ2_pair_address(&weth, &usdc);

        assert_eq!(calculated, expected);
    }
}