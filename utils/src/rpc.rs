use corebc_addressbook::Network;
use corebc_blockindex::Client;

/// Returns _mainnet_ rpc endpoint in inline
pub fn http_rpc_endpoint() -> String {
    rpc_endpoint(Network::Mainnet)
}

pub fn rpc_endpoint(network: Network) -> String {
    match network {
        Network::Mainnet => String::from("http://127.0.0.1:8565/"),
        Network::Devin => String::from("https://xcbapi.corecoin.cc/"),
        _ => panic!("Invalid Network. Only devin and mainnet are availible"),
    }
}

/// Returns endpoint that has access to archive state
pub fn next_http_archive_rpc_endpoint() -> String {
    // TODO:error2215 add a blockindex api url that has access to archive state
    let client = Client::new(Network::Mainnet).unwrap();
    client.blockindex_api_url().as_str().to_string()
}

pub fn next_http_rpc_endpoint() -> String {
    todo!("Here we will need a list of publicly avalible rpc endpoints");
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
