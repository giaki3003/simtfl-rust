// bft/src/event_queue.rs

use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Ordering};
use crate::message::Message;

#[derive(Debug, Clone)]
pub struct Event {
    pub timestamp: u64, // Logical timestamp
    pub sender: usize,  // Sender node ID
    pub receiver: usize, // Receiver node ID
    pub message: Message, // Message content
}

// Implement ordering for the event queue (min-heap)
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make it a min-heap
        other.timestamp.cmp(&self.timestamp)
    }
}

pub struct EventQueue {
    queue: BinaryHeap<Event>,
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, event: Event) {
        self.queue.push(event);
    }

    pub fn process_next_event(&mut self) -> Option<Event> {
        self.queue.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
