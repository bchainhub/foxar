//! transaction related data

use crate::eth::receipt::Log;
use corebc_core::{
    types::{Address, Bloom, Bytes, Network, Signature, SignatureError, TxHash, H256, U256, U64},
    utils::{
        rlp,
        rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream},
        sha3,
    },
};
use foundry_evm::trace::CallTraceArena;
use foundry_utils::types::ToRuint;
use revm::{
    interpreter::InstructionResult,
    primitives::{CreateScheme, TransactTo, TxEnv},
};
use std::ops::Deref;

/// compatibility with `corebc-rs` types
mod ethers_compat;

/// The signature used to bypass signing via the `eth_sendUnsignedTransaction` cheat RPC
#[cfg(feature = "impersonated-tx")]
pub const IMPERSONATED_SIGNATURE: Signature = Signature { sig: corebc_core::types::U1368::zero() };

/// Container type for various Ethereum transaction requests
///
/// Its variants correspond to specific allowed transactions:
/// 1. Legacy (pre-EIP2718) [`LegacyTransactionRequest`]
/// 2. EIP2930 (state access lists) [`EIP2930TransactionRequest`]
/// 3. EIP1559 [`EIP1559TransactionRequest`]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypedTransactionRequest {
    Legacy(LegacyTransactionRequest),
}

/// Represents _all_ transaction requests received from RPC
#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct EthTransactionRequest {
    /// from address
    pub from: Option<Address>,
    /// to address
    pub to: Option<Address>,
    /// legacy, gas Price
    #[cfg_attr(feature = "serde", serde(default))]
    pub gas_price: Option<U256>,
    /// gas
    pub gas: Option<U256>,
    /// value of th tx in wei
    pub value: Option<U256>,
    /// Any additional data sent
    pub data: Option<Bytes>,
    /// Transaction nonce
    pub nonce: Option<U256>,
    /// chain id
    #[cfg_attr(feature = "serde", serde(default))]
    pub network_id: Option<U64>,
}

// == impl EthTransactionRequest ==

