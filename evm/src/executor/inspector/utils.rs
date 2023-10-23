use corebc::{
    types::{Address, Network},
    utils::{get_contract_address, get_create2_address},
};
use revm::{
    interpreter::CreateInputs,
    primitives::{CreateScheme, SpecId},
};

use crate::utils::{b176_to_h176, ru256_to_u256};

/// Returns [InstructionResult::Continue] on an error, discarding the error.
///
/// Useful for inspectors that read state that might be invalid, but do not want to emit
/// appropriate errors themselves, instead opting to continue.
macro_rules! try_or_continue {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => return InstructionResult::Continue,
        }
    };
}

/// Get the address of a contract creation
pub fn get_create_address(call: &CreateInputs, nonce: u64, network: &Network) -> Address {
    match call.scheme {
        CreateScheme::Create => get_contract_address(b176_to_h176(call.caller), nonce, network),
        CreateScheme::Create2 { salt } => {
            let salt = ru256_to_u256(salt);
            let mut salt_bytes = [0u8; 32];
            salt.to_big_endian(&mut salt_bytes);
            get_create2_address(
                b176_to_h176(call.caller),
                salt_bytes,
                call.init_code.clone(),
                *network,
            )
        }
    }
}

/// Get the gas used, accounting for refunds
pub fn gas_used(_spec: SpecId, spent: u64, refunded: u64) -> u64 {
    let refund_quotient = 2;
    spent - (refunded).min(spent / refund_quotient)
}
