#[cfg(test)]
mod tests {
    use bc::transaction::Note;
    use bc::block::BlockHash;
    use bc::block::BCBlock;
    use bc::context::BCContext;
    use bc::transaction::{BCTransaction, TXO};

    // Helper to create a dummy BCTransaction for TXO purposes.
    fn dummy_bc_transaction() -> BCTransaction {
        BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: 0,
            anchor: None,
            issuance: 0,
        }
    }

    // Helper to create a dummy TXO with the given value.
    fn dummy_txo(value: i32) -> TXO {
        TXO {
            tx: dummy_bc_transaction(),
            index: 0,
            value,
        }
    }

    #[test]
    fn test_context_initialization() {
        let ctx = BCContext::new();
        assert!(ctx.transactions.is_empty());
        assert!(ctx.utxo_set.is_empty());
        assert!(ctx.notes.is_empty());
        assert_eq!(ctx.total_issuance, 0);
    }

    #[test]
    fn test_add_valid_transaction() {
        let mut ctx = BCContext::new();
        let tx = BCTransaction {
            transparent_inputs: Vec::new(),
            transparent_outputs: Vec::new(),
            shielded_inputs: Vec::new(),
            shielded_outputs: Vec::new(),
            fee: 0,
            anchor: None,
            issuance: 0,
        };
        assert!(ctx.add_transaction(tx));
        assert_eq!(ctx.transactions.len(), 1);
    }

    #[test]
    fn test_add_invalid_transaction() {
        let mut ctx = BCContext::new();
        
        // Create a dummy BCTransaction for TXO
        let dummy_tx = BCTransaction {
            transparent_inputs: Vec::new(),
            transparent_outputs: Vec::new(),
            shielded_inputs: Vec::new(),
            shielded_outputs: Vec::new(),
            fee: 0,
            anchor: None,
            issuance: 0,
        };

        // Create a dummy TXO
        let dummy_txo = TXO {
            tx: dummy_tx,
            index: 0,
            value: 100,
        };

        // Create a transaction with the dummy TXO to ensure it's not a coinbase transaction
        let tx = BCTransaction {
            transparent_inputs: vec![dummy_txo],
            transparent_outputs: Vec::new(),
            shielded_inputs: Vec::new(),
            shielded_outputs: Vec::new(),
            fee: -1,
            anchor: None,
            issuance: 0,
        };
        
        println!("Attempting to add invalid transaction with fee: {}", tx.fee);
        let result = ctx.add_transaction(tx);
        println!("Result of adding invalid transaction: {}", result);
        
        assert!(!result);
        assert!(ctx.transactions.is_empty());
    }
    #[test]
    fn test_basic() {
        // Step 1: Create a BCContext.
        let mut ctx = BCContext::new();

        // Step 2: Create a coinbase transaction.
        // Python: coinbase_tx0 = BCTransaction([], [10], [], [], 0, issuance=10)
        let coinbase_tx0 = BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![dummy_txo(10)],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: 0,
            anchor: None,
            issuance: 10,
        };

        // Add coinbase_tx0 to the context.
        assert!(ctx.add_transaction(coinbase_tx0.clone()));
        // After adding coinbase_tx0, total issuance should be 10.
        assert_eq!(ctx.total_issuance, 10);

        // Step 3: Create the genesis block.
        // Python: genesis = BCBlock(None, 1, [coinbase_tx0])
        let genesis = BCBlock {
            parent: None,
            score: 1,
            transactions: vec![coinbase_tx0.clone()],
            hash: BlockHash::new(),
        };

        // Verify the genesis block's score and the context.
        assert_eq!(genesis.score, 1);
        assert_eq!(ctx.total_issuance, 10);

        // Step 4: Create coinbase_tx1 and spend_tx.
        // coinbase_tx1 = BCTransaction([], [6], [], [], -1, issuance=5)
        let coinbase_tx1 = BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![dummy_txo(6)],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: -1,
            anchor: None,
            issuance: 5,
        };

        // For spend_tx, we simulate consuming coinbase_tx0.transparent_output(0)
        let spend_input = dummy_txo(10); // In a real implementation, you might retrieve the actual output.
        // spend_tx = BCTransaction([coinbase_tx0.transparent_output(0)], [9], [], [], 1)
        let spend_tx = BCTransaction {
            transparent_inputs: vec![spend_input],
            transparent_outputs: vec![dummy_txo(9)],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: 1,
            anchor: None,
            issuance: 0,
        };

        // Add coinbase_tx1 and spend_tx.
        assert!(ctx.add_transaction(coinbase_tx1.clone()));
        assert!(ctx.add_transaction(spend_tx.clone()));

        // Create block1: parent = genesis, score = 2, transactions = [coinbase_tx1, spend_tx]
        let block1 = BCBlock {
            parent: Some(genesis.hash),
            score: 2,
            transactions: vec![coinbase_tx1.clone(), spend_tx.clone()],
            hash: BlockHash::new(),
        };

        // After block1, total issuance should be 10 + 5 = 15.
        assert_eq!(block1.score, 2);
        assert_eq!(ctx.total_issuance, 15);

        // Step 5: Create coinbase_tx2 and shielding_tx.
        // coinbase_tx2 = BCTransaction([], [6], [], [], -1, issuance=5)
        let coinbase_tx2 = BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![dummy_txo(6)],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: -1,
            anchor: None,
            issuance: 5,
        };

        // For shielding_tx, we need coinbase_tx1.transparent_output(0) and spend_tx.transparent_output(0).
        let coinbase_tx1_output = dummy_txo(6);
        let spend_tx_output = dummy_txo(9);
        // shielding_tx = BCTransaction([coinbase_tx1.transparent_output(0), spend_tx.transparent_output(0)], [], [], [8, 6], 1)
        let shielding_tx = BCTransaction {
            transparent_inputs: vec![coinbase_tx1_output, spend_tx_output],
            transparent_outputs: vec![],
            shielded_inputs: vec![],
            shielded_outputs: vec![Note { value: 8 }, Note { value: 6 }],
            fee: 1,
            anchor: None,
            issuance: 0,
        };

        assert!(ctx.add_transaction(coinbase_tx2.clone()));
        assert!(ctx.add_transaction(shielding_tx.clone()));

        // Create block2: parent = block1, score = 4, transactions = [coinbase_tx2, shielding_tx]
        let block2 = BCBlock {
            parent: Some(block1.hash),
            score: 4,
            transactions: vec![coinbase_tx2.clone(), shielding_tx.clone()],
            hash: BlockHash::new(),
        };

        // Simulate anchoring by copying the context.
        let block2_anchor = ctx.clone();
        assert_eq!(block2.score, 4);
        // Total issuance becomes 15 + 5 = 20.
        assert_eq!(ctx.total_issuance, 20);

        // Step 6: Create coinbase_tx3, shielded_tx, and deshielding_tx.
        // coinbase_tx3 = BCTransaction([], [7], [], [], -2, issuance=5)
        let coinbase_tx3 = BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![dummy_txo(7)],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: -2,
            anchor: None,
            issuance: 5,
        };

        // For shielded_tx, we simulate shielding_tx.shielded_output(0) as the first element.
        let shielded_tx = BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![],
            shielded_inputs: vec![],
            shielded_outputs: vec![Note { value: 7 }],
            fee: 1,
            anchor: Some(block2_anchor.clone()),
            issuance: 0,
        };

        // For deshielding_tx, simulate shielding_tx.shielded_output(1) as the second element.
        let deshielding_tx = BCTransaction {
            transparent_inputs: vec![],
            transparent_outputs: vec![dummy_txo(5)],
            shielded_inputs: vec![],
            shielded_outputs: vec![],
            fee: 1,
            anchor: Some(block2_anchor.clone()),
            issuance: 0,
        };

        assert!(ctx.add_transaction(coinbase_tx3.clone()));
        assert!(ctx.add_transaction(shielded_tx.clone()));
        assert!(ctx.add_transaction(deshielding_tx.clone()));

        // Create block3: parent = block2, score = 7, transactions = [coinbase_tx3, shielded_tx, deshielding_tx]
        let block3 = BCBlock {
            parent: Some(block2.hash),
            score: 7,
            transactions: vec![
                coinbase_tx3.clone(),
                shielded_tx.clone(),
                deshielding_tx.clone(),
            ],
            hash: BlockHash::new(),
        };

        // Total issuance becomes 20 + 5 = 25.
        assert_eq!(block3.score, 7);
        assert_eq!(ctx.total_issuance, 25);
    }
}
