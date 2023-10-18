use crate::eth::{
    backend::{info::StorageInfo, notifications::NewBlockNotifications},
    error::BlockchainError,
};
use anvil_core::eth::transaction::TypedTransaction;
use corebc::types::{H256, U256};
use foundry_evm::revm::primitives::SpecId;
use futures::StreamExt;
use parking_lot::{Mutex, RwLock};
use std::{
    collections::BTreeMap,
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tracing::trace;

/// Maximum number of entries in the fee history cache
pub const MAX_FEE_HISTORY_CACHE_SIZE: u64 = 2048u64;

/// Initial base fee for EIP-1559 blocks.
pub const INITIAL_BASE_FEE: u64 = 1_000_000_000;

/// Initial default gas price for the first block
pub const INITIAL_GAS_PRICE: u64 = 1_875_000_000;

/// Bounds the amount the base fee can change between blocks.
pub const BASE_FEE_CHANGE_DENOMINATOR: u64 = 8;

/// Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_ELASTICITY_MULTIPLIER: u64 = 2;

pub fn default_elasticity() -> f64 {
    1f64 / BASE_FEE_CHANGE_DENOMINATOR as f64
}

/// Stores the fee related information
#[derive(Debug, Clone)]
pub struct FeeManager {
    /// Hardfork identifier
    spec_id: SpecId,
    /// The base price to use Pre London
    ///
    /// This will be constant value unless changed manually
    gas_price: Arc<RwLock<U256>>,
    elasticity: Arc<RwLock<f64>>,
}

// === impl FeeManager ===

impl FeeManager {
    pub fn new(spec_id: SpecId, gas_price: U256) -> Self {
        Self {
            spec_id,
            gas_price: Arc::new(RwLock::new(gas_price)),
            elasticity: Arc::new(RwLock::new(default_elasticity())),
        }
    }

    pub fn elasticity(&self) -> f64 {
        *self.elasticity.read()
    }

    /// Calculates the current gas price
    pub fn gas_price(&self) -> U256 {
        *self.gas_price.read()
    }

    /// Returns the current gas price
    pub fn set_gas_price(&self, price: U256) {
        let mut gas = self.gas_price.write();
        *gas = price;
    }
}

/// An async service that takes care of the `FeeHistory` cache
pub struct FeeHistoryService {
    /// incoming notifications about new blocks
    new_blocks: NewBlockNotifications,
    /// contains all fee history related entries
    cache: FeeHistoryCache,
    /// number of items to consider
    fee_history_limit: u64,
    // current fee info
    fees: FeeManager,
    /// a type that can fetch ethereum-storage data
    storage_info: StorageInfo,
}

// === impl FeeHistoryService ===

impl FeeHistoryService {
    pub fn new(
        new_blocks: NewBlockNotifications,
        cache: FeeHistoryCache,
        fees: FeeManager,
        storage_info: StorageInfo,
    ) -> Self {
        Self {
            new_blocks,
            cache,
            fee_history_limit: MAX_FEE_HISTORY_CACHE_SIZE,
            fees,
            storage_info,
        }
    }

    /// Returns the configured history limit
    pub fn fee_history_limit(&self) -> u64 {
        self.fee_history_limit
    }

    /// Create a new history entry for the block
    fn create_cache_entry(
        &self,
        hash: H256,
        elasticity: f64,
    ) -> (FeeHistoryCacheItem, Option<u64>) {
        // percentile list from 0.0 to 100.0 with a 0.5 resolution.
        // this will create 200 percentile points
        let reward_percentiles: Vec<f64> = {
            let mut percentile: f64 = 0.0;
            (0..=200)
                .map(|_| {
                    let val = percentile;
                    percentile += 0.5;
                    val
                })
                .collect()
        };

        let mut block_number: Option<u64> = None;
        let mut item = FeeHistoryCacheItem { gas_used_ratio: 0f64, rewards: Vec::new() };

        let current_block = self.storage_info.block(hash);
        let current_receipts = self.storage_info.receipts(hash);

        if let (Some(block), Some(receipts)) = (current_block, current_receipts) {
            block_number = Some(block.header.number.as_u64());

            let gas_used = block.header.gas_used.as_u64() as f64;
            let gas_limit = block.header.gas_limit.as_u64() as f64;

            let gas_target = gas_limit / elasticity;
            item.gas_used_ratio = gas_used / (gas_target * elasticity);

            // extract useful tx info (gas_used, effective_reward)
            let mut transactions: Vec<(u64, u64)> = receipts
                .iter()
                .enumerate()
                .map(|(i, receipt)| {
                    let gas_used = receipt.gas_used().as_u64();
                    let effective_reward = match block.transactions.get(i).map(|tx| &tx.transaction)
                    {
                        Some(TypedTransaction::Legacy(t)) => t.gas_price.as_u64(),
                        None => 0,
                    };

                    (gas_used, effective_reward)
                })
                .collect();

            // sort by effective reward asc
            transactions.sort_by(|(_, a), (_, b)| a.cmp(b));

            // calculate percentile rewards
            item.rewards = reward_percentiles
                .into_iter()
                .filter_map(|p| {
                    let target_gas = (p * gas_used / 100f64) as u64;
                    let mut sum_gas = 0;
                    for (gas_used, effective_reward) in transactions.iter().cloned() {
                        sum_gas += gas_used;
                        if target_gas <= sum_gas {
                            return Some(effective_reward)
                        }
                    }
                    None
                })
                .collect();
        } else {
            item.rewards = reward_percentiles.iter().map(|_| 0).collect();
        }
        (item, block_number)
    }

    fn insert_cache_entry(&self, item: FeeHistoryCacheItem, block_number: Option<u64>) {
        if let Some(block_number) = block_number {
            trace!(target: "fees", "insert new history item={:?} for {}", item, block_number);
            let mut cache = self.cache.lock();
            cache.insert(block_number, item);

            // adhere to cache limit
            let pop_next = block_number.saturating_sub(self.fee_history_limit);

            let num_remove = (cache.len() as u64).saturating_sub(self.fee_history_limit);
            for num in 0..num_remove {
                let key = pop_next - num;
                cache.remove(&key);
            }
        }
    }
}

// An endless future that listens for new blocks and updates the cache
impl Future for FeeHistoryService {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pin = self.get_mut();

        while let Poll::Ready(Some(notification)) = pin.new_blocks.poll_next_unpin(cx) {
            let hash = notification.hash;
            let elasticity = default_elasticity();

            // add the imported block.
            let (result, block_number) = pin.create_cache_entry(hash, elasticity);
            pin.insert_cache_entry(result, block_number)
        }

        Poll::Pending
    }
}

