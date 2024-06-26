//! probe create2 subcommand

use crate::cmd::Cmd;
use clap::Parser;
use corebc::{
    core::rand::thread_rng,
    types::{Address, Bytes, Network, H256, U256},
    utils::{get_create2_h160_address_from_hash, sha3, to_ican},
};
use eyre::{Result, WrapErr};
use rayon::prelude::*;
use regex::RegexSetBuilder;
use std::time::Instant;

/// CLI arguments for `probe create2`.
#[derive(Debug, Clone, Parser)]
pub struct Create2Args {
    /// Prefix for the contract address.
    #[clap(
        long,
        short,
        required_unless_present_any = &["ends_with", "matching"],
        value_name = "HEX"
    )]
    starts_with: Option<String>,

    /// Suffix for the contract address.
    #[clap(long, short, value_name = "HEX")]
    ends_with: Option<String>,

    /// Sequence that the address has to match.
    #[clap(long, short, value_name = "HEX")]
    matching: Option<String>,

    /// Case sensitive matching.
    #[clap(short, long)]
    case_sensitive: bool,

    /// Address of the contract deployer.
    #[clap(
        short,
        long,
        default_value = "ce8147e798c3a0d867f70f8785334da06c3418e18ba9",
        value_name = "ADDRESS"
    )]
    deployer: Address,

    /// Init code of the contract to be deployed.
    #[clap(short, long, value_name = "HEX", default_value = "")]
    init_code: String,

    /// Init code hash of the contract to be deployed.
    #[clap(alias = "ch", long, value_name = "HASH")]
    init_code_hash: Option<String>,

    /// Network to use for address prefix validation.
    #[clap(short, long)]
    pub network: Network,
}

#[allow(dead_code)]
pub struct Create2Output {
    pub address: Address,
    pub salt: U256,
}

impl Cmd for Create2Args {
    type Output = Create2Output;

    fn run(self) -> eyre::Result<Self::Output> {
        Create2Args::generate_address(self)
    }
}

