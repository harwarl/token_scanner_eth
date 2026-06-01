use alloy::{
    dyn_abi::abi::token,
    primitives::{Address, B256, keccak256},
};

pub fn get_univ2_pair_address(token_a: &Address, token_b: &Address) -> Address {
    let (token0, token1) = if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    };

    // Uniswap V2 Pair Address Calculation
    let factory: Address = "0x5C69bEe701ef814a2B6a3EcC1B0CEB0Ce2383b98"
        .parse()
        .unwrap();

    let init_code_hash: B256 = "0x96e8ac4277198ff8ba78caa5194fbbd3f0f89f5d1d6a8462fd5b0fce0c27a3a0"
        .parse()
        .unwrap();

    let salt = keccak256(&[token0.as_slice(), token1.as_slice()].concat());

    let pair = keccak256(
        &[
            &[0xff],
            factory.as_slice(),
            salt.as_slice(),
            init_code_hash.as_slice(),
        ]
        .concat(),
    );

    Address::from_slice(&pair[12..])
}

pub fn address_match(token_a: Address, token_b: Address) -> bool {
    token_a.to_checksum(None).to_lowercase() == token_b.to_checksum(None).to_lowercase()
}