pub type FeeHistoryCache = Arc<Mutex<BTreeMap<u64, FeeHistoryCacheItem>>>;

/// A single item in the whole fee history cache
#[derive(Debug, Clone)]
pub struct FeeHistoryCacheItem {
    pub gas_used_ratio: f64,
    pub rewards: Vec<u64>,
}

#[derive(Default, Clone)]
pub struct FeeDetails {
    pub gas_price: Option<U256>,
}

impl FeeDetails {
    /// All values zero
    pub fn zero() -> Self {
        Self { gas_price: Some(U256::zero()) }
    }

    /// If neither `gas_price` nor `max_fee_per_gas` is `Some`, this will set both to `0`
    pub fn or_zero_fees(self) -> Self {
        let FeeDetails { gas_price } = self;

        let no_fees = gas_price.is_none();
        let gas_price = if no_fees { Some(U256::zero()) } else { gas_price };

        Self { gas_price }
    }

    /// Turns this type into a tuple
    pub fn split(self) -> Option<U256> {
        let Self { gas_price } = self;
        gas_price
    }

    pub fn gas_price(&self) -> Option<U256> {
        self.gas_price
    }

    /// Creates a new instance from the request's gas related values
    pub fn new(request_gas_price: Option<U256>) -> Result<FeeDetails, BlockchainError> {
        match request_gas_price {
            gas_price => {
                // Legacy request, all default to gas price.
                Ok(FeeDetails { gas_price })
            }
        }
    }
}

impl fmt::Debug for FeeDetails {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Fees {{ ")?;
        write!(fmt, "gaPrice: {:?}, ", self.gas_price)?;
        write!(fmt, "}}")?;
        Ok(())
    }
}
