use clap::builder::{PossibleValuesParser, TypedValueParser};
use corebc::types::Network as NamedNetwork;
use foundry_config::Network;
use std::ffi::OsStr;

/// Custom Clap value parser for [`Network`]s.
///
/// Displays all possible networks when an invalid network is provided.
#[derive(Clone, Debug)]
pub struct NetworkValueParser {
    pub inner: PossibleValuesParser,
}

impl Default for NetworkValueParser {
    fn default() -> Self {
        Self { inner: PossibleValuesParser::from(NamedNetwork::Mainnet) }
    }
}

impl TypedValueParser for NetworkValueParser {
    type Value = Network;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let s =
            value.to_str().ok_or_else(|| clap::Error::new(clap::error::ErrorKind::InvalidUtf8))?;
        if let Ok(id) = s.parse() {
            Ok(Network::Id(id))
        } else {
            // NamedNetwork::VARIANTS is a subset of all possible variants, since there are aliases:
            // mumbai instead of polygon-mumbai etc
            //
            // Parse first as NamedNetwork, if it fails parse with NamedNetwork::VARIANTS for displaying
            // the error to the user
            s.parse()
                .map_err(|_| self.inner.parse_ref(cmd, arg, value).unwrap_err())
                .map(Network::Named)
        }
    }
}
