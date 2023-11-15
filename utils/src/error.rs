//! error handling and support

use corebc_core::{abi::AbiEncode, types::Bytes};
use std::fmt::Display;

/// Solidity revert prefix.
///
/// `sha3("Error(string)")[..4] == 0x4e401cbe`
pub const REVERT_PREFIX: [u8; 4] = [78, 64, 28, 190];

/// Custom Cheatcode error prefix.
///
/// `sha3("CheatCodeError")[..4] == 0x07b9c7f6`
pub const ERROR_PREFIX: [u8; 4] = [7, 185, 199, 246];

/// An extension trait for `std::error::Error` that can abi-encode itself
pub trait SolError: std::error::Error {
    /// Returns the abi-encoded custom error
    ///
    /// Same as `encode_string` but prefixed with `ERROR_PREFIX`
    fn encode_error(&self) -> Bytes {
        encode_error(self)
    }

    /// Returns the error as abi-encoded String
    ///
    /// See also [`AbiEncode`](corebc::abi::AbiEncode)
    fn encode_string(&self) -> Bytes {
        self.to_string().encode().into()
    }
}

/// Encodes the given messages as solidity custom error
pub fn encode_error(reason: impl Display) -> Bytes {
    [ERROR_PREFIX.as_slice(), reason.to_string().encode().as_slice()].concat().into()
}
