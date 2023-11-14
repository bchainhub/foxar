use corebc_core::{
    types::{Address, Bloom, Bytes, H256, U256},
    utils::{
        rlp,
        rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream},
    },
};
use revm::primitives::B256;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "fastrlp", derive(open_fastrlp::RlpEncodable, open_fastrlp::RlpDecodable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Bytes,
}

impl From<revm::primitives::Log> for Log {
    fn from(log: revm::primitives::Log) -> Self {
        let revm::primitives::Log { address, topics, data } = log;
        Log {
            address: address.into(),
            topics: topics.into_iter().map(|num| H256(num.0)).collect(),
            data: data.into(),
        }
    }
}

impl From<Log> for revm::primitives::Log {
    fn from(log: Log) -> Self {
        let Log { address, topics, data } = log;
        revm::primitives::Log {
            address: address.into(),
            topics: topics.into_iter().map(|num| B256(num.0)).collect(),
            data: data.0,
        }
    }
}

impl Encodable for Log {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(3);
        stream.append(&self.address);
        stream.append_list(&self.topics);
        stream.append(&self.data.as_ref());
    }
}

impl Decodable for Log {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let result = Log {
            address: rlp.val_at(0)?,
            topics: rlp.list_at(1)?,
            data: rlp.val_at::<Vec<u8>>(2)?.into(),
        };
        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "fastrlp", derive(open_fastrlp::RlpEncodable, open_fastrlp::RlpDecodable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EIP658Receipt {
    pub status_code: u8,
    pub gas_used: U256,
    pub logs_bloom: Bloom,
    pub logs: Vec<Log>,
}

impl Encodable for EIP658Receipt {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(4);
        stream.append(&self.status_code);
        stream.append(&self.gas_used);
        stream.append(&self.logs_bloom);
        stream.append_list(&self.logs);
    }
}

impl Decodable for EIP658Receipt {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let result = EIP658Receipt {
            status_code: rlp.val_at(0)?,
            gas_used: rlp.val_at(1)?,
            logs_bloom: rlp.val_at(2)?,
            logs: rlp.list_at(3)?,
        };
        Ok(result)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypedReceipt {
    /// Legacy receipt
    Legacy(EIP658Receipt),
}

// == impl TypedReceipt ==

impl TypedReceipt {
    /// Returns the gas used by the transactions
    pub fn gas_used(&self) -> U256 {
        match self {
            TypedReceipt::Legacy(r) => r.gas_used,
        }
    }

    /// Returns the gas used by the transactions
    pub fn logs_bloom(&self) -> &Bloom {
        match self {
            TypedReceipt::Legacy(r) => &r.logs_bloom,
        }
    }
}

impl Encodable for TypedReceipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            TypedReceipt::Legacy(r) => r.rlp_append(s),
        }
    }
}

impl Decodable for TypedReceipt {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_list() {
            return Ok(TypedReceipt::Legacy(Decodable::decode(rlp)?));
        }

        Err(DecoderError::Custom("unknown receipt type"))
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for TypedReceipt {
    fn length(&self) -> usize {
        match self {
            TypedReceipt::Legacy(r) => r.length(),
        }
    }
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        match self {
            TypedReceipt::Legacy(r) => r.encode(out),
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for TypedReceipt {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        <EIP658Receipt as open_fastrlp::Decodable>::decode(buf).map(TypedReceipt::Legacy)
    }
}

impl From<TypedReceipt> for EIP658Receipt {
    fn from(v3: TypedReceipt) -> Self {
        match v3 {
            TypedReceipt::Legacy(receipt) => receipt,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "fastrlp")]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn encode_legacy_receipt() {
        use std::str::FromStr;

        use corebc_core::{
            types::{Bytes, H176, H256},
            utils::hex,
        };
        use open_fastrlp::Encodable;

        use crate::eth::receipt::{EIP658Receipt, Log, TypedReceipt};

        let expected = hex::decode("f901060180b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0").unwrap();

        let mut data = vec![];
        let receipt = TypedReceipt::Legacy(EIP658Receipt {
            logs_bloom: [0; 256].into(),
            gas_used: 0.into(),
            logs: Vec::new(),
            status_code: 1,
        });
        receipt.encode(&mut data);

        // check that the rlp length equals the length of the expected rlp
        assert_eq!(receipt.length(), expected.len());
        assert_eq!(data, expected);
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn decode_legacy_receipt() {
        use std::str::FromStr;

        use corebc_core::{
            types::{Bytes, H176, H256},
            utils::hex,
        };
        use open_fastrlp::Decodable;

        use crate::eth::receipt::{EIP658Receipt, Log, TypedReceipt};

        let data = hex::decode("f901060180b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0").unwrap();

        let expected = TypedReceipt::Legacy(EIP658Receipt {
            logs_bloom: [0; 256].into(),
            gas_used: 0.into(),
            logs: Vec::new(),
            status_code: 1,
        });

        let receipt = TypedReceipt::decode(&mut &data[..]).unwrap();
        assert_eq!(receipt, expected);
    }
}
