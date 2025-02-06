//! # Best-Chain Protocol (BC)
//!
//! The `bc` crate implements the core types and logic for the best-chain protocol,
//! including transactions, blocks, and context management.
//!
//! ## Features
//! - Transaction validation
//! - Block creation
//! - Context management

pub mod transaction;
pub mod block;
pub mod context;
pub mod traits;

/// Initialize logging (if needed).
pub fn init_logging() {
    // Logging initialization logic (if applicable)
}

#[cfg(test)]
pub mod tests {
    // Test module body
}