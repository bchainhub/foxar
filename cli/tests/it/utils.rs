//! Various helper functions

use corebc::prelude::{Address, LocalWallet, Network, Signer};

/// Returns the current millis since unix epoch.
///
/// This way we generate unique contracts so, etherscan will always have to verify them
pub fn millis_since_epoch() -> u128 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|err| panic!("Current time {now:?} is invalid: {err:?}"))
        .as_millis()
}

pub fn network_private_key(chain: &str) -> Option<String> {
    let key = format!("{}_PRIVATE_KEY", chain.to_uppercase().replace('-', "_"));
    std::env::var(key).or_else(|_| std::env::var("TEST_PRIVATE_KEY")).ok()
}

/// Represents external input required for executing verification requests
pub struct EnvExternalities {
    pub chain: Network,
    pub rpc: String,
    pub pk: String,
    pub verifier: String,
}

#[allow(dead_code)]
impl EnvExternalities {
    pub fn address(&self) -> Option<Address> {
        let pk: LocalWallet = self.pk.parse().ok()?;
        Some(pk.address())
    }

    pub fn devin() -> Option<Self> {
        Some(Self {
            chain: Network::Devin,
            rpc: "".to_string(),
            pk: network_private_key("devin")?,
            verifier: "etherscan".to_string(),
        })
    }

    /// Returns the arguments required to deploy the contract
    pub fn create_args(&self) -> Vec<String> {
        vec![
            "--chain".to_string(),
            self.chain.to_string(),
            "--rpc-url".to_string(),
            self.rpc.clone(),
            "--private-key".to_string(),
            self.pk.clone(),
        ]
    }
}

/// Parses the address the contract was deployed to
pub fn parse_deployed_address(out: &str) -> Option<String> {
    for line in out.lines() {
        if line.starts_with("Deployed to") {
            return Some(line.trim_start_matches("Deployed to: ").to_string())
        }
    }
    None
}

pub fn parse_verification_guid(out: &str) -> Option<String> {
    for line in out.lines() {
        if line.contains("GUID") {
            return Some(line.replace("GUID:", "").replace('`', "").trim().to_string())
        }
    }
    None
}
