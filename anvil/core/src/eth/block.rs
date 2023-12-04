use crate::eth::{receipt::TypedReceipt, transaction::TransactionInfo, trie};
use corebc_core::{
    types::{Address, Bloom, Bytes, H256, H64, U256},
    utils::{
        rlp,
        rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream},
        sha3,
    },
};

/// Container type that gathers all block data
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub block: Block,
    pub transactions: Vec<TransactionInfo>,
    pub receipts: Vec<TypedReceipt>,
}

// Type alias to optionally support impersonated transactions
#[cfg(not(feature = "impersonated-tx"))]
type Transaction = crate::eth::transaction::TypedTransaction;
#[cfg(feature = "impersonated-tx")]
type Transaction = crate::eth::transaction::MaybeImpersonatedTransaction;

/// An Ethereum block
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "fastrlp", derive(open_fastrlp::RlpEncodable, open_fastrlp::RlpDecodable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    pub header: Header,
    /// Note: this supports impersonated transactions
    pub transactions: Vec<Transaction>,
    pub ommers: Vec<Header>,
}

// == impl Block ==

impl Block {
    /// Creates a new block
    ///
    /// Note: if the `impersonate-tx` feature is enabled this  will also accept
    /// [MaybeImpersonatedTransaction]
    pub fn new<T>(
        partial_header: PartialHeader,
        transactions: impl IntoIterator<Item = T>,
        ommers: Vec<Header>,
    ) -> Self
    where
        T: Into<Transaction>,
    {
        let transactions: Vec<_> = transactions.into_iter().map(Into::into).collect();
        let ommers_hash = H256::from_slice(sha3(&rlp::encode_list(&ommers)[..]).as_slice());
        let transactions_root =
            trie::ordered_trie_root(transactions.iter().map(|r| rlp::encode(r).freeze()));

        Self {
            header: Header::new(partial_header, ommers_hash, transactions_root),
            transactions,
            ommers,
        }
    }
}

impl Encodable for Block {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.header);
        s.append_list(&self.transactions);
        s.append_list(&self.ommers);
    }
}

impl Decodable for Block {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Self { header: rlp.val_at(0)?, transactions: rlp.list_at(1)?, ommers: rlp.list_at(2)? })
    }
}

/// ethereum block header
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Header {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary: Address,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: U256,
    pub energy_limit: U256,
    pub energy_used: U256,
    pub timestamp: u64,
    pub extra_data: Bytes,
    pub nonce: H64,
}

// == impl Header ==

impl Header {
    pub fn new(partial_header: PartialHeader, ommers_hash: H256, transactions_root: H256) -> Self {
        Self {
            parent_hash: partial_header.parent_hash,
            ommers_hash,
            beneficiary: partial_header.beneficiary,
            state_root: partial_header.state_root,
            transactions_root,
            receipts_root: partial_header.receipts_root,
            logs_bloom: partial_header.logs_bloom,
            difficulty: partial_header.difficulty,
            number: partial_header.number,
            energy_limit: partial_header.energy_limit,
            energy_used: partial_header.energy_used,
            timestamp: partial_header.timestamp,
            extra_data: partial_header.extra_data,
            nonce: partial_header.nonce,
        }
    }

    pub fn hash(&self) -> H256 {
        H256::from_slice(sha3(&rlp::encode(self)).as_slice())
    }

    /// Returns the rlp length of the Header body, _not including_ trailing EIP155 fields or the
    /// rlp list header
    /// To get the length including the rlp list header, refer to the Encodable implementation.
    #[cfg(feature = "fastrlp")]
    pub(crate) fn header_payload_length(&self) -> usize {
        use open_fastrlp::Encodable;

        let mut length = 0;
        length += self.parent_hash.length();
        length += self.ommers_hash.length();
        length += self.beneficiary.length();
        length += self.state_root.length();
        length += self.transactions_root.length();
        length += self.receipts_root.length();
        length += self.logs_bloom.length();
        length += self.difficulty.length();
        length += self.number.length();
        length += self.energy_limit.length();
        length += self.energy_used.length();
        length += self.timestamp.length();
        length += self.extra_data.length();
        length += self.nonce.length();
        length
    }
}

impl rlp::Encodable for Header {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(14);
        s.append(&self.parent_hash);
        s.append(&self.ommers_hash);
        s.append(&self.beneficiary);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.logs_bloom);
        s.append(&self.difficulty);
        s.append(&self.number);
        s.append(&self.energy_limit);
        s.append(&self.energy_used);
        s.append(&self.timestamp);
        s.append(&self.extra_data.as_ref());
        s.append(&self.nonce);
    }
}

