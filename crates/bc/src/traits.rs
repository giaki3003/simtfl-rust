// src/traits.rs
use crate::transaction::BCTransaction;
use crate::transaction::TXO;
use crate::transaction::Note;
use crate::context::BCContext;


/// Traits for best-chain protocol components.
pub trait TransactionTrait {
    /// Get the transparent inputs.
    fn transparent_inputs(&self) -> &[TXO];
    /// Get the transparent outputs.
    fn transparent_outputs(&self) -> &[TXO];
    /// Get the shielded inputs.
    fn shielded_inputs(&self) -> &[Note];
    /// Get the shielded outputs.
    fn shielded_outputs(&self) -> &[Note];
    /// Get the fee.
    fn fee(&self) -> i32;
    /// Get the anchor (if any).
    fn anchor(&self) -> Option<&BCContext>;
    /// Get the issuance.
    fn issuance(&self) -> i32;
}

/// Traits for best-chain protocol components.
pub trait ContextTrait {
    /// Add a transaction to the context.
    fn add_transaction(&mut self, tx: BCTransaction) -> bool;
    /// Check if a note is spent.
    fn is_spent(&self, note: &Note) -> bool;
    /// Copy the context (for forks).
    fn copy(&self) -> Self;
}