impl EthTransactionRequest {
    /// Converts the request into a [TypedTransactionRequest]
    pub fn into_typed_request(self) -> Option<TypedTransactionRequest> {
        let EthTransactionRequest { to, gas_price, gas, value, data, nonce, network_id, .. } = self;
        let network_id = network_id.map(|id| id.as_u64());
        match gas_price {
            Some(_) => Some(TypedTransactionRequest::Legacy(LegacyTransactionRequest {
                nonce: nonce.unwrap_or(U256::zero()),
                gas_price: gas_price.unwrap_or_default(),
                gas_limit: gas.unwrap_or_default(),
                value: value.unwrap_or(U256::zero()),
                input: data.unwrap_or_default(),
                kind: match to {
                    Some(to) => TransactionKind::Call(to),
                    None => TransactionKind::Create,
                },
                network_id,
            })),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransactionKind {
    Call(Address),
    Create,
}

// == impl TransactionKind ==

impl TransactionKind {
    /// If this transaction is a call this returns the address of the callee
    pub fn as_call(&self) -> Option<&Address> {
        match self {
            TransactionKind::Call(to) => Some(to),
            TransactionKind::Create => None,
        }
    }
}

impl Encodable for TransactionKind {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            TransactionKind::Call(address) => {
                s.encoder().encode_value(&address[..]);
            }
            TransactionKind::Create => s.encoder().encode_value(&[]),
        }
    }
}

impl Decodable for TransactionKind {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            if rlp.is_data() {
                Ok(TransactionKind::Create)
            } else {
                Err(DecoderError::RlpExpectedToBeData)
            }
        } else {
            Ok(TransactionKind::Call(rlp.as_val()?))
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for TransactionKind {
    fn length(&self) -> usize {
        match self {
            TransactionKind::Call(to) => to.length(),
            TransactionKind::Create => ([]).length(),
        }
    }
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        match self {
            TransactionKind::Call(to) => to.encode(out),
            TransactionKind::Create => ([]).encode(out),
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for TransactionKind {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        use bytes::Buf;

        if let Some(&first) = buf.first() {
            if first == 0x80 {
                buf.advance(1);
                Ok(TransactionKind::Create)
            } else {
                let addr = <Address as open_fastrlp::Decodable>::decode(buf)?;
                Ok(TransactionKind::Call(addr))
            }
        } else {
            Err(open_fastrlp::DecodeError::InputTooShort)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyTransactionRequest {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub kind: TransactionKind,
    pub value: U256,
    pub input: Bytes,
    pub network_id: u64,
}

// == impl LegacyTransactionRequest ==

impl LegacyTransactionRequest {
    pub fn hash(&self) -> H256 {
        H256::from_slice(sha3(&rlp::encode(self)).as_slice())
    }
}

impl From<LegacyTransaction> for LegacyTransactionRequest {
    fn from(tx: LegacyTransaction) -> Self {
        let network_id = tx.network_id();
        Self {
            nonce: tx.nonce,
            gas_price: tx.gas_price,
            gas_limit: tx.gas_limit,
            kind: tx.kind,
            value: tx.value,
            input: tx.input,
            network_id,
        }
    }
}

impl Encodable for LegacyTransactionRequest {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(8);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.network_id);
        s.append(&self.kind);
        s.append(&self.value);
        s.append(&self.input);
        s.append(&self.input.as_ref());
    }
}

/// A wrapper for `TypedTransaction` that allows impersonating accounts.
///
/// This is a helper that carries the `impersonated` sender so that the right hash
/// [TypedTransaction::impersonated_hash] can be created.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MaybeImpersonatedTransaction {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub transaction: TypedTransaction,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub impersonated_sender: Option<Address>,
}

// === impl MaybeImpersonatedTransaction ===

impl MaybeImpersonatedTransaction {
    /// Creates a new wrapper for the given transaction
    pub fn new(transaction: TypedTransaction) -> Self {
        Self { transaction, impersonated_sender: None }
    }

    /// Creates a new impersonated transaction wrapper using the given sender
    pub fn impersonated(transaction: TypedTransaction, impersonated_sender: Address) -> Self {
        Self { transaction, impersonated_sender: Some(impersonated_sender) }
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    ///
    /// Note: this is feature gated so it does not conflict with the `Deref`ed
    /// [TypedTransaction::recover] function by default.
    #[cfg(feature = "impersonated-tx")]
    pub fn recover(&self) -> Result<Address, SignatureError> {
        if let Some(sender) = self.impersonated_sender {
            return Ok(sender);
        }
        self.transaction.recover()
    }

    /// Returns the hash of the transaction
    ///
    /// Note: this is feature gated so it does not conflict with the `Deref`ed
    /// [TypedTransaction::hash] function by default.
    #[cfg(feature = "impersonated-tx")]
    pub fn hash(&self) -> H256 {
        if self.transaction.is_impersonated() {
            if let Some(sender) = self.impersonated_sender {
                return self.transaction.impersonated_hash(sender);
            }
        }
        self.transaction.hash()
    }
}

impl Encodable for MaybeImpersonatedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.transaction.rlp_append(s)
    }
}

impl From<MaybeImpersonatedTransaction> for TypedTransaction {
    fn from(value: MaybeImpersonatedTransaction) -> Self {
        value.transaction
    }
}

impl From<TypedTransaction> for MaybeImpersonatedTransaction {
    fn from(value: TypedTransaction) -> Self {
        MaybeImpersonatedTransaction::new(value)
    }
}

impl Decodable for MaybeImpersonatedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let transaction = TypedTransaction::decode(rlp)?;
        Ok(Self { transaction, impersonated_sender: None })
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for MaybeImpersonatedTransaction {
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        self.transaction.encode(out)
    }
    fn length(&self) -> usize {
        self.transaction.length()
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for MaybeImpersonatedTransaction {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        Ok(Self { transaction: open_fastrlp::Decodable::decode(buf)?, impersonated_sender: None })
    }
}

impl AsRef<TypedTransaction> for MaybeImpersonatedTransaction {
    fn as_ref(&self) -> &TypedTransaction {
        &self.transaction
    }
}

impl Deref for MaybeImpersonatedTransaction {
    type Target = TypedTransaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypedTransaction {
    /// Legacy transaction type
    Legacy(LegacyTransaction),
}

// == impl TypedTransaction ==

impl TypedTransaction {
    pub fn gas_price(&self) -> U256 {
        match self {
            TypedTransaction::Legacy(tx) => tx.gas_price,
        }
    }

    pub fn gas_limit(&self) -> U256 {
        match self {
            TypedTransaction::Legacy(tx) => tx.gas_limit,
        }
    }

    pub fn value(&self) -> U256 {
        match self {
            TypedTransaction::Legacy(tx) => tx.value,
        }
    }

    pub fn data(&self) -> &Bytes {
        match self {
            TypedTransaction::Legacy(tx) => &tx.input,
        }
    }

    /// Returns the transaction type
    pub fn r#type(&self) -> Option<u8> {
        match self {
            TypedTransaction::Legacy(_) => None,
        }
    }

    /// Max cost of the transaction
    pub fn max_cost(&self) -> U256 {
        self.gas_limit().saturating_mul(self.gas_price())
    }

    /// Returns a helper type that contains commonly used values as fields
    pub fn essentials(&self) -> TransactionEssentials {
        match self {
            TypedTransaction::Legacy(t) => TransactionEssentials {
                kind: t.kind,
                input: t.input.clone(),
                nonce: t.nonce,
                gas_limit: t.gas_limit,
                gas_price: Some(t.gas_price),
                value: t.value,
                network_id: t.network_id(),
            },
        }
    }

    pub fn nonce(&self) -> &U256 {
        match self {
            TypedTransaction::Legacy(t) => t.nonce(),
        }
    }

    pub fn network_id(&self) -> u64 {
        match self {
            TypedTransaction::Legacy(t) => t.network_id(),
        }
    }

    pub fn as_legacy(&self) -> Option<&LegacyTransaction> {
        match self {
            TypedTransaction::Legacy(tx) => Some(tx),
        }
    }

    /// Returns true whether this tx is a legacy transaction
    pub fn is_legacy(&self) -> bool {
        matches!(self, TypedTransaction::Legacy(_))
    }

    /// Returns the hash of the transaction.
    ///
    /// Note: If this transaction has the Impersonated signature then this returns a modified unique
    /// hash. This allows us to treat impersonated transactions as unique.
    pub fn hash(&self) -> H256 {
        match self {
            TypedTransaction::Legacy(t) => t.hash(),
        }
    }

    /// Returns true if the transaction was impersonated (using the impersonate Signature)
    #[cfg(feature = "impersonated-tx")]
    pub fn is_impersonated(&self) -> bool {
        self.signature() == IMPERSONATED_SIGNATURE
    }

    /// Returns the hash if the transaction is impersonated (using a fake signature)
    ///
    /// This appends the `address` before hashing it
    #[cfg(feature = "impersonated-tx")]
    pub fn impersonated_hash(&self, sender: Address) -> H256 {
        let mut bytes = rlp::encode(self);
        bytes.extend_from_slice(sender.as_ref());
        H256::from_slice(sha3(&bytes).as_slice())
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    pub fn recover(&self) -> Result<Address, SignatureError> {
        match self {
            TypedTransaction::Legacy(tx) => tx.recover(),
        }
    }

    /// Returns what kind of transaction this is
    pub fn kind(&self) -> &TransactionKind {
        match self {
            TypedTransaction::Legacy(tx) => &tx.kind,
        }
    }

    /// Returns the callee if this transaction is a call
    pub fn to(&self) -> Option<&Address> {
        self.kind().as_call()
    }

    /// Returns the Signature of the transaction
    pub fn signature(&self) -> Signature {
        match self {
            TypedTransaction::Legacy(tx) => tx.signature,
        }
    }
}

impl Encodable for TypedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            TypedTransaction::Legacy(tx) => tx.rlp_append(s),
        }
    }
}

