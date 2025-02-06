#[cfg(test)]
mod tests {
    
    use futures::executor::block_on;
    use std::sync::Mutex;
    use async_std::sync::Arc;
    use bft::event_queue::Event;
    use bft::event_queue::EventQueue;
    use bft::network::Network;
    
    use util::logging;
    use bft::streamlet::StreamletGenesis;
    use bft::streamlet::StreamletProposal;
    use bft::PermissionedBFTEnum;
    use bft::PermissionedBFTBase;
    use bft::streamlet::StreamletBlock;
    use bft::node::*;
    use bft::message::Message;

    #[test]
    fn test_logging() {
        // Initialize the logger with the INFO level
        logging::init_logger();

        // Log messages at different levels
        logging::log_info("This is an info message.");
        logging::log_warn("This is a warning message.");
        logging::log_error("This is an error message.");
        logging::log_debug("This is a debug message."); // This won't appear unless the log level is DEBUG or lower
    }

    /// Test that a block created without any parent returns itself.
    #[test]
    fn test_streamlet() {
        // Create a genesis block for a workspace with 3 nodes.
        // For example, StreamletGenesis::new(3) creates a genesis with n=3 and t=2 (if calculated as (3*2+2)/3).
        let _genesis = StreamletGenesis::new(3);
        // Create a proposal on top of an explicit genesis base.
        let proposal1 = StreamletProposal::new(
            Box::new(PermissionedBFTEnum::Base(PermissionedBFTBase {
                n: 3,
                t: 2,
                parent: None,
            })),
            1, // epoch 1
        );
        // Create a block from the proposal, with no parent.
        let block1 = StreamletBlock {
            proposal: Box::new(proposal1),
            parent: None,
        };

        // In the Python code, if a block's parent is None, last_final() returns self.
        // Therefore, we expect block1.last_final() to equal block1.
        assert_eq!(
            block1.last_final(),
            PermissionedBFTEnum::Block(block1.clone())
        );
    }

    /// Test a chain built on top of genesis.
    #[test]
    fn test_streamlet_basic() {
        // Construct the genesis block.
        // Using 5 nodes so that t is computed (for example, (5*2+2)/3) and genesis always has epoch 0.
        let genesis = StreamletGenesis::new(5);
        // genesis.last_final() should return a Base wrapping the genesis block.
        let genesis_bft = genesis.last_final();
        let current = genesis_bft.clone();

        // Verify that the genesis block is final and has epoch 0.
        assert_eq!(current, genesis_bft);
        assert_eq!(current.epoch(), 0);

        // Build one new block on top of genesis.
        // (In Python, with only one new block, the chain is too short for an update,
        // so last_final() should return the genesis block.)
        {
            // Create a new proposal with epoch = parent's epoch + 1 (i.e. 1).
            let mut proposal = StreamletProposal::new(Box::new(current.clone()), current.epoch() + 1);
            // When current is genesis (epoch 0), proposal.epoch() should be 1.
            assert_eq!(proposal.epoch(), 1);
            // Inherit parameters from genesis.
            assert_eq!(proposal.n(), genesis.n);
            assert_eq!(proposal.t(), genesis.t);

            logging::log_info(&format!("Current epoch: {}", current.epoch()));

            // Determine the number of unique signatures required for notarization.
            let required_signatures = proposal.t() + 1;
            // Add fewer signatures (all the same) so that notarization fails.
            for _ in 0..(required_signatures - 1) {
                proposal.add_signature(0);
            }
            assert!(!proposal.is_notarized());
            // Now add the required unique signatures.
            for i in 0..required_signatures {
                proposal.add_signature(i);
            }
            assert!(proposal.is_notarized());

            // Create a new block from the notarized proposal.
            // Its parent is the current final block.
            let block = StreamletBlock {
                proposal: Box::new(proposal),
                parent: Some(Box::new(current.clone())),
            };

            // According to the Python semantics:
            //   - Let last = block (epoch 1),
            //   - middle = block.parent (the genesis, epoch 0), and since genesis.parent is None,
            //     last_final() should return middle (i.e. the genesis).
            let last_final = block.last_final();
            match last_final {
                PermissionedBFTEnum::Base(ref base) => {
                    // Expect the genesis base to be returned.
                    assert_eq!(base.epoch(), 0);
                    assert_eq!(last_final, genesis_bft);
                }
                PermissionedBFTEnum::Block(ref b) => {
                    // In our desired semantics, we do not want to update finality if there's only one new block.
                    // So if a Block is returned, its epoch should be 0 (which is not the case for block with epoch 1).
                    panic!("Expected genesis finalization, got Block with epoch {}", b.epoch());
                }
                _ => unreachable!(),
            }
        }
    }

