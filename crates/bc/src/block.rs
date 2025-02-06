/// Represents a block in the Best-Chain protocol.
/// 
/// A `BCBlock` contains the parent block hash, score, transactions, and its own hash.
/// 
/// ## Fields
/// - `parent`: The hash of the parent block.
/// - `score`: The block's score relative to the parent.
/// - `transactions`: The list of transactions included in the block.
/// - `hash`: The unique hash of the block.
use serde::{Serialize, Deserialize};
use rand::Rng;

/// A block in a best-chain protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCBlock {
    /// Parent block hash
    pub parent: Option<BlockHash>,
    /// Block score (relative to the parent)
    pub score: i32,
    /// Transactions in this block
    pub transactions: Vec<super::transaction::BCTransaction>,
    /// Block hash
    pub hash: BlockHash,
}

/// Unique value representing a best-chain block hash.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockHash(u64);

impl Default for BlockHash {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockHash {
    /// Create a new random block hash.
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        BlockHash(rng.gen())
    }
}

/// Trait for block operations.
pub trait BlockTrait {
    /// Get the parent block hash.
    fn parent(&self) -> &Option<BlockHash>;
    /// Get the block score.
    fn score(&self) -> i32;
    /// Get the transactions in this block.
    fn transactions(&self) -> &Vec<super::transaction::BCTransaction>;
}

impl BlockTrait for BCBlock {
    fn parent(&self) -> &Option<BlockHash> {
        &self.parent
    }

    fn score(&self) -> i32 {
        self.score
    }

    fn transactions(&self) -> &Vec<super::transaction::BCTransaction> {
        &self.transactions
    }
}
