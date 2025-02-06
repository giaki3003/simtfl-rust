// bft/src/simulation.rs

use crate::logging;
use crate::message::Message;
use crate::node::Node;
use crate::network::Network;
use std::sync::{Arc, Mutex};

pub struct Simulation {
    // Wrap Network in a Mutex to allow mutable access behind the Arc.
    pub network: Arc<Mutex<Network>>,
    pub nodes: Vec<Arc<Mutex<dyn Node + Send + Sync>>>,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            network: Arc::new(Mutex::new(Network::new())),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: impl Node + Send + Sync + 'static) {
        // Lock network for mutable access to add the node.
        let node_id = {
            let mut network = self.network.lock().unwrap();
            network.add_node()
        };

        let node_arc = Arc::new(Mutex::new(node));

        {
            // Initialize the node.
            let mut node_lock = node_arc.lock().unwrap();
            std::mem::drop(node_lock.handle(
                0,
                Message {
                    content: format!("Node {} initialized.", node_id),
                    timestamp: 0,
                },
            ));
        }

        self.nodes.push(node_arc);
    }

    pub async fn start(&mut self) {
        logging::log_info("Starting BFT simulation...");

        // Continue processing until the event queue is empty.
        loop {
            {
                // Check if the event queue is empty.
                let network = self.network.lock().unwrap();
                if network.event_queue.is_empty() {
                    break;
                }
            }

            // Lock the network mutably to process events.
            {
                let mut network = self.network.lock().unwrap();
                network.process_events();
            }

            // Run each node's main loop.
            for node in &self.nodes {
                // Collect the async effects produced by the node's run.
                let effects: Vec<_> = {
                    let mut node_lock = node.lock().unwrap();
                    node_lock.run().collect()
                };

                // Await each effect.
                for effect in effects {
                    effect.await;
                }
            }
        }

        logging::log_info("BFT simulation completed.");
    }
}
