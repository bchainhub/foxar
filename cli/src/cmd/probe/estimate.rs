// probe estimate subcommands
use crate::{
    opts::{EtherscanOpts, RpcOpts},
    utils::{self, parse_ether_value},
};
use clap::Parser;
use corebc::types::{NameOrAddress, U256};
use eyre::Result;
use foxar_config::{figment::Figment, Config};
use probe::{Cast, TxBuilder};
use std::str::FromStr;

/// CLI arguments for `probe estimate`.
#[derive(Debug, Parser)]
pub struct EstimateArgs {
    /// The destination of the transaction.
    #[clap(value_parser = NameOrAddress::from_str)]
    to: Option<NameOrAddress>,

    /// The signature of the function to call.
    sig: Option<String>,

    /// The arguments of the function to call.
    args: Vec<String>,

    /// The sender account.
    #[clap(
        short,
        long,
        value_parser = NameOrAddress::from_str,
        default_value = "0xce566dadee521bea601692312454a655a0f49051ddc9", 
        env = "ETH_FROM",
    )]
    from: NameOrAddress,

    /// Ether to send in the transaction.
    ///
    /// Either specified in wei, or as a string with a unit type:
    ///
    /// Examples: 1ether, 10gwei, 0.01ether
    #[clap(long, value_parser = parse_ether_value)]
    value: Option<U256>,

    #[clap(flatten)]
    rpc: RpcOpts,

    #[clap(flatten)]
    etherscan: EtherscanOpts,

    #[clap(subcommand)]
    command: Option<EstimateSubcommands>,
}

#[derive(Debug, Parser)]
pub enum EstimateSubcommands {
    /// Estimate gas cost to deploy a smart contract
    #[clap(name = "--create")]
    Create {
        /// The bytecode of contract
        code: String,

        /// The signature of the constructor
        sig: Option<String>,

        /// Constructor arguments
        args: Vec<String>,

        /// Ether to send in the transaction
        ///
        /// Either specified in wei, or as a string with a unit type:
        ///
        /// Examples: 1ether, 10gwei, 0.01ether
        #[clap(long, value_parser = parse_ether_value)]
        value: Option<U256>,
    },
}

impl EstimateArgs {
    pub async fn run(self) -> Result<()> {
        let EstimateArgs { from, to, sig, args, value, rpc, etherscan, command } = self;

        let figment = Figment::from(Config::figment()).merge(etherscan).merge(rpc);
        let config = Config::from_provider(figment);

        let provider = utils::get_provider(&config)?;
        let network = utils::get_network(config.network_id, &provider).await?;
        // let api_key = config.get_etherscan_api_key(Some(network));

        let mut builder = TxBuilder::new(&provider, from, to, network).await?;
        // builder.etherscan_api_key(api_key);

        match command {
            Some(EstimateSubcommands::Create { code, sig, args, value }) => {
                builder.value(value);

                let mut data = hex::decode(code.strip_prefix("0x").unwrap_or(&code))?;

                if let Some(s) = sig {
                    let (mut sigdata, _func) = builder.create_args(&s, args).await?;
                    data.append(&mut sigdata);
                }

                builder.set_data(data);
            }
            _ => {
                let sig = sig.ok_or_else(|| eyre::eyre!("Function signature must be provided."))?;
                builder.value(value).set_args(sig.as_str(), args).await?;
            }
        };

        let builder_output = builder.peek();
        let gas = Cast::new(&provider).estimate(builder_output).await?;
        println!("{gas}");
        Ok(())
    }
}