impl rlp::Decodable for Header {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let result = Header {
            parent_hash: rlp.val_at(0)?,
            ommers_hash: rlp.val_at(1)?,
            beneficiary: rlp.val_at(2)?,
            state_root: rlp.val_at(3)?,
            transactions_root: rlp.val_at(4)?,
            receipts_root: rlp.val_at(5)?,
            logs_bloom: rlp.val_at(6)?,
            difficulty: rlp.val_at(7)?,
            number: rlp.val_at(8)?,
            energy_limit: rlp.val_at(9)?,
            energy_used: rlp.val_at(10)?,
            timestamp: rlp.val_at(11)?,
            extra_data: rlp.val_at::<Vec<u8>>(12)?.into(),
            nonce: rlp.val_at(13)?,
        };
        Ok(result)
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for Header {
    fn length(&self) -> usize {
        // add each of the fields' rlp encoded lengths
        let mut length = 0;
        length += self.header_payload_length();
        length += open_fastrlp::length_of_length(length);

        length
    }

    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        let list_header =
            open_fastrlp::Header { list: true, payload_length: self.header_payload_length() };
        list_header.encode(out);
        self.parent_hash.encode(out);
        self.ommers_hash.encode(out);
        self.beneficiary.encode(out);
        self.state_root.encode(out);
        self.transactions_root.encode(out);
        self.receipts_root.encode(out);
        self.logs_bloom.encode(out);
        self.difficulty.encode(out);
        self.number.encode(out);
        self.energy_limit.encode(out);
        self.energy_used.encode(out);
        self.timestamp.encode(out);
        self.extra_data.encode(out);
        self.nonce.encode(out);
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for Header {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        let _ = open_fastrlp::Header::decode(buf)?;

        let parent_hash = <H256 as open_fastrlp::Decodable>::decode(buf)?;
        let ommers_hash = <H256 as open_fastrlp::Decodable>::decode(buf)?;
        let beneficiary = <Address as open_fastrlp::Decodable>::decode(buf)?;
        let state_root = <H256 as open_fastrlp::Decodable>::decode(buf)?;
        let transactions_root = <H256 as open_fastrlp::Decodable>::decode(buf)?;
        let receipts_root = <H256 as open_fastrlp::Decodable>::decode(buf)?;
        let logs_bloom = <Bloom as open_fastrlp::Decodable>::decode(buf)?;
        let difficulty = <U256 as open_fastrlp::Decodable>::decode(buf)?;
        let number = <U256 as open_fastrlp::Decodable>::decode(buf)?;
        let energy_limit = <U256 as open_fastrlp::Decodable>::decode(buf)?;
        let energy_used = <U256 as open_fastrlp::Decodable>::decode(buf)?;
        let timestamp = <u64 as open_fastrlp::Decodable>::decode(buf)?;
        let extra_data = <Bytes as open_fastrlp::Decodable>::decode(buf)?;
        let nonce = <H64 as open_fastrlp::Decodable>::decode(buf)?;

        Ok(Header {
            parent_hash,
            ommers_hash,
            beneficiary,
            state_root,
            transactions_root,
            receipts_root,
            logs_bloom,
            difficulty,
            number,
            energy_limit,
            energy_used,
            timestamp,
            extra_data,
            nonce,
        })
    }
}

/// Partial header definition without ommers hash and transactions root
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PartialHeader {
    pub parent_hash: H256,
    pub beneficiary: Address,
    pub state_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: U256,
    pub energy_limit: U256,
    pub energy_used: U256,
    pub timestamp: u64,
    pub extra_data: Bytes,
    pub nonce: H64,
}

impl From<Header> for PartialHeader {
    fn from(header: Header) -> PartialHeader {
        Self {
            parent_hash: header.parent_hash,
            beneficiary: header.beneficiary,
            state_root: header.state_root,
            receipts_root: header.receipts_root,
            logs_bloom: header.logs_bloom,
            difficulty: header.difficulty,
            number: header.number,
            energy_limit: header.energy_used,
            energy_used: header.energy_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data,
            nonce: header.nonce,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use corebc_core::{
        types::H176,
        utils::{hex, hex::FromHex},
    };

    use super::*;

    #[test]
    fn header_rlp_roundtrip() {
        let mut header = Header {
            parent_hash: Default::default(),
            ommers_hash: Default::default(),
            beneficiary: Default::default(),
            state_root: Default::default(),
            transactions_root: Default::default(),
            receipts_root: Default::default(),
            logs_bloom: Default::default(),
            difficulty: Default::default(),
            number: 124u64.into(),
            energy_limit: Default::default(),
            energy_used: 1337u64.into(),
            timestamp: 0,
            extra_data: Default::default(),
            nonce: 99u64.to_be_bytes().into(),
        };

        let encoded = rlp::encode(&header);
        let decoded: Header = rlp::decode(encoded.as_ref()).unwrap();
        assert_eq!(header, decoded);

        let encoded = rlp::encode(&header);
        let decoded: Header = rlp::decode(encoded.as_ref()).unwrap();
        assert_eq!(header, decoded);
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn header_fastrlp_roundtrip() {
        let mut header = Header {
            parent_hash: Default::default(),
            ommers_hash: Default::default(),
            beneficiary: Default::default(),
            state_root: Default::default(),
            transactions_root: Default::default(),
            receipts_root: Default::default(),
            logs_bloom: Default::default(),
            difficulty: Default::default(),
            number: 124u64.into(),
            energy_limit: Default::default(),
            energy_used: 1337u64.into(),
            timestamp: 0,
            extra_data: Default::default(),
            nonce: H64::from_low_u64_be(99u64),
        };

        let mut encoded = vec![];
        <Header as open_fastrlp::Encodable>::encode(&header, &mut encoded);
        let decoded: Header =
            <Header as open_fastrlp::Decodable>::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(header, decoded);

        encoded.clear();
        <Header as open_fastrlp::Encodable>::encode(&header, &mut encoded);
        let decoded: Header =
            <Header as open_fastrlp::Decodable>::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(header, decoded);
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn test_encode_block_header() {
        use open_fastrlp::Encodable;

        let expected = hex::decode("f901daa00000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000009600000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788880000000000000000").unwrap();
        let mut data = vec![];
        let header = Header {
            parent_hash: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            ommers_hash: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            beneficiary: H176::from_str("00000000000000000000000000000000000000000000").unwrap(),
            state_root: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            transactions_root: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            receipts_root: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            logs_bloom: <[u8; 256]>::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap().into(),
            difficulty: 0x8aeu64.into(),
            number: 0xd05u64.into(),
            energy_limit: 0x115cu64.into(),
            energy_used: 0x15b3u64.into(),
            timestamp: 0x1a0au64,
            extra_data: hex::decode("7788").unwrap().into(),
            nonce: H64::from_low_u64_be(0x0),
        };
        header.encode(&mut data);
        assert_eq!(hex::encode(&data), hex::encode(expected));
        assert_eq!(header.length(), data.len());
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    fn test_decode_block_header() {
        let data = hex::decode("f901daa00000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000009600000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788880000000000000000").unwrap();

        use open_fastrlp::Encodable;
        let expected = Header {
            parent_hash: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            ommers_hash: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            beneficiary: H176::from_str("00000000000000000000000000000000000000000000").unwrap(),
            state_root: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            transactions_root: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            receipts_root: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            logs_bloom: <[u8; 256]>::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap().into(),
            difficulty: 0x8aeu64.into(),
            number: 0xd05u64.into(),
            energy_limit: 0x115cu64.into(),
            energy_used: 0x15b3u64.into(),
            timestamp: 0x1a0au64,
            extra_data: hex::decode("7788").unwrap().into(),
            nonce: H64::from_low_u64_be(0x0),
        };

        let header = <Header as open_fastrlp::Decodable>::decode(&mut data.as_slice()).unwrap();
        assert_eq!(header, expected);
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    // Test vector from network
    fn block_network_fastrlp_roundtrip() {
        use open_fastrlp::Encodable;
        let data = hex::decode("f901dff901daa00000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000009600000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788880000000000000000c0c0").unwrap();

        let block = <Block as open_fastrlp::Decodable>::decode(&mut data.as_slice()).unwrap();

        // encode and check that it matches the original data
        let mut encoded = Vec::new();
        <Block as open_fastrlp::Encodable>::encode(&block, &mut encoded);
        assert_eq!(data, encoded);

        // check that length of encoding is the same as the output of `length`
        assert_eq!(block.length(), encoded.len());
    }
}
