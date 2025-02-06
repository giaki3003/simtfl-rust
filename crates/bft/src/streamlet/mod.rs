// bft/src/streamlet/lib.rs

use std::collections::HashSet;
use crate::*; // Import everything from the parent module (`bft/src/lib.rs`)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamletGenesis {
    pub n: usize,
    pub t: usize,
}

impl StreamletGenesis {
    pub fn new(n: usize) -> Self {
        let t = (n * 2 + 2) / 3;
        Self { n, t }
    }

    pub fn last_final(&self) -> PermissionedBFTEnum {
        PermissionedBFTEnum::Base(PermissionedBFTBase {
            n: self.n,
            t: self.t,
            parent: None,
        })
    }
}

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

    pub fn n(&self) -> usize {
        self.parent.n()
    }

    pub fn t(&self) -> usize {
        self.parent.t()
    }

    pub fn epoch(&self) -> usize {
        self.epoch
    }
    
    /// Add a signature from a specific node.
    pub fn add_signature(&mut self, node_id: usize) {
        self.signatures.insert(node_id);
    }

    /// Check if the proposal is notarized (has enough signatures).
    pub fn is_notarized(&self) -> bool {
        let required_signatures = self.t() + 1; // t + 1 signatures required for notarization
        self.signatures.len() >= required_signatures
    }

    /// Assert that the proposal is notarized.
    pub fn assert_notarized(&self) {
        assert!(self.is_notarized(), "Proposal is not notarized");
    }
}

impl StreamletBlock {
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

    pub fn epoch(&self) -> usize {
        self.proposal.epoch()
    }
}
