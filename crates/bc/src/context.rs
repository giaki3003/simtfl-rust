/// Adds a transaction to the context.
/// 
/// This method updates the UTXO set, notes, and total issuance based on the transaction.
/// 
/// ## Parameters
/// - `tx`: The transaction to add.
/// 
/// ## Returns
/// - `true` if the transaction is valid and successfully added.
/// - `false` if the transaction is invalid.
use serde::{Serialize, Deserialize};
use std::collections::{HashSet, HashMap};
use std::hash::Hash;

/// Context for a best-chain protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
pub struct BCContext {
    /// Transactions in this context
    pub transactions: Vec<super::transaction::BCTransaction>,
    /// UTXO set
    pub utxo_set: HashSet<super::transaction::TXO>,
    /// Notes and their spentness status
    pub notes: HashMap<super::transaction::Note, Spentness>,
    /// Total issuance
    pub total_issuance: i32,
}

impl Default for BCContext {
    fn default() -> Self {
        Self::new()
    }
}

impl BCContext {
    /// Create a new `BCContext`.
    pub fn new() -> Self {
        BCContext {
            transactions: Vec::new(),
            utxo_set: HashSet::new(),
            notes: HashMap::new(),
            total_issuance: 0,
        }
    }

    pub fn add_transaction(&mut self, tx: super::transaction::BCTransaction) -> bool {
        if !tx.is_valid(self) {
            return false;
        }

        // Update UTXO set
        for txo in &tx.transparent_inputs {
            self.utxo_set.remove(txo);
        }

        for txo in &tx.transparent_outputs {
            self.utxo_set.insert(txo.clone());
        }

        // Update shielded notes
        for note in &tx.shielded_inputs {
            if let Some(entry) = self.notes.get_mut(note) {
                *entry = Spentness::Spent;
            }
        }

        for note in &tx.shielded_outputs {
            self.notes.insert(note.clone(), Spentness::Unspent);
        }

        // Update total issuance
        self.total_issuance += tx.issuance;

        // Add the transaction to the list
        self.transactions.push(tx);

        true
    }

    /// Check if all notes in the given slice are unspent.
    pub fn can_spend(&self, notes: &[super::transaction::Note]) -> bool {
        notes.iter().all(|note| {
            self.notes.get(note).is_some_and(|spent| *spent == Spentness::Unspent)
        })
    }

    /// Check if a note is spent.
    pub fn is_spent(&self, note: &super::transaction::Note) -> bool {
        self.notes.get(note).is_some_and(|s| *s == Spentness::Spent)
    }

    /// Copy the context (for forks).
    pub fn copy(&self) -> Self {
        Self {
            transactions: self.transactions.clone(),
            utxo_set: self.utxo_set.clone(),
            notes: self.notes.clone(),
            total_issuance: self.total_issuance,
        }
    }
}

impl Hash for BCContext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Implement the hash function for BCContext
        // For simplicity, hash the total_issuance field
        self.total_issuance.hash(state);
    }
}

/// Spentness status of a note.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Eq, PartialEq)]
pub enum Spentness {
    /// The note is unspent.
    Unspent,
    /// The note is spent.
    Spent,
}
