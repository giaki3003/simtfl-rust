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

    #[test]
    fn test_streamlet() {
        let _genesis = StreamletGenesis::new(3);
        let proposal1 = StreamletProposal::new(
            Box::new(PermissionedBFTEnum::Base(PermissionedBFTBase {
                n: 3,
                t: 2,
                parent: None,
            })),
            1,
        );

        let block1 = StreamletBlock {
            proposal: Box::new(proposal1),
            parent: None,
        };

        // Verify that the last_final method returns the genesis block
        assert_eq!(
            block1.last_final(),
            PermissionedBFTEnum::Base(PermissionedBFTBase {
                n: 3,
                t: 2,
                parent: None,
            })
        );
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
        // Create a new event queue
        let mut event_queue = EventQueue::new();

        // Create events with different timestamps
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

        // Schedule events in the queue
        event_queue.schedule(event1);
        event_queue.schedule(event2);
        event_queue.schedule(event3);

        // Process events in order of their timestamps
        let mut processed_events = Vec::new();
        while let Some(event) = event_queue.process_next_event() {
            processed_events.push(event);
        }

        // Verify the order of processed events
        assert_eq!(processed_events.len(), 3);
        assert_eq!(processed_events[0].timestamp, 5);
        assert_eq!(processed_events[1].timestamp, 7);
        assert_eq!(processed_events[2].timestamp, 10);

        println!("Processed events:");
        for event in processed_events {
            println!(
                "Timestamp: {}, Content: {}",
                event.timestamp, event.message.content
            );
        }
    }
}
