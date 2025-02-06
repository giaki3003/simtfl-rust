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

/// Transparent transaction output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, Hash, PartialEq)]
pub struct TXO {
    pub tx: BCTransaction,
    pub index: usize,
    pub value: i32,
}

/// Shielded note
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, Hash, PartialEq)]
pub struct Note {
    pub value: i32,
}

// crates/bc/src/transaction.rs
impl BCTransaction {
    /// Check if the transaction is valid.
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
    
    /// Check if the transaction is a coinbase transaction.
    fn is_coinbase(&self) -> bool {
        self.transparent_inputs.is_empty() && self.shielded_inputs.is_empty()
    }
}
