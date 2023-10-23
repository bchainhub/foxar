//! Utility functions for Ethereum adapted from https://github.dev/rust-blockchain/ethereum/blob/755dffaa4903fbec1269f50cde9863cf86269a14/src/util.rs
use corebc_core::types::H256;

pub use keccak_hasher::KeccakHasher;

// reexport some trie types
pub use reference_trie::*;

/// The SHA-3 hash of the RLP encoding of empty data. So 0x80
pub const SHA3_NULL_RLP: H256 = H256([
    0x14, 0x48, 0x0e, 0x81, 0x06, 0xce, 0x92, 0x72, 0x89, 0x12, 0x88, 0x06, 0xf6, 0x1a, 0xac, 0x38,
    0x1b, 0x48, 0x40, 0xca, 0x4c, 0x22, 0x12, 0xa8, 0x57, 0xd9, 0x1a, 0xd5, 0x5b, 0x0e, 0xee, 0xa5,
]);

/// Generates a trie root hash for a vector of key-value tuples
pub fn trie_root<I, K, V>(input: I) -> H256
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<[u8]> + Ord,
    V: AsRef<[u8]>,
{
    H256::from(triehash::trie_root::<KeccakHasher, _, _, _>(input))
}

/// Generates a key-hashed (secure) trie root hash for a vector of key-value tuples.
pub fn sec_trie_root<I, K, V>(input: I) -> H256
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    H256::from(triehash::sec_trie_root::<KeccakHasher, _, _, _>(input))
}

/// Generates a trie root hash for a vector of values
pub fn ordered_trie_root<I, V>(input: I) -> H256
where
    I: IntoIterator<Item = V>,
    V: AsRef<[u8]>,
{
    H256::from(triehash::ordered_trie_root::<KeccakHasher, I>(input))
}
