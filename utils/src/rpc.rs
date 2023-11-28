use corebc_addressbook::Network;

/// Returns _mainnet_ rpc endpoint in inline
pub fn http_rpc_endpoint() -> String {
    rpc_endpoint(Network::Mainnet)
}

pub fn rpc_endpoint(network: Network) -> String {
    next_http_rpc_endpoint(network)
}

/// Returns endpoint that has access to archive state
pub fn next_http_archive_rpc_endpoint(network: Network) -> String {
    next_http_rpc_endpoint(network)
}

pub fn next_http_rpc_endpoint(network: Network) -> String {
    match network {
        Network::Mainnet => String::from("https://xcbapi.coreblockchain.net/"),
        Network::Devin => String::from("https://xcbapi.corecoin.cc/"),
        _ => panic!("Invalid Network. Only devin and mainnet are availible"),
    }
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
