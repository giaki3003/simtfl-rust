//! # Network Module
//!
//! The `network` module implements the communication layer for the BFT simulation.
//! It includes functionality for sending and receiving messages between nodes.

use crate::event_queue::{Event, EventQueue};
use crate::message::Message;
use log::error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender, Receiver};

/// The Network struct facilitates communication between nodes.
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
    /// Creates a new Network instance.
    pub fn new() -> Self {
        Self {
            senders: Arc::new(Mutex::new(HashMap::new())),
            receivers: Arc::new(Mutex::new(HashMap::new())),
            event_queue: EventQueue::new(),
        }
    }

    /// Adds a node to the network and returns its unique ID.
    pub fn add_node(&mut self) -> usize {
        let mut senders = self.senders.lock().unwrap();
        let mut receivers = self.receivers.lock().unwrap();

        let id = senders.len();
        let (tx, rx) = unbounded();
        senders.insert(id, tx);
        receivers.insert(id, rx);

        id
    }

    /// Sends a message from `sender_id` to `target_id` with a specified delay.
    ///
    /// Note: This method requires mutable access because it schedules events.
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
    /// This function uses the sender channel (tx) for the target node.
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

    /// Receives a message for the given `node_id`, blocking until a message is available.
    pub fn receive(&self, node_id: usize) -> Option<Message> {
        let receivers = self.receivers.lock().unwrap();
        if let Some(rx) = receivers.get(&node_id) {
            rx.recv().ok()
        } else {
            None
        }
    }
}
