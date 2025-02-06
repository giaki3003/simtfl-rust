//! # Node Module
//!
//! This module defines the behavior of nodes in the BFT simulation.
//!
//! Nodes can be honest, Byzantine, or passive. Each node implements the `Node` trait, which defines methods for handling messages, proposing values, voting, and finalizing values.

use futures::future::{ready};
use std::pin::Pin;
use std::collections::HashMap;
use crate::message::Message;
use futures::FutureExt;
use std::collections::VecDeque;
use crate::logging;
use futures::{future::BoxFuture};

/// Trait defining the behavior of a node in the BFT simulation.
/// 
/// ## Methods
/// - `handle`: Handles an incoming message.
/// - `run`: Runs the node's main loop.
/// - `propose`: Proposes a value for consensus.
/// - `vote`: Votes on a proposed value.
/// - `finalize`: Finalizes a value.
pub trait Node {
    /// Handles an incoming message.
    /// 
    /// ## Parameters
    /// - `sender`: The ID of the sender node.
    /// - `message`: The message to handle.
    /// 
    /// ## Returns
    /// A future that resolves when the message is processed.
    fn handle(&mut self, sender: usize, message: Message) -> BoxFuture<'static, ()>;

    /// Runs the node's main loop.
    /// 
    /// ## Returns
    /// An iterator over futures representing the node's operations.
    fn run(&mut self) -> Box<dyn Iterator<Item = BoxFuture<'static, ()>> + Send + '_>;

    /// Proposes a value for consensus.
    /// 
    /// ## Parameters
    /// - `value`: The value to propose.
    /// 
    /// ## Returns
    /// A future that resolves when the proposal is processed.
    fn propose(&mut self, value: String) -> BoxFuture<'static, ()>;

    /// Votes on a proposed value.
    /// 
    /// ## Parameters
    /// - `proposal_id`: The ID of the proposal.
    /// - `value`: The value to vote for.
    /// 
    /// ## Returns
    /// A future that resolves when the vote is processed.
    fn vote(&mut self, proposal_id: usize, value: String) -> BoxFuture<'static, ()>;

    /// Finalizes a value.
    /// 
    /// ## Parameters
    /// - `value`: The value to finalize.
    /// 
    /// ## Returns
    /// A future that resolves with the finalized value, or `None` if finalization fails.
    fn finalize(&mut self, value: String) -> BoxFuture<'static, Option<String>>;
}

/// Represents a passive node in the BFT simulation.
/// 
/// Passive nodes do not actively participate in the consensus process but can still receive and log messages.
pub struct PassiveNode {
    pub id: usize,
}

impl PassiveNode {
    /// Creates a new passive node.
    /// 
    /// ## Parameters
    /// - `id`: The unique ID of the node.
    /// 
    /// ## Returns
    /// A new `PassiveNode` instance.
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Node for PassiveNode {
    fn handle(&mut self, _sender: usize, message: Message) -> BoxFuture<'static, ()> {
        // Copy the id so that nothing with a short lifetime is captured.
        let id = self.id;
        Box::pin(async move {
            logging::log_info(&format!(
                "Node {} received message: {}",
                id, message.content
            ));
        })
    }

    fn run(&mut self) -> Box<dyn Iterator<Item = BoxFuture<'static, ()>> + Send + '_> {
        // For a passive node, we simply return an empty iterator.
        Box::new(std::iter::empty())
    }
fn propose(&mut self, _: String) -> Pin<Box<(dyn futures::Future<Output = ()> + std::marker::Send + 'static)>> { todo!() }
fn vote(&mut self, _: usize, _: String) -> Pin<Box<(dyn futures::Future<Output = ()> + std::marker::Send + 'static)>> { todo!() }
fn finalize(&mut self, _: String) -> Pin<Box<(dyn futures::Future<Output = Option<String>> + std::marker::Send + 'static)>> { todo!() }
}

/// Represents a sequential node in the BFT simulation.
/// 
/// Sequential nodes process messages in the order they are received.
pub struct SequentialNode {
    pub id: usize,
    pub mailbox: VecDeque<(usize, Message)>, // Mailbox for incoming messages
}

impl SequentialNode {
    /// Creates a new sequential node.
    /// 
    /// ## Parameters
    /// - `id`: The unique ID of the node.
    /// 
    /// ## Returns
    /// A new `SequentialNode` instance.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            mailbox: VecDeque::new(),
        }
    }
}

impl Node for SequentialNode {
    // When a message is received, push it into the mailbox.
    // We immediately return a future that resolves to ().
    fn handle(&mut self, sender: usize, message: Message) -> BoxFuture<'static, ()> {
        self.mailbox.push_back((sender, message));
        async {}.boxed() // Return an immediately-ready future.
    }

