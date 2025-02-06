//! # Transactions
//!
//! This module defines the `BCTransaction` struct and related types for the Best-Chain protocol.
//!
//! Transactions can include transparent and shielded inputs/outputs, fees, and issuance.
//! They are validated against the current context to ensure correctness.

/// Represents a transaction in the Best-Chain protocol.
/// 
/// A `BCTransaction` contains transparent and shielded inputs/outputs, a fee, an anchor, and issuance.
/// 
/// ## Fields
/// - `transparent_inputs`: List of transparent inputs.
/// - `transparent_outputs`: List of transparent outputs.
/// - `shielded_inputs`: List of shielded inputs.
/// - `shielded_outputs`: List of shielded outputs.
/// - `fee`: The transaction fee.
/// - `anchor`: Optional anchor to a prior context.
/// - `issuance`: The amount of new coins issued by the transaction.

use crate::context::Spentness;
use crate::context::BCContext;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, Hash, PartialEq)]
pub struct BCTransaction {
    pub transparent_inputs: Vec<TXO>,
    pub transparent_outputs: Vec<TXO>,
    pub shielded_inputs: Vec<Note>,
    pub shielded_outputs: Vec<Note>,
    pub fee: i32,
    pub anchor: Option<BCContext>,
    pub issuance: i32,
}

/// Represents a transparent transaction output.
/// 
/// A `TXO` contains the transaction it belongs to, its index, and its value.
/// 
/// ## Fields
/// - `tx`: The transaction this output belongs to.
/// - `index`: The index of this output in the transaction.
/// - `value`: The value of this output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, Hash, PartialEq)]
pub struct TXO {
    pub tx: BCTransaction,
    pub index: usize,
    pub value: i32,
}

/// Represents a shielded note.
/// 
/// A `Note` contains a value and is used for shielded transactions.
/// 
/// ## Fields
/// - `value`: The value of the note.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, Hash, PartialEq)]
pub struct Note {
    pub value: i32,
}

// crates/bc/src/transaction.rs
impl BCTransaction {
    /// Validates the transaction against the given context.
    /// 
    /// ## Parameters
    /// - `context`: The current context to validate against.
    /// 
    /// ## Returns
    /// - `true` if the transaction is valid.
    /// - `false` otherwise.
    pub fn is_valid(&self, context: &BCContext) -> bool {
        println!("Validating transaction:");
        println!("Is Coinbase: {}", self.is_coinbase());
        println!("Fee: {}", self.fee);
        println!("Issuance: {}", self.issuance);
        
        // Check if it's a coinbase transaction
        let is_coinbase = self.is_coinbase();
        
        // Validate fee
        if !is_coinbase && self.fee < 0 {
            println!("Invalid transaction: Negative fee for non-coinbase transaction");
            return false;
        }
        
        // Validate issuance
        if !is_coinbase && self.issuance != 0 {
            println!("Invalid transaction: Non-zero issuance for non-coinbase transaction");
            return false;
        }
        
        // Check transparent inputs
        for txo in &self.transparent_inputs {
            if !context.utxo_set.contains(txo) {
                println!("Invalid transaction: Transparent input not found in UTXO set");
                return false;
            }
        }
        
        // Check shielded inputs
        for note in &self.shielded_inputs {
            if !context.notes.contains_key(note) || context.notes.get(note) != Some(&Spentness::Unspent) {
                println!("Invalid transaction: Shielded input not found or already spent");
                return false;
            }
        }
        
        // Check if the transaction's anchor is valid
        if !self.shielded_inputs.is_empty() {
            if let Some(context) = &self.anchor {
                if !context.can_spend(&self.shielded_inputs) {
                    println!("Invalid transaction: Cannot spend shielded inputs");
                    return false;
                }
            } else {
                println!("Invalid transaction: No anchor provided for shielded inputs");
                return false;
            }
        }
        
        println!("Transaction is valid");
        true
    }
    
    /// Checks if the transaction is a coinbase transaction.
    /// 
    /// A coinbase transaction has no transparent or shielded inputs.
    fn is_coinbase(&self) -> bool {
        self.transparent_inputs.is_empty() && self.shielded_inputs.is_empty()
    }
}