impl Decodable for TypedTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(TypedTransaction::Legacy(rlp.as_val()?))
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Encodable for TypedTransaction {
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        match self {
            TypedTransaction::Legacy(tx) => tx.encode(out),
        }
    }
    fn length(&self) -> usize {
        match self {
            TypedTransaction::Legacy(tx) => tx.length(),
        }
    }
}

#[cfg(feature = "fastrlp")]
impl open_fastrlp::Decodable for TypedTransaction {
    fn decode(buf: &mut &[u8]) -> Result<Self, open_fastrlp::DecodeError> {
        <LegacyTransaction as open_fastrlp::Decodable>::decode(buf).map(TypedTransaction::Legacy)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "fastrlp", derive(open_fastrlp::RlpEncodable, open_fastrlp::RlpDecodable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegacyTransaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub network_id: u64,
    pub kind: TransactionKind,
    pub value: U256,
    pub input: Bytes,
    pub signature: Signature,
}

impl LegacyTransaction {
    pub fn nonce(&self) -> &U256 {
        &self.nonce
    }

    pub fn hash(&self) -> H256 {
        H256::from_slice(sha3(&rlp::encode(self)).as_slice())
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    pub fn recover(&self) -> Result<Address, SignatureError> {
        self.signature.recover(
            LegacyTransactionRequest::from(self.clone()).hash(),
            &Network::from(self.network_id()),
        )
    }

    pub fn network_id(&self) -> u64 {
        self.network_id
    }

    /// See <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md>
    /// > If you do, then the v of the signature MUST be set to {0,1} + CHAIN_ID * 2 + 35 where
    /// > {0,1} is the parity of the y value of the curve point for which r is the x-value in the
    /// > secp256k1 signing process.
    pub fn meets_eip155(&self, _chain_id: u64) -> bool {
        todo!("CORETODO GET NETWORK ID SOMEHOW");
        // let double_chain_id = chain_id.saturating_mul(2);
        // let v = self.signature.v;
        // v == double_chain_id + 35 || v == double_chain_id + 36;
    }
}

impl Encodable for LegacyTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(8);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.kind);
        s.append(&self.value);
        s.append(&self.input.as_ref());
        s.append(&self.signature.sig);
    }
}

