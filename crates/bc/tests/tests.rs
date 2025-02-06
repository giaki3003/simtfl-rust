#[cfg(test)]
mod tests {
    use bc::context::BCContext;
    use bc::transaction::{BCTransaction, TXO};

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
}
