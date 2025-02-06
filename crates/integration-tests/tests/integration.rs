// tests/integration.rs
#[cfg(test)]
mod tests {
    use bc::context::BCContext;
    use bc::transaction::{BCTransaction, TXO, Note};
    use bft::simulation::Simulation;
    use bft::node::{HonestNode, ByzantineNode};
    use bft::message::Message;
    use async_std::task;
    use util::logging;

    #[test]
    fn test_integration_bft_and_bc() {
        // Initialize the logger
        logging::init_logger();

        // Step 1: Create a BCContext
        let mut ctx = BCContext::new();

        // Step 2: Create a coinbase transaction
        let coinbase_tx = BCTransaction {
            transparent_inputs: Vec::new(),
            transparent_outputs: vec![TXO {
                tx: BCTransaction {
                    transparent_inputs: Vec::new(),
                    transparent_outputs: Vec::new(),
                    shielded_inputs: Vec::new(),
                    shielded_outputs: Vec::new(),
                    fee: 0,
                    anchor: None,
                    issuance: 10,
                },
                index: 0,
                value: 10,
            }],
            shielded_inputs: Vec::new(),
            shielded_outputs: vec![Note { value: 5 }],
            fee: 0,
            anchor: None,
            issuance: 10,
        };

        // Step 3: Add the transaction to the context
        assert!(ctx.add_transaction(coinbase_tx.clone()));

        // Step 4: Create a BFT simulation
        let mut simulation = Simulation::new();

        // Step 5: Add nodes to the simulation
        let honest_node = HonestNode::new(0);
        let byzantine_node = ByzantineNode::new(1);

        simulation.add_node(honest_node);
        simulation.add_node(byzantine_node);

        task::block_on(async {
            // Create the future while the lock is held...
            let propose_future = {
                let mut node_lock = simulation.nodes[0].lock().unwrap();
                node_lock.propose("Block Proposal".to_string())
                // The lock guard is dropped here at the end of the block.
            };
            // Now await the future without holding the lock.
            propose_future.await;
        });

        // Step 7: Send a message from one node to another.
        // First, get a clone of the network.
        let network = simulation.network.clone();

        // Add two more nodes to the network.
        let node1_id = {
            let mut net = network.lock().unwrap();
            net.add_node()
        };
        let node2_id = {
            let mut net = network.lock().unwrap();
            net.add_node()
        };

        let message = Message {
            content: "Propose Block".to_string(),
            timestamp: 0,
        };

        {
            // Lock the network to send the message.
            let mut net = network.lock().unwrap();
            net.send(node1_id, node2_id, message.clone(), 100);
            // Process events to deliver the scheduled message.
            net.process_events();
        }

        // Step 8: Verify the message was received.
        let received_message = {
            let net = network.lock().unwrap();
            net.receive(node2_id)
        };

        if let Some(received_message) = received_message {
            logging::log_info(&format!(
                "Node {} received message: {}",
                node2_id, received_message.content
            ));
            assert_eq!(received_message.content, "Propose Block");
        } else {
            logging::log_error("No message received.");
            panic!("No message received.");
        }

        // Step 9: Run the simulation.
        logging::log_info("Starting BFT simulation...");
        task::block_on(simulation.start());

        // Step 10: Verify the finalized value.
        let finalize_future = {
            let mut node = simulation.nodes[0].lock().unwrap();
            node.finalize("Block Proposal".to_string())
        };

        if let Some(finalized_value) = task::block_on(finalize_future) {
            logging::log_info(&format!(
                "Node {} finalized value: {}",
                0, finalized_value
            ));
            assert_eq!(finalized_value, "Block Proposal".to_string());
        } else {
            logging::log_error("No value finalized.");
            panic!("No value finalized.");
        }
    }
}