impl Decodable for LegacyTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.item_count()? != 8 {
            return Err(DecoderError::RlpIncorrectListLen);
        }

        Ok(Self {
            nonce: rlp.val_at(0)?,
            gas_price: rlp.val_at(1)?,
            gas_limit: rlp.val_at(2)?,
            network_id: rlp.val_at(3)?,
            kind: rlp.val_at(4)?,
            value: rlp.val_at(5)?,
            input: rlp.val_at::<Vec<u8>>(6)?.into(),
            signature: Signature { sig: rlp.val_at::<U1368>(7)? },
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionEssentials {
    pub kind: TransactionKind,
    pub input: Bytes,
    pub nonce: U256,
    pub gas_limit: U256,
    pub gas_price: Option<U256>,
    pub value: U256,
    pub network_id: u64,
}

/// Queued transaction
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingTransaction {
    /// The actual transaction
    pub transaction: MaybeImpersonatedTransaction,
    /// the recovered sender of this transaction
    sender: Address,
    /// hash of `transaction`, so it can easily be reused with encoding and hashing agan
    hash: TxHash,
}

// == impl PendingTransaction ==

impl PendingTransaction {
    /// Creates a new pending transaction and tries to verify transaction and recover sender.
    pub fn new(transaction: TypedTransaction) -> Result<Self, SignatureError> {
        let sender = transaction.recover()?;
        Ok(Self { hash: transaction.hash(), transaction: transaction.into(), sender })
    }

    /// Creates a new transaction with the given sender.
    ///
    /// In order to prevent collisions from multiple different impersonated accounts, we update the
    /// transaction's hash with the address to make it unique.
    ///
    /// See: <https://github.com/foundry-rs/foundry/issues/3759>
    #[cfg(feature = "impersonated-tx")]
    pub fn with_impersonated(transaction: TypedTransaction, sender: Address) -> Self {
        let hash = transaction.impersonated_hash(sender);
        let transaction = MaybeImpersonatedTransaction::impersonated(transaction, sender);
        Self { hash, transaction, sender }
    }

    pub fn nonce(&self) -> &U256 {
        self.transaction.nonce()
    }

    pub fn hash(&self) -> &TxHash {
        &self.hash
    }

    pub fn sender(&self) -> &Address {
        &self.sender
    }

    /// Converts the [PendingTransaction] into the [TxEnv] context that [`revm`](foundry_evm)
    /// expects.
    pub fn to_revm_tx_env(&self) -> TxEnv {
        fn transact_to(kind: &TransactionKind) -> TransactTo {
            match kind {
                TransactionKind::Call(c) => TransactTo::Call((*c).into()),
                TransactionKind::Create => TransactTo::Create(CreateScheme::Create),
            }
        }

        let caller = *self.sender();
        match &self.transaction.transaction {
            TypedTransaction::Legacy(tx) => {
                let network_id = tx.network_id();
                let LegacyTransaction { nonce, gas_price, gas_limit, value, kind, input, .. } = tx;
                TxEnv {
                    caller: caller.into(),
                    transact_to: transact_to(kind),
                    data: input.0.clone(),
                    network_id: Some(network_id),
                    nonce: Some(nonce.as_u64()),
                    value: (*value).to_ruint(),
                    energy_price: (*gas_price).to_ruint(),
                    energy_limit: gas_limit.as_u64(),
                }
            }
        }
    }
}

/// Represents all relevant information of an executed transaction
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransactionInfo {
    pub transaction_hash: H256,
    pub transaction_index: u32,
    pub from: Address,
    pub to: Option<Address>,
    pub contract_address: Option<Address>,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
    pub traces: CallTraceArena,
    pub exit: InstructionResult,
    pub out: Option<Bytes>,
}

// === impl TransactionInfo ===

impl TransactionInfo {
    /// Returns the `traceAddress` of the node in the arena
    ///
    /// The `traceAddress` field of all returned traces, gives the exact location in the call trace
    /// [index in root, index in first CALL, index in second CALL, …].
    ///
    /// # Panics
    ///
    /// if the `idx` does not belong to a node
    pub fn trace_address(&self, idx: usize) -> Vec<usize> {
        if idx == 0 {
            // root call has empty traceAddress
            return vec![];
        }
        let mut graph = vec![];
        let mut node = &self.traces.arena[idx];
        while let Some(parent) = node.parent {
            // the index of the child call in the arena
            let child_idx = node.idx;
            node = &self.traces.arena[parent];
            // find the index of the child call in the parent node
            let call_idx = node
                .children
                .iter()
                .position(|child| *child == child_idx)
                .expect("child exists in parent");
            graph.push(call_idx);
        }
        graph.reverse();
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corebc_core::types::U1368;
    use corebc_core::utils::hex;

    #[test]
    fn can_recover_sender() {
        let bytes = hex::decode("f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804").unwrap();

        let tx: TypedTransaction = rlp::decode(&bytes).expect("decoding TypedTransaction failed");
        let tx = match tx {
            TypedTransaction::Legacy(tx) => tx,
            _ => panic!("Invalid typed transaction"),
        };
        assert_eq!(tx.input, Bytes::from(b""));
        assert_eq!(tx.gas_price, U256::from(0x01u64));
        assert_eq!(tx.gas_limit, U256::from(0x5208u64));
        assert_eq!(tx.nonce, U256::from(0x00u64));
        if let TransactionKind::Call(ref to) = tx.kind {
            assert_eq!(*to, "095e7baea6a6c7c4c2dfeb977efac326af552d87".parse().unwrap());
        } else {
            panic!();
        }
        assert_eq!(tx.value, U256::from(0x0au64));
        assert_eq!(
            tx.recover().unwrap(),
            "0f65fe9276bc9a24ae7083ae28e2660ef72df99e".parse().unwrap()
        );
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn test_decode_fastrlp_create_goerli() {
        // test that an example create tx from goerli decodes properly
        let tx_bytes =
              hex::decode("02f901ee05228459682f008459682f11830209bf8080b90195608060405234801561001057600080fd5b50610175806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c80630c49c36c14610030575b600080fd5b61003861004e565b604051610045919061011d565b60405180910390f35b60606020600052600f6020527f68656c6c6f2073746174656d696e64000000000000000000000000000000000060405260406000f35b600081519050919050565b600082825260208201905092915050565b60005b838110156100be5780820151818401526020810190506100a3565b838111156100cd576000848401525b50505050565b6000601f19601f8301169050919050565b60006100ef82610084565b6100f9818561008f565b93506101098185602086016100a0565b610112816100d3565b840191505092915050565b6000602082019050818103600083015261013781846100e4565b90509291505056fea264697066735822122051449585839a4ea5ac23cae4552ef8a96b64ff59d0668f76bfac3796b2bdbb3664736f6c63430008090033c080a0136ebffaa8fc8b9fda9124de9ccb0b1f64e90fbd44251b4c4ac2501e60b104f9a07eb2999eec6d185ef57e91ed099afb0a926c5b536f0155dd67e537c7476e1471")
                  .unwrap();
        let _decoded =
            <TypedTransaction as open_fastrlp::Decodable>::decode(&mut &tx_bytes[..]).unwrap();
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn decode_transaction_consumes_buffer() {
        let bytes = &mut &hex::decode("b87502f872041a8459682f008459682f0d8252089461815774383099e24810ab832a5b2a5425c154d58829a2241af62c000080c001a059e6b67f48fb32e7e570dfb11e042b5ad2e55e3ce3ce9cd989c7e06e07feeafda0016b83f4f980694ed2eee4d10667242b1f40dc406901b34125b008d334d47469").unwrap()[..];
        let _transaction_res =
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes).unwrap();
        assert_eq!(
            bytes.len(),
            0,
            "did not consume all bytes in the buffer, {:?} remaining",
            bytes.len()
        );
    }

    #[test]
    #[cfg(feature = "fastrlp")]
    fn decode_multiple_network_txs() {
        use std::str::FromStr;

        let bytes_first = &mut &hex::decode("f86b02843b9aca00830186a094d3e8763675e4c425df46cc3b5c0f6cbdac39604687038d7ea4c68000802ba00eb96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5aea03a456401896b1b6055311536bf00a718568c744d8c1f9df59879e8350220ca18").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 2u64.into(),
            gas_price: 1000000000u64.into(),
            gas_limit: 100000u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 1000000000000000u64.into(),
            input: Bytes::default(),
            signature: Signature { sig: U1368::from_str("").unwrap() },
        });
        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_first).unwrap()
        );

        let bytes_second = &mut &hex::decode("f86b01843b9aca00830186a094d3e8763675e4c425df46cc3b5c0f6cbdac3960468702769bb01b2a00802ba0e24d8bd32ad906d6f8b8d7741e08d1959df021698b19ee232feba15361587d0aa05406ad177223213df262cb66ccbb2f46bfdccfdfbbb5ffdda9e2c02d977631da").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 1u64.into(),
            gas_price: 1000000000u64.into(),
            gas_limit: 100000u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 693361000000000u64.into(),
            input: Bytes::default(),
            signature: Signature { sig: U1368::from_str("").unwrap() },
        });
        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_second).unwrap()
        );

        let bytes_third = &mut &hex::decode("f86b0384773594008398968094d3e8763675e4c425df46cc3b5c0f6cbdac39604687038d7ea4c68000802ba0ce6834447c0a4193c40382e6c57ae33b241379c5418caac9cdc18d786fd12071a03ca3ae86580e94550d7c071e3a02eadb5a77830947c9225165cf9100901bee88").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 3u64.into(),
            gas_price: 2000000000u64.into(),
            gas_limit: 10000000u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 1000000000000000u64.into(),
            input: Bytes::default(),
            signature: Signature { sig: U1368::from_str("").unwrap() },
        });
        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_third).unwrap()
        );

        let bytes_fifth = &mut &hex::decode("f8650f84832156008287fb94cf7f9e66af820a19257a2108375b180b0ec491678204d2802ca035b7bfeb9ad9ece2cbafaaf8e202e706b4cfaeb233f46198f00b44d4a566a981a0612638fb29427ca33b9a3be2a0a561beecfe0269655be160d35e72d366a6a860").unwrap()[..];
        let expected = TypedTransaction::Legacy(LegacyTransaction {
            nonce: 15u64.into(),
            gas_price: 2200000000u64.into(),
            gas_limit: 34811u64.into(),
            network_id: 1,
            kind: TransactionKind::Call(Address::from_slice(
                &hex::decode("cf7f9e66af820a19257a2108375b180b0ec49167").unwrap()[..],
            )),
            value: 1234u64.into(),
            input: Bytes::default(),
            signature: Signature { sig: U1 },
        });
        assert_eq!(
            expected,
            <TypedTransaction as open_fastrlp::Decodable>::decode(bytes_fifth).unwrap()
        );
    }

    // <https://github.com/gakonst/corebc-rs/issues/1732>
    #[test]
    fn test_recover_legacy_tx() {
        let raw_tx = "f9015482078b8505d21dba0083022ef1947a250d5630b4cf539739df2c5dacb4c659f2488d880c46549a521b13d8b8e47ff36ab50000000000000000000000000000000000000000000066ab5a608bd00a23f2fe000000000000000000000000000000000000000000000000000000000000008000000000000000000000000048c04ed5691981c42154c6167398f95e8f38a7ff00000000000000000000000000000000000000000000000000000000632ceac70000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000006c6ee5e31d828de241282b9606c8e98ea48526e225a0c9077369501641a92ef7399ff81c21639ed4fd8fc69cb793cfa1dbfab342e10aa0615facb2f1bcf3274a354cfe384a38d0cc008a11c2dd23a69111bc6930ba27a8";

        let tx: TypedTransaction = rlp::decode(&hex::decode(raw_tx).unwrap()).unwrap();
        let recovered = tx.recover().unwrap();
        let expected: Address = "0xa12e1462d0ced572f396f58b6e2d03894cd7c8a4".parse().unwrap();
        assert_eq!(expected, recovered);
    }
}
