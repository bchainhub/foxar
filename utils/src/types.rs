use corebc_core::types::{H256, U256};
use revm::primitives::{ruint::Uint, B256};

pub trait ToRuint {
    type To;

    fn to_ruint(self) -> Self::To;
}

impl ToRuint for U256 {
    type To = Uint<256, 4>;
    #[inline(always)]
    fn to_ruint(self) -> Self::To {
        let mut buffer = [0u8; 32];
        self.to_little_endian(buffer.as_mut_slice());
        Uint::<256, 4>::from_le_bytes(buffer)
    }
}

pub trait ToEthersH256 {
    type To;

    fn to_ethers_h256(self) -> Self::To;
}

impl ToEthersH256 for B256 {
    type To = H256;
    #[inline(always)]
    fn to_ethers_h256(self) -> Self::To {
        H256(self.0)
    }
}

pub trait ToEthersU256 {
    type To;

    fn to_ethers_u256(self) -> Self::To;
}

impl ToEthersU256 for Uint<256, 4> {
    type To = U256;
    #[inline(always)]
    fn to_ethers_u256(self) -> Self::To {
        U256::from_little_endian(&self.as_le_bytes())
    }
}
