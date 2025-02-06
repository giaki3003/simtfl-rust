//! # Network Module
//!
//! This module implements the communication layer for the BFT simulation.
//!
//! The `Network` struct facilitates communication between nodes by managing sender and receiver channels.
//! It also includes an event queue for scheduling and processing messages with logical timestamps.


use crate::event_queue::{Event, EventQueue};
use crate::message::Message;
use log::error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};

/// Represents the network for the BFT simulation.
/// 
/// The `Network` struct manages sender and receiver channels for nodes and includes an event queue for processing messages.
/// 
/// ## Fields
/// - `senders`: A map of sender channels for each node.
/// - `receivers`: A map of receiver channels for each node.
/// - `event_queue`: The event queue for scheduling and processing messages.
pub struct Network {
    // The sender channels map.
    senders: Arc<Mutex<HashMap<usize, Sender<Message>>>>,
    // The receiver channels map.
    receivers: Arc<Mutex<HashMap<usize, Receiver<Message>>>>,
    pub event_queue: EventQueue,
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

impl Network {
    /// Creates a new network.
    /// 
    /// Initializes empty sender and receiver maps and an empty event queue.
    /// 
    /// ## Returns
    /// A new `Network` instance.
    pub fn new() -> Self {
        Self {
            senders: Arc::new(Mutex::new(HashMap::new())),
            receivers: Arc::new(Mutex::new(HashMap::new())),
            event_queue: EventQueue::new(),
        }
    }

    /// Adds a new node to the network and assigns it a unique ID.
    /// 
    /// ## Returns
    /// The unique ID assigned to the new node.
    pub fn add_node(&mut self) -> usize {
        let mut senders = self.senders.lock().unwrap();
        let mut receivers = self.receivers.lock().unwrap();

        let id = senders.len();
        let (tx, rx) = unbounded();
        senders.insert(id, tx);
        receivers.insert(id, rx);

        id
    }

    /// Sends a message from one node to another with a specified delay.
    /// 
    /// The message is scheduled in the event queue with the specified delay.
    /// 
    /// ## Parameters
    /// - `sender_id`: The ID of the sender node.
    /// - `target_id`: The ID of the target node.
    /// - `message`: The message to send.
    /// - `delay`: The delay (in logical time units) before the message is delivered.
    pub fn send(&mut self, sender_id: usize, target_id: usize, mut message: Message, delay: u64) {
        let senders = self.senders.lock().unwrap();
        if senders.get(&sender_id).is_some() {
            let receivers = self.receivers.lock().unwrap();
            if receivers.get(&target_id).is_some() {
                // Assign a logical timestamp to the message.
                message.timestamp += delay;

                // Schedule the message in the event queue.
                let event = Event {
                    timestamp: message.timestamp,
                    sender: sender_id,
                    receiver: target_id,
                    message,
                };

                self.event_queue.schedule(event);
            } else {
                error!("Error: Target node {} does not exist.", target_id);
            }
        } else {
            error!("Error: Sender node {} does not exist.", sender_id);
        }
    }

    /// Processes all scheduled events in the event queue.
    /// 
    /// Messages are delivered to their respective receivers based on their timestamps.
    pub fn process_events(&mut self) {
        while let Some(event) = self.event_queue.process_next_event() {
            let senders = self.senders.lock().unwrap();
            if let Some(tx) = senders.get(&event.receiver) {
                tx.send(event.message).unwrap();
            } else {
                error!("Error: Receiver node {} does not exist.", event.receiver);
            }
        }
    }

    /// Receives a message for the given node.
    /// 
    /// This method blocks until a message is available for the specified node.
    /// 
    /// ## Parameters
    /// - `node_id`: The ID of the node receiving the message.
    /// 
    /// ## Returns
    /// The received message, or `None` if no message is available.
    pub fn receive(&self, node_id: usize) -> Option<Message> {
        let receivers = self.receivers.lock().unwrap();
        if let Some(rx) = receivers.get(&node_id) {
            rx.recv().ok()
        } else {
            None
        }
    }
}