impl Create2Args {
    fn generate_address(self) -> Result<Create2Output> {
        let Create2Args {
            starts_with,
            ends_with,
            matching,
            case_sensitive,
            deployer,
            init_code,
            init_code_hash,
            network,
        } = self;

        let mut regexs = vec![];

        if let Some(matches) = matching {
            if starts_with.is_some() || ends_with.is_some() {
                eyre::bail!("Either use --matching or --starts/ends-with");
            }

            let matches = matches.trim_start_matches("0x");

            if matches.len() != 40 {
                eyre::bail!("Please provide a 40 characters long sequence for matching");
            }

            hex::decode(matches.replace('X', "0")).wrap_err("invalid matching hex provided")?;
            // replacing X placeholders by . to match any character at these positions

            regexs.push(matches.replace('X', "."));
        }

        if let Some(prefix) = starts_with {
            let pad_width = prefix.len() + prefix.len() % 2;
            hex::decode(format!("{prefix:0>pad_width$}"))
                .wrap_err("invalid prefix hex provided")?;
            regexs.push(format!(r"^{prefix}"));
        }
        if let Some(suffix) = ends_with {
            let pad_width = suffix.len() + suffix.len() % 2;
            hex::decode(format!("{suffix:0>pad_width$}"))
                .wrap_err("invalid suffix hex provided")?;
            regexs.push(format!(r"{suffix}$"));
        }

        debug_assert!(
            regexs.iter().map(|p| p.len() - 1).sum::<usize>() <= 40,
            "vanity patterns length exceeded. cannot be more than 40 characters",
        );

        let regex = RegexSetBuilder::new(regexs).case_insensitive(!case_sensitive).build()?;

        let init_code_hash = if let Some(init_code_hash) = init_code_hash {
            let mut a: [u8; 32] = [0; 32];
            let init_code_hash = init_code_hash.strip_prefix("0x").unwrap_or(&init_code_hash);
            assert!(init_code_hash.len() == 64, "init code hash should be 32 bytes long"); // 32 bytes * 2
            a.copy_from_slice(&hex::decode(init_code_hash)?[..32]);
            a
        } else {
            let init_code = init_code.strip_prefix("0x").unwrap_or(&init_code).as_bytes();
            sha3(hex::decode(init_code)?)
        };

        println!("Starting to generate deterministic contract address...");
        let timer = Instant::now();
        let (salt, addr) = std::iter::repeat(())
            .par_bridge()
            .map(|_| {
                let salt = H256::random_using(&mut thread_rng());
                let salt = Bytes::from(salt.to_fixed_bytes());

                let h160 =
                    get_create2_h160_address_from_hash(deployer, salt.clone(), init_code_hash);

                (salt, h160)
            })
            .find_any(move |(_, addr)| {
                let addr = addr.to_string();
                regex.matches(addr.as_str()).into_iter().count() == regex.patterns().len()
            })
            .unwrap();

        let salt = U256::from(salt.to_vec().as_slice());
        let address = to_ican(&addr, &network);

        println!(
            "Successfully found contract address in {} seconds.\nAddress: {}\nSalt: {}",
            timer.elapsed().as_secs(),
            address,
            salt
        );

        Ok(Create2Output { address, salt })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use corebc::{abi::AbiEncode, types::H176, utils::get_create2_address};

    use super::*;

    const DEPLOYER: &str = "cb063edadf999cb7b8b3ebc71f5e97783176d289d640";

    #[test]
    fn basic_create2() {
        let args =
            Create2Args::parse_from(["foxar-cli", "--starts-with", "babe", "--network", "1"]);
        let create2_out = args.run().unwrap();
        let address = create2_out.address;
        let address = format!("{address:x}");

        assert!(address[4..].starts_with("babe"));
    }

    #[test]
    fn matches_pattern() {
        let args = Create2Args::parse_from([
            "foxar-cli",
            "--matching",
            "babeXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            "--network",
            "1",
        ]);
        let create2_out = args.run().unwrap();
        let address = create2_out.address;
        let address = format!("{address:x}");

        assert!(address[4..].starts_with("babe"));
    }

    #[test]
    fn create2_init_code() {
        let init_code = "00";
        let args = Create2Args::parse_from([
            "foxar-cli",
            "--starts-with",
            "babe",
            "--init-code",
            init_code,
            "--network",
            "1",
            "--deployer",
            "cb063edadf999cb7b8b3ebc71f5e97783176d289d640",
        ]);
        let create2_out = args.run().unwrap();
        let address = create2_out.address;
        let address_str = format!("{address:x}");
        let salt = create2_out.salt;
        let deployer = H176::from_str(DEPLOYER).unwrap();

        assert!(address_str[4..].starts_with("babe"));
        assert_eq!(address, verify_create2(deployer, salt, hex::decode(init_code).unwrap()));
    }

    #[test]
    fn create2_init_code_hash() {
        let init_code_hash = "bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a";
        let args = Create2Args::parse_from([
            "foxar-cli",
            "--starts-with",
            "babe",
            "--init-code-hash",
            init_code_hash,
            "--network",
            "1",
            "--deployer",
            "cb063edadf999cb7b8b3ebc71f5e97783176d289d640",
        ]);
        let create2_out = args.run().unwrap();
        let address = create2_out.address;
        let address_str = format!("{address:x}");
        let salt = create2_out.salt;
        let deployer = Address::from_str(DEPLOYER).unwrap();

        assert!(address_str[4..].starts_with("babe"));
        assert_eq!(
            address,
            verify_create2_hash(deployer, salt, hex::decode(init_code_hash).unwrap())
        );
    }

    #[test]
    fn verify_helpers() {
        // https://eips.ethereum.org/EIPS/eip-1014
        let eip_address =
            Address::from_str("cb52a55032de3186cea55fdef3fdb0dbd45b18bba964").unwrap();

        let deployer = Address::from_str("00000000000000000000000000000000000000000000").unwrap();
        let salt =
            U256::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        let init_code = hex::decode("00").unwrap();
        let address = verify_create2(deployer, salt, init_code);

        assert_eq!(address, eip_address);

        let init_code_hash =
            hex::decode("5d53469f20fef4f8eab52b88044ede69c77a6a68a60728609fc4a65ff531e7d0")
                .unwrap();
        let address = verify_create2_hash(deployer, salt, init_code_hash);

        assert_eq!(address, eip_address);
    }

    fn verify_create2(deployer: Address, salt: U256, init_code: Vec<u8>) -> Address {
        // let init_code_hash = sha3(init_code);
        get_create2_address(deployer, salt.encode(), init_code, corebc::types::Network::Mainnet)
    }

    fn verify_create2_hash(deployer: Address, salt: U256, init_code_hash: Vec<u8>) -> Address {
        let addr = get_create2_h160_address_from_hash(deployer, salt.encode(), init_code_hash);
        to_ican(&addr, &corebc::types::Network::Mainnet)
    }
}