    /// Test that asserting notarization on an under-signed proposal panics.
    #[test]
    #[should_panic(expected = "Proposal is not notarized")]
    fn test_streamlet_assertions() {
        // Construct the genesis block.
        let genesis = StreamletGenesis::new(5);
        let genesis_bft = genesis.last_final();

        // Create a proposal using the genesis block as parent, with epoch 1.
        let mut proposal = StreamletProposal::new(Box::new(genesis_bft), 1);

        // Without signatures, assert_notarized should panic.
        proposal.assert_notarized();

        // After adding one signature, still not notarized.
        proposal.add_signature(0);
        proposal.assert_notarized();

        // Now add the required number of unique signatures.
        let required_signatures = proposal.t() + 1;
        for i in 1..required_signatures {
            proposal.add_signature(i);
        }
        // At this point, the proposal is notarized and assert_notarized() should succeed.
        proposal.assert_notarized();
    }

    #[test]
    fn test_message_passing() {
        // Create a new network wrapped in Arc<Mutex<...>>
        let network = Arc::new(Mutex::new(Network::new()));

        // Add two nodes by locking the network for mutable access.
        let node1_id = {
            let mut net = network.lock().unwrap();
            net.add_node()
        };
        let node2_id = {
            let mut net = network.lock().unwrap();
            net.add_node()
        };

        logging::log_info(&format!("Created nodes: {}, {}", node1_id, node2_id));

        // Create a message from node1 to node2.
        let message = Message {
            content: "Hello, node2!".to_string(),
            timestamp: 0,
        };

        // Send the message with a delay of 100ms.
        {
            let mut net = network.lock().unwrap();
            net.send(node1_id, node2_id, message.clone(), 100);
            // Process the events to deliver the scheduled message.
            net.process_events();
        }

        logging::log_info("Message sent. Waiting to receive...");

        // Try to receive the message on node2.
        let received_message = {
            let net = network.lock().unwrap();
            net.receive(node2_id)
        };

        if let Some(received_message) = received_message {
            logging::log_info(&format!(
                "Node {} received message: {}",
                node2_id, received_message.content
            ));
            assert_eq!(received_message.content, "Hello, node2!");
        } else {
            logging::log_error("No message received.");
            panic!("No message received.");
        }
    }
    #[test]
    fn test_passive_node() {
        // Create a PassiveNode with id 0.
        let mut node = PassiveNode::new(0);

        // Create a message for the node.
        let message = Message {
            content: "Hello, PassiveNode!".to_string(),
            timestamp: 1,
        };

        // Use block_on to run the asynchronous code.
        block_on(async {
            // Await the future returned by handle.
            node.handle(1, message).await;

            // Run the node's main loop by awaiting all asynchronous effects.
            for effect in node.run() {
                effect.await;
            }
        });
    }

    #[test]
    fn test_sequential_node() {
        // Create a SequentialNode with id 0.
        let mut node = SequentialNode::new(0);

        // Create two messages.
        let message1 = Message {
            content: "Message 1".to_string(),
            timestamp: 0,
        };
        let message2 = Message {
            content: "Message 2".to_string(),
            timestamp: 1,
        };

        // Use block_on to run the asynchronous code.
        block_on(async {
            // Await the futures returned by handle.
            node.handle(1, message1).await;
            node.handle(2, message2).await;

            // Run the node's main loop by awaiting each effect.
            for effect in node.run() {
                effect.await;
            }
        });
    }

    #[test]
    fn test_event_queue_ordering() {
        // Create a new event queue.
        let mut event_queue = EventQueue::new();

        // Create events with different timestamps.
        let event1 = Event {
            timestamp: 10,
            sender: 0,
            receiver: 1,
            message: Message {
                content: "Event 1".to_string(),
                timestamp: 10,
            },
        };

        let event2 = Event {
            timestamp: 5,
            sender: 1,
            receiver: 0,
            message: Message {
                content: "Event 2".to_string(),
                timestamp: 5,
            },
        };

        let event3 = Event {
            timestamp: 7,
            sender: 2,
            receiver: 3,
            message: Message {
                content: "Event 3".to_string(),
                timestamp: 7,
            },
        };

        // Schedule events in the queue.
        event_queue.schedule(event1);
        event_queue.schedule(event2);
        event_queue.schedule(event3);

        // Process events in order of their timestamps.
        let mut processed_events = Vec::new();
        while let Some(event) = event_queue.process_next_event() {
            processed_events.push(event);
        }

        // Verify the order of processed events.
        assert_eq!(processed_events.len(), 3);
        assert_eq!(processed_events[0].timestamp, 5);
        assert_eq!(processed_events[1].timestamp, 7);
        assert_eq!(processed_events[2].timestamp, 10);

        // Log the processed events.
        util::logging::log_info("Processed events:");
        for event in processed_events {
            util::logging::log_info(&format!(
                "Timestamp: {}, Content: {}",
                event.timestamp, event.message.content
            ));
        }
    }
}