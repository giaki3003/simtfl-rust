//! # Byzantine Fault Tolerance (BFT)
//!
//! The `bft` crate provides a simulation framework for Byzantine Fault Tolerance (BFT) protocols.
//! It includes modules for nodes, networks, and simulations.
//!
//! ## Features
//! - Event-driven simulation
//! - Logical clocks for causal ordering
//! - Support for honest, Byzantine, and passive nodes

use std::fmt::Debug;
use util::logging;

// Import the `streamlet` module
pub mod streamlet;
pub mod network;
pub mod node;
pub mod simulation;
pub mod message;
pub mod event_queue;

pub trait PermissionedBFT: Debug + Clone {}

pub trait LastFinal {
    fn last_final(self) -> PermissionedBFTEnum;
}

#[derive(Debug, Clone)]
pub enum PermissionedBFTEnum {
    Base( PermissionedBFTBase),
    Block( streamlet::StreamletBlock), // Ensure `streamlet` is imported correctly
    Proposal( streamlet::StreamletProposal),
}

impl PermissionedBFT for PermissionedBFTEnum {}

impl LastFinal for PermissionedBFTEnum {
    fn last_final(self) -> PermissionedBFTEnum {
        match self {
            PermissionedBFTEnum::Base(base) => base.last_final(),
            PermissionedBFTEnum::Block(block) => block.last_final(),
            PermissionedBFTEnum::Proposal(proposal) => proposal.parent.last_final(),
        }
    }
}

// Implement PartialEq for PermissionedBFTEnum
impl PartialEq for PermissionedBFTEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PermissionedBFTEnum::Base(a), PermissionedBFTEnum::Base(b)) => a == b,
            (PermissionedBFTEnum::Block(a), PermissionedBFTEnum::Block(b)) => a == b,
            (PermissionedBFTEnum::Proposal(a), PermissionedBFTEnum::Proposal(b)) => a == b,
            _ => false,
        }
    }
}

// Implement Eq for PermissionedBFTEnum
impl Eq for PermissionedBFTEnum {}

// Define the PermissionedBFTBase struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionedBFTBase {
    pub n: usize,
    pub t: usize,
    pub parent: Option<Box<PermissionedBFTEnum>>,
}

impl PermissionedBFTBase {
    pub fn last_final(&self) -> PermissionedBFTEnum {
        PermissionedBFTEnum::Base(self.clone())
    }

    pub fn epoch (&self) -> usize {
        0 // The genesis block has always epoch 0
    }
}

impl PermissionedBFTEnum {
    pub fn n(&self) -> usize {
        match self {
            PermissionedBFTEnum::Base(base) => base.n,
            PermissionedBFTEnum::Block(block) => block.proposal.n(),
            PermissionedBFTEnum::Proposal(proposal) => proposal.n(),
        }
    }

    pub fn t(&self) -> usize {
        match self {
            PermissionedBFTEnum::Base(base) => base.t,
            PermissionedBFTEnum::Block(block) => block.proposal.t(),
            PermissionedBFTEnum::Proposal(proposal) => proposal.t(),
        }
    }

    pub fn epoch(&self) -> usize {
        match self {
            PermissionedBFTEnum::Base(_) => 0,
            PermissionedBFTEnum::Block(block) => block.proposal.epoch(),
            PermissionedBFTEnum::Proposal(proposal) => proposal.epoch(),
        }
    }

    pub fn parent(&self) -> Option<&PermissionedBFTEnum> {
        match self {
            PermissionedBFTEnum::Base(_) => None,
            PermissionedBFTEnum::Block(block) => block.parent.as_deref(),
            PermissionedBFTEnum::Proposal(proposal) => Some(&proposal.parent),
        }
    }
}
