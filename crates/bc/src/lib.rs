//! # Best-Chain Protocol (BC)
//!
//! The `bc` crate implements the core types and logic for the Best-Chain (BC) protocol,
//! including blocks, transactions, and context management.
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