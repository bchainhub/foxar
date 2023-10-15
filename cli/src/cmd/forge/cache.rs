//! cache command

use crate::cmd::Cmd;
use cache::Cache;
use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Arg, Command, Parser, Subcommand,
};
use corebc::prelude::Network;
use eyre::Result;
use foundry_config::{cache, Config, Network as FoundryConfigNetwork};
use std::{ffi::OsStr, str::FromStr};
use strum::VariantNames;

/// CLI arguments for `forge cache`.
#[derive(Debug, Parser)]
pub struct CacheArgs {
    #[clap(subcommand)]
    pub sub: CacheSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CacheSubcommands {
    /// Cleans cached data from the global foundry directory.
    Clean(CleanArgs),

    /// Shows cached data from the global foundry directory.
    Ls(LsArgs),
}

/// CLI arguments for `forge clean`.
#[derive(Debug, Parser)]
#[clap(group = clap::ArgGroup::new("etherscan-blocks").multiple(false))]
pub struct CleanArgs {
    /// The networks to clean the cache for.
    ///
    /// Can also be "all" to clean all networks.
    #[clap(
        env = "NETWORK",
        default_value = "all",
        value_parser = NetworkOrAllValueParser::default(),
    )]
    networks: Vec<NetworkOrAll>,

    /// The blocks to clean the cache for.
    #[clap(
        short,
        long,
        num_args(1..),
        use_value_delimiter(true),
        value_delimiter(','),
        group = "etherscan-blocks"
    )]
    blocks: Vec<u64>,

    /// Whether to clean the Etherscan cache.
    #[clap(long, group = "etherscan-blocks")]
    etherscan: bool,
}

impl Cmd for CleanArgs {
    type Output = ();

    fn run(self) -> Result<Self::Output> {
        let CleanArgs { networks, blocks, etherscan } = self;

        for network_or_all in networks {
            match network_or_all {
                NetworkOrAll::Network(network) => clean_network_cache(network, blocks.to_vec(), etherscan)?,
                NetworkOrAll::All => {
                    if etherscan {
                        Config::clean_foundry_etherscan_cache()?;
                    } else {
                        Config::clean_foundry_cache()?
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct LsArgs {
    /// The networks to list the cache for.
    ///
    /// Can also be "all" to list all networks.
    #[clap(
        env = "NETWORK",
        default_value = "all",
        value_parser = NetworkOrAllValueParser::default(),
    )]
    networks: Vec<NetworkOrAll>,
}

impl Cmd for LsArgs {
    type Output = ();

    fn run(self) -> Result<Self::Output> {
        let LsArgs { networks } = self;
        let mut cache = Cache::default();
        for network_or_all in networks {
            match network_or_all {
                NetworkOrAll::Network(network) => {
                    cache.networks.push(Config::list_foundry_network_cache(network.into())?)
                }
                NetworkOrAll::All => cache = Config::list_foundry_cache()?,
            }
        }
        print!("{cache}");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum NetworkOrAll {
    Network(Network),
    All,
}

impl FromStr for NetworkOrAll {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(network) = ethers::prelude::Network::from_str(s) {
            Ok(NetworkOrAll::Network(network))
        } else if s == "all" {
            Ok(NetworkOrAll::All)
        } else {
            Err(format!("Expected known network or all, found: {s}"))
        }
    }
}

fn clean_network_cache(
    network: impl Into<FoundryConfigNetwork>,
    blocks: Vec<u64>,
    etherscan: bool,
) -> Result<()> {
    let network = network.into();
    if blocks.is_empty() {
        Config::clean_foundry_etherscan_network_cache(network)?;
        if etherscan {
            return Ok(())
        }
        Config::clean_foundry_network_cache(network)?;
    } else {
        for block in blocks {
            Config::clean_foundry_block_cache(network, block)?;
        }
    }
    Ok(())
}

/// The value parser for `ChainOrAll`
#[derive(Clone, Debug)]
pub struct NetworkOrAllValueParser {
    inner: PossibleValuesParser,
}

impl Default for NetworkOrAllValueParser {
    fn default() -> Self {
        NetworkOrAllValueParser { inner: possible_chains() }
    }
}

impl TypedValueParser for NetworkOrAllValueParser {
    type Value = NetworkOrAll;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        self.inner.parse_ref(cmd, arg, value)?.parse::<NetworkOrAll>().map_err(|_| {
            clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                "chain argument did not match any possible chain variant",
            )
        })
    }
}

fn possible_chains() -> PossibleValuesParser {
    Some([&"all", Network::Mainnet, Network::Devin]).into_iter().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_cache_ls() {
        let args: CacheArgs = CacheArgs::parse_from(["cache", "ls"]);
        assert!(matches!(args.sub, CacheSubcommands::Ls(_)));
    }
}
