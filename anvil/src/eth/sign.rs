use crate::eth::error::BlockchainError;
use anvil_core::eth::transaction::{
    LegacyTransaction, LegacyTransactionRequest, TypedTransaction, TypedTransactionRequest,
};
use corebc::{
    core::libgoldilocks::SigningKey,
    prelude::{Address, Wallet},
    signers::Signer as EthersSigner,
    types::{
        transaction::{
            eip2718::TypedTransaction as EthersTypedTransactionRequest, eip712::TypedData,
        },
        Signature,
    },
};
use std::collections::HashMap;

/// A transaction signer
#[async_trait::async_trait]
pub trait Signer: Send + Sync {
    /// returns the available accounts for this signer
    fn accounts(&self) -> Vec<Address>;

    /// Returns `true` whether this signer can sign for this address
    fn is_signer_for(&self, addr: Address) -> bool {
        self.accounts().contains(&addr)
    }

    /// Returns the signature
    async fn sign(&self, address: Address, message: &[u8]) -> Result<Signature, BlockchainError>;

    /// Encodes and signs the typed data according EIP-712. Payload must implement Eip712 trait.
    async fn sign_typed_data(
        &self,
        address: Address,
        payload: &TypedData,
    ) -> Result<Signature, BlockchainError>;

    /// signs a transaction request using the given account in request
    fn sign_transaction(
        &self,
        request: TypedTransactionRequest,
        address: &Address,
    ) -> Result<Signature, BlockchainError>;
}

/// Maintains developer keys
pub struct DevSigner {
    addresses: Vec<Address>,
    accounts: HashMap<Address, Wallet<SigningKey>>,
}

impl DevSigner {
    pub fn new(accounts: Vec<Wallet<SigningKey>>) -> Self {
        let addresses = accounts.iter().map(|wallet| wallet.address()).collect::<Vec<_>>();
        let accounts = addresses.iter().cloned().zip(accounts).collect();
        Self { addresses, accounts }
    }
}

#[async_trait::async_trait]
impl Signer for DevSigner {
    fn accounts(&self) -> Vec<Address> {
        self.addresses.clone()
    }

    fn is_signer_for(&self, addr: Address) -> bool {
        self.accounts.contains_key(&addr)
    }

    async fn sign(&self, address: Address, message: &[u8]) -> Result<Signature, BlockchainError> {
        let signer = self.accounts.get(&address).ok_or(BlockchainError::NoSignerAvailable)?;

        Ok(signer.sign_message(message).await?)
    }

    async fn sign_typed_data(
        &self,
        address: Address,
        payload: &TypedData,
    ) -> Result<Signature, BlockchainError> {
        let signer = self.accounts.get(&address).ok_or(BlockchainError::NoSignerAvailable)?;
        Ok(signer.sign_typed_data(payload).await?)
    }

    fn sign_transaction(
        &self,
        request: TypedTransactionRequest,
        address: &Address,
    ) -> Result<Signature, BlockchainError> {
        let signer = self.accounts.get(address).ok_or(BlockchainError::NoSignerAvailable)?;
        let ethers_tx: EthersTypedTransactionRequest = request.into();

        Ok(signer.sign_transaction_sync(&ethers_tx)?)
    }
}

/// converts the `request` into a [`TypedTransactionRequest`] with the given signature
///
/// # Errors
///
/// This will fail if the `signature` contains an erroneous recovery id.
pub fn build_typed_transaction(
    request: TypedTransactionRequest,
    signature: Signature,
) -> Result<TypedTransaction, BlockchainError> {
    let tx = match request {
        TypedTransactionRequest::Legacy(tx) => {
            let LegacyTransactionRequest {
                nonce,
                gas_price,
                gas_limit,
                kind,
                value,
                input,
                network_id,
                ..
            } = tx;
            TypedTransaction::Legacy(LegacyTransaction {
                nonce,
                gas_price,
                gas_limit,
                network_id,
                kind,
                value,
                input,
                signature,
            })
        }
    };

    Ok(tx)
}
