use corebc::{
    addressbook::contract,
    types::{Address, Network},
};

/// Returns a set of various contract addresses
pub fn contract_addresses(chain: Network) -> Vec<Address> {
    vec![contract("ctn").unwrap().address(chain).unwrap()]
}
