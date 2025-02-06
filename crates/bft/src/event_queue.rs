//! # Event Queue Module
//!
//! This module implements an event queue for processing messages in the BFT simulation.
//!
//! The event queue is a priority queue that schedules events based on their timestamps.
//! It ensures that events are processed in the correct order, maintaining causal consistency.

use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Ordering};
use crate::message::Message;


/// Represents an event in the BFT simulation.
/// 
/// An `Event` contains a timestamp, sender ID, receiver ID, and a message.
/// 
/// ## Fields
/// - `timestamp`: The logical timestamp of the event.
/// - `sender`: The ID of the node sending the event.
/// - `receiver`: The ID of the node receiving the event.
/// - `message`: The content of the event.
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

/// Represents the event queue for the BFT simulation.
/// 
/// The `EventQueue` is implemented as a binary heap to efficiently process events in order of their timestamps.
pub struct EventQueue {
    queue: BinaryHeap<Event>,
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    /// Creates a new empty event queue.
    /// 
    /// ## Returns
    /// A new `EventQueue` instance.
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    /// Schedules an event in the queue.
    /// 
    /// ## Parameters
    /// - `event`: The event to schedule.
    pub fn schedule(&mut self, event: Event) {
        self.queue.push(event);
    }

    /// Processes the next event in the queue.
    /// 
    /// ## Returns
    /// The next event in the queue, or `None` if the queue is empty.
    pub fn process_next_event(&mut self) -> Option<Event> {
        self.queue.pop()
    }

    /// Checks if the queue is empty.
    /// 
    /// ## Returns
    /// `true` if the queue is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