    // The run method returns an iterator over futures. Each future, when awaited,
    // processes a message from the mailbox (if any).
    fn run(&mut self) -> Box<dyn Iterator<Item = BoxFuture<'static, ()>> + Send + '_> {
        Box::new(std::iter::from_fn(move || {
            if let Some((sender, message)) = self.mailbox.pop_front() {
                // Capture the node's id to use inside the async block.
                let id = self.id;
                // Create a future that logs the message processing.
                let future = async move {
                    // For example, log the message handling.
                    // Replace `logging::log_info` with your own logging function.
                    logging::log_info(&format!(
                        "Node {} handling message from {}: {}",
                        id, sender, message.content
                    ));
                }
                .boxed();
                Some(future)
            } else {
                None
            }
        }))
    }
fn propose(&mut self, _: String) -> Pin<Box<(dyn futures::Future<Output = ()> + std::marker::Send + 'static)>> { todo!() }
fn vote(&mut self, _: usize, _: String) -> Pin<Box<(dyn futures::Future<Output = ()> + std::marker::Send + 'static)>> { todo!() }
fn finalize(&mut self, _: String) -> Pin<Box<(dyn futures::Future<Output = Option<String>> + std::marker::Send + 'static)>> { todo!() }
}

/// Represents an honest node in the BFT simulation.
/// 
/// Honest nodes actively participate in the consensus process by proposing values, voting, and finalizing values.
pub struct HonestNode {
    pub id: usize,
    pub mailbox: VecDeque<(usize, Message)>, // Mailbox for incoming messages
    pub proposals: Vec<String>,             // List of proposed values
    pub votes: HashMap<usize, Vec<String>>, // Votes for each proposal
    pub finalized: Option<String>,          // Finalized value
    pub clock: u64,                         // Logical clock
}

/// Creates a new `HonestNode` instance.
impl HonestNode {
    /// Creates a new honest node.
    /// 
    /// ## Parameters
    /// - `id`: The unique ID of the node.
    /// 
    /// ## Returns
    /// A new `HonestNode` instance.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            mailbox: VecDeque::new(),
            proposals: Vec::new(),
            votes: HashMap::new(),
            finalized: None,
            clock: 0,
        }
    }

    /// Increments the logical clock.
    /// 
    /// ## Returns
    /// The updated logical clock value.
    fn increment_clock(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

    /// Updates the logical clock based on another node's clock.
    /// 
    /// ## Parameters
    /// - `other_clock`: The logical clock value of another node.
    fn update_clock(&mut self, other_clock: u64) {
        self.clock = self.clock.max(other_clock) + 1;
    }
}

impl Node for HonestNode {
    fn handle(&mut self, sender: usize, message: Message) -> BoxFuture<'static, ()> {
        // Update the logical clock
        self.update_clock(message.timestamp);

        self.mailbox.push_back((sender, message));
        async {}.boxed()
    }

    fn run(&mut self) -> Box<dyn Iterator<Item = BoxFuture<'static, ()>> + Send + '_> {
        Box::new(std::iter::from_fn(move || {
            if let Some((sender, message)) = self.mailbox.pop_front() {
                // Simulate processing the message (e.g., voting or finalizing)
                let id = self.id;
                let future = async move {
                    logging::log_info(&format!(
                        "Node {} handling message from {}: {}",
                        id, sender, message.content
                    ));
                }
                .boxed();
                Some(future)
            } else {
                None
            }
        }))
    }

    fn propose(&mut self, value: String) -> BoxFuture<'static, ()> {
        let _timestamp = self.increment_clock();
        self.proposals.push(value.clone());
        logging::log_info(&format!("Node {} proposing value: {}", self.id, value));
        async {}.boxed()
    }

    fn vote(&mut self, proposal_id: usize, value: String) -> BoxFuture<'static, ()> {
        self.votes.entry(proposal_id).or_default().push(value.clone());
        logging::log_info(&format!(
            "Node {} voting for proposal {}: {}",
            self.id, proposal_id, value
        ));
        async {}.boxed()
    }

    fn finalize(&mut self, proposal: String) -> BoxFuture<'static, Option<String>> {
        // Update your node’s state immediately.
        self.finalized = Some(proposal.clone());
        // Return a future that is immediately ready with the value.
        ready(Some(proposal)).boxed()
    }
}

/// Represents a Byzantine node in the BFT simulation.
/// 
/// Byzantine nodes may behave adversarially by ignoring messages, sending conflicting responses, or refusing to finalize values.
pub struct ByzantineNode {
    pub id: usize,
}

/// Creates a new Byzantine node.
/// 
/// ## Parameters
/// - `id`: The unique ID of the node.
/// 
/// ## Returns
/// A new `ByzantineNode` instance.
impl ByzantineNode {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Node for ByzantineNode {
    fn handle(&mut self, _sender: usize, _message: Message) -> BoxFuture<'static, ()> {
        // Byzantine nodes may ignore messages or send conflicting responses
        async {}.boxed()
    }

    fn run(&mut self) -> Box<dyn Iterator<Item = BoxFuture<'static, ()>> + Send + '_> {
        Box::new(std::iter::empty())
    }

    fn propose(&mut self, value: String) -> BoxFuture<'static, ()> {
        // Byzantine nodes may propose conflicting values
        logging::log_info(&format!(
            "Byzantine Node {} proposing conflicting value: {}",
            self.id, value
        ));
        async {}.boxed()
    }

    fn vote(&mut self, proposal_id: usize, value: String) -> BoxFuture<'static, ()> {
        // Byzantine nodes may vote inconsistently
        logging::log_info(&format!(
            "Byzantine Node {} voting inconsistently for proposal {}: {}",
            self.id, proposal_id, value
        ));
        async {}.boxed()
    }

    fn finalize(&mut self, value: String) -> BoxFuture<'static, Option<String>> {
        logging::log_info(&format!(
            "Byzantine Node {} refusing to finalize value: {}",
            self.id, value
        ));
        ready(None).boxed()
    }
}