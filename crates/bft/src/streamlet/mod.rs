//! # Streamlet Module
//!
//! This module implements the Streamlet protocol for Byzantine Fault Tolerance (BFT).
//!
//! Streamlet is a BFT protocol that ensures safety and liveness in the presence of faulty nodes.
//! It includes the following components:
//! - **Genesis Block**: Represents the initial block in the protocol.
//! - **Proposal**: Represents a proposal for a new block.
//! - **Block**: Represents a finalized block in the protocol.
//!
//! ## Key Features
//! - **Genesis Creation**: Creates the genesis block with a specified number of nodes (`n`) and threshold (`t`).
//! - **Proposal and Block Creation**: Proposals are created, signed, and finalized into blocks.
//! - **Notarization**: Proposals are notarized when they receive enough signatures.
//! - **Finalization**: Finalized blocks are added to the chain.

/// Represents the genesis block in the Streamlet protocol.
/// 
/// The genesis block is created with a specified number of nodes (`n`) and a threshold (`t`).
/// 
/// ## Fields
/// - `n`: The total number of nodes in the network.
/// - `t`: The maximum number of faulty nodes tolerated by the protocol.

use std::collections::HashSet;
use crate::*; // Import everything from the parent module (`bft/src/lib.rs`)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletGenesis {
    pub n: usize,
    pub t: usize,
}

impl StreamletGenesis {
    /// Creates a new Streamlet genesis block.
    /// 
    /// The threshold `t` is calculated as `(n * 2 + 2) / 3`.
    /// 
    /// ## Parameters
    /// - `n`: The total number of nodes in the network.
    /// 
    /// ## Returns
    /// A new `StreamletGenesis` instance.
    pub fn new(n: usize) -> Self {
        let t = (n * 2 + 2) / 3;
        Self { n, t }
    }

    /// Returns the last finalized block in the Streamlet protocol.
    /// 
    /// The genesis block is always the last finalized block at epoch 0.
    pub fn last_final(&self) -> PermissionedBFTEnum {
        PermissionedBFTEnum::Base(PermissionedBFTBase {
            n: self.n,
            t: self.t,
            parent: None,
        })
    }
}

/// Represents a proposal in the Streamlet protocol.
/// 
/// A `StreamletProposal` is created based on a parent block and includes an epoch and signatures.
/// 
/// ## Fields
/// - `parent`: The parent block for the proposal.
/// - `epoch`: The epoch of the proposal.
/// - `signatures`: A set of signatures from nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletProposal {
    pub parent: Box<PermissionedBFTEnum>,
    pub epoch: usize,
    pub signatures: HashSet<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletBlock {
    pub proposal: Box<StreamletProposal>,
    pub parent: Option<Box<PermissionedBFTEnum>>,
}

impl StreamletProposal {
    /// Creates a new Streamlet proposal.
    /// 
    /// The proposal's epoch must be greater than the parent's epoch.
    /// 
    /// ## Parameters
    /// - `parent`: The parent block for the proposal.
    /// - `epoch`: The epoch of the proposal.
    /// 
    /// ## Returns
    /// A new `StreamletProposal` instance.
    pub fn new(parent: Box<PermissionedBFTEnum>, epoch: usize) -> Self {
        // Match on the PermissionedBFTEnum variant to access its methods
        let parent_epoch = match *parent {
            PermissionedBFTEnum::Base(ref base) => base.epoch(),
            PermissionedBFTEnum::Block(ref block) => block.proposal.epoch(),
            PermissionedBFTEnum::Proposal(ref proposal) => proposal.epoch(),
        };

        // Ensure the new epoch is greater than the parent's epoch
        assert!(epoch > parent_epoch);

        logging::log_info(&format!(
            "Creating StreamletProposal with epoch {} and parent {:?}",
            epoch, parent
        ));

        Self { parent, epoch , signatures: HashSet::new()}
    }


    /// Returns the total number of nodes in the network.
    pub fn n(&self) -> usize {
        self.parent.n()
    }

    /// Returns the maximum number of faulty nodes tolerated by the protocol.
    pub fn t(&self) -> usize {
        self.parent.t()
    }

    /// Returns the epoch of the proposal.
    pub fn epoch(&self) -> usize {
        self.epoch
    }
    
    /// Adds a signature to the proposal.
    /// 
    /// ## Parameters
    /// - `node_id`: The ID of the node adding the signature.
    pub fn add_signature(&mut self, node_id: usize) {
        self.signatures.insert(node_id);
    }

    /// Checks if the proposal is notarized.
    /// 
    /// A proposal is notarized when it receives enough signatures (`t + 1`).
    /// 
    /// ## Returns
    /// `true` if the proposal is notarized, `false` otherwise.
    pub fn is_notarized(&self) -> bool {
        let required_signatures = self.t() + 1; // t + 1 signatures required for notarization
        self.signatures.len() >= required_signatures
    }

    /// Asserts that the proposal is notarized.
    /// 
    /// If the proposal is not notarized, this method panics with the message `"Proposal is not notarized"`.
    pub fn assert_notarized(&self) {
        assert!(self.is_notarized(), "Proposal is not notarized");
    }
}


/// Represents a block in the Streamlet protocol.
/// 
/// A `StreamletBlock` is created from a notarized proposal and includes the proposal and its parent block.
/// 
/// ## Fields
/// - `proposal`: The notarized proposal for the block.
/// - `parent`: The parent block for the block.
impl StreamletBlock {
    /// Returns the last finalized block in the Streamlet protocol.
    /// 
    /// The last finalized block is determined by traversing the chain and identifying three consecutive blocks.
    pub fn last_final(&self) -> PermissionedBFTEnum {
        logging::log_info("Calculating last_final for StreamletBlock.");

        // Let `last` be self.
        let mut last = self;

        // If there is no parent, return self.
        if last.parent.is_none() {
            return PermissionedBFTEnum::Block(last.clone());
        }

        // Let middle = last.parent.
        let middle_enum = last.parent.as_ref().unwrap();
        // We expect a Block here; if not, simply return the parent's clone.
        let mut middle = match **middle_enum {
            PermissionedBFTEnum::Block(ref b) => b,
            ref other => return other.clone(),
        };

        // If middle.parent is None, return middle.
        if middle.parent.is_none() {
            return PermissionedBFTEnum::Block(middle.clone());
        }

        // Let first = middle.parent.
        let first_enum = middle.parent.as_ref().unwrap();
        let mut first = match **first_enum {
            PermissionedBFTEnum::Block(ref b) => b,
            ref other => return other.clone(),
        };

        // Now iterate as in Python.
        loop {
            // If first.parent is None, return first.
            if first.parent.is_none() {
                return PermissionedBFTEnum::Block(first.clone());
            }
            // Check if the epochs form three consecutive values.
            if first.epoch() + 1 == middle.epoch() && middle.epoch() + 1 == last.epoch() {
                return PermissionedBFTEnum::Block(middle.clone());
            }
            // Shift the window upward:
            // new_first = first.parent, new_middle = first, new_last = middle.
            let new_first_enum = first.parent.as_ref().unwrap();
            let new_first = match **new_first_enum {
                PermissionedBFTEnum::Block(ref b) => b,
                ref other => return other.clone(),
            };

            first = new_first;
            // Update middle and last:
            let new_middle = first; // previous `first`
            let new_last = middle;  // previous `middle`
            middle = new_middle;
            last = new_last;
        }
    }

    /// Returns the epoch of the block.
    pub fn epoch(&self) -> usize {
        self.proposal.epoch()
    }
}
