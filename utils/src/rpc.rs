use corebc_addressbook::Network;
use corebc_blockindex::Client;

/// Returns _mainnet_ rpc endpoint in inline
pub fn http_rpc_endpoint() -> String {
    rpc_endpoint(Network::Mainnet)
}

pub fn rpc_endpoint(network: Network) -> String {
    let client = Client::new(network).unwrap();
    client.blockindex_api_url().as_str().to_string()
}

/// Returns endpoint that has access to archive state
pub fn next_http_archive_rpc_endpoint() -> String {
    // TODO:error2215 add a blockindex api url that has access to archive state
    let client = Client::new(Network::Mainnet).unwrap();
    client.blockindex_api_url().as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    #[ignore]
    fn can_rotate_unique() {
        let mut keys = HashSet::new();
        for _ in 0..100 {
            keys.insert(http_rpc_endpoint());
        }
        assert_eq!(keys.len(), 1);
    }
}
