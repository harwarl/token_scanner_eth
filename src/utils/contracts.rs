use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "abi/IERC_20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV2Pair,
    "abi/IUniswapV2Pair.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapV2Factory,
    "abi/IUniswapV2Factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IUniswapPoolManager,
    "abi/IUniswap_V4.json"
);
