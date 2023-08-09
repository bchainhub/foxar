use crate::U256;
use corebc_core::types::{Network as NamedNetwork, U64};
use eyre::Result;
use open_fastrlp::{Decodable, Encodable};
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};

/// Either a named or network id or the actual id value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum Network {
    /// Contains a known network
    #[serde(serialize_with = "super::from_str_lowercase::serialize")]
    Named(NamedNetwork),
    /// Contains the id of a network
    Id(u64),
}

impl Network {
    /// The id of the network.
    pub const fn id(&self) -> u64 {
        match self {
            Network::Named(network) => *network as u64,
            Network::Id(id) => *id,
        }
    }

    /// Returns the wrapped named network or tries converting the ID into one.
    pub fn named(&self) -> Result<NamedNetwork> {
        match self {
            Self::Named(network) => Ok(*network),
            Self::Id(id) => {
                NamedNetwork::try_from(*id).map_err(|_| eyre::eyre!("Unsupported network: {id}"))
            }
        }
    }

    //TODO:error2215 remove 'legacy' of a network
    /// Helper function for checking if a networkid corresponds to a legacy networkid
    /// without eip1559
    pub fn is_legacy(&self) -> bool {
        self.named().map_or(false, |c| c.is_legacy())
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Network::Named(network) => network.fmt(f),
            Network::Id(id) => {
                if let Ok(network) = NamedNetwork::try_from(*id) {
                    network.fmt(f)
                } else {
                    id.fmt(f)
                }
            }
        }
    }
}

impl From<NamedNetwork> for Network {
    fn from(id: NamedNetwork) -> Self {
        Network::Named(id)
    }
}

impl From<u64> for Network {
    fn from(id: u64) -> Self {
        NamedNetwork::try_from(id).map(Network::Named).unwrap_or_else(|_| Network::Id(id))
    }
}

impl From<U256> for Network {
    fn from(id: U256) -> Self {
        id.as_u64().into()
    }
}

impl From<Network> for u64 {
    fn from(c: Network) -> Self {
        match c {
            Network::Named(c) => c as u64,
            Network::Id(id) => id,
        }
    }
}

impl From<Network> for U64 {
    fn from(c: Network) -> Self {
        u64::from(c).into()
    }
}

impl From<Network> for U256 {
    fn from(c: Network) -> Self {
        u64::from(c).into()
    }
}

impl TryFrom<Network> for NamedNetwork {
    type Error = <NamedNetwork as TryFrom<u64>>::Error;

    fn try_from(network: Network) -> Result<Self, Self::Error> {
        match network {
            Network::Named(network) => Ok(network),
            Network::Id(id) => id.try_into(),
        }
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(network) = NamedNetwork::from_str(s) {
            Ok(Network::Named(network))
        } else {
            s.parse::<u64>()
                .map(Network::Id)
                .map_err(|_| format!("Expected known network or integer, found: {s}"))
        }
    }
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum NetworkId {
            Named(String),
            Id(u64),
        }

        match NetworkId::deserialize(deserializer)? {
            NetworkId::Named(s) => {
                s.to_lowercase().parse().map(Network::Named).map_err(serde::de::Error::custom)
            }
            NetworkId::Id(id) => {
                Ok(NamedNetwork::try_from(id).map(Network::Named).unwrap_or_else(|_| Network::Id(id)))
            }
        }
    }
}

impl Encodable for Network {
    fn length(&self) -> usize {
        match self {
            Self::Named(network) => u64::from(*network).length(),
            Self::Id(id) => id.length(),
        }
    }
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        match self {
            Self::Named(network) => u64::from(*network).encode(out),
            Self::Id(id) => id.encode(out),
        }
    }
}

impl Decodable for Network {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        Ok(u64::decode(buf)?.into())
    }
}

impl Default for Network {
    fn default() -> Self {
        NamedNetwork::Mainnet.into()
    }
}
