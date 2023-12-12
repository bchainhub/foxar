use corebc::{
    addressbook::contract,
    types::{Address, Network},
};

/// Returns a set of various contract addresses
pub fn contract_addresses(chain: Network) -> Vec<Address> {
    vec![
        contract("dai").unwrap().address(chain).unwrap(),
        contract("usdc").unwrap().address(chain).unwrap(),
        contract("weth").unwrap().address(chain).unwrap(),
        contract("uniswapV3Factory").unwrap().address(chain).unwrap(),
        contract("uniswapV3SwapRouter02").unwrap().address(chain).unwrap(),
    ]
}
