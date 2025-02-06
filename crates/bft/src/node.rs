// bft/src/node.rs

use futures::future::{ready};
use std::pin::Pin;
use std::collections::HashMap;
use crate::message::Message;
use futures::FutureExt;
use std::collections::VecDeque;
use crate::logging;
use futures::{future::BoxFuture};

pub trait Node {
    /// Handles an incoming message.
    fn handle(&mut self, sender: usize, message: Message) -> BoxFuture<'static, ()>;

    /// Runs the node's main loop.
    fn run(&mut self) -> Box<dyn Iterator<Item = BoxFuture<'static, ()>> + Send + '_>;

    /// Proposes a value for consensus.
    fn propose(&mut self, value: String) -> BoxFuture<'static, ()>;

    /// Votes on a proposed value.
    fn vote(&mut self, proposal_id: usize, value: String) -> BoxFuture<'static, ()>;

    fn finalize(&mut self, value: String) -> BoxFuture<'static, Option<String>>;
}

pub struct PassiveNode {
    pub id: usize,
}

impl PassiveNode {
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

pub struct SequentialNode {
    pub id: usize,
    pub mailbox: VecDeque<(usize, Message)>, // Mailbox for incoming messages
}

impl SequentialNode {
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
    fn increment_clock(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

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

/// Represents an Byzantine node in the BFT simulation.
pub struct ByzantineNode {
    pub id: usize,
}

/// Creates a new `ByzantineNode` instance.
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