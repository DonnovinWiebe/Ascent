use crate::vault::transaction::*;

/// Holds a list of all the transactions.
pub struct Bank {
    /// The central list of all transactions.
    ledger: Vec<Transaction>,
    /// The central id tracker for new transactions.
    id_tracker: usize,
}
impl Bank {
    /// Creates a new bank object.
    pub fn new() -> Bank {
        Bank { ledger: Vec::new(), id_tracker: 0 }
    }

    
    
    // manages the ledger
    /// Returns a mutable reference to the ledger.
    pub fn ledger(&mut self) -> &Vec<Transaction> {
        &mut self.ledger
    }
    
    /// Returns a mutable reference to a transaction.
    pub fn get_transaction(&mut self, id: usize) -> &mut Transaction {
        for transaction in &mut self.ledger {
            if transaction.get_id() == id { return transaction }
        }
        panic!("Transaction not found!")
    }
    
    /// Sorts a ledger by date.
    pub fn sorted_ledger(ledger: Vec<Transaction>) -> Vec<Transaction> {
        let mut ledger = ledger;
        ledger.sort_by(|a, b| a.date.as_value().cmp(&b.date.as_value()));
        ledger
    }

    /// Sorts the ledger by date.
    pub fn sort_ledger(&mut self) {
        // I could duplicate sorted_ledger() here, but this is faster
        self.ledger.sort_by(|a, b| a.date.as_value().cmp(&b.date.as_value()));
    }
    
    /// Adds a new transaction to the ledger.
    pub fn add_transaction(&mut self, value: Value, date: Date, description: Tag, tags: Vec<Tag>) {
        self.ledger.push(Transaction::new(self.id_tracker, value, date, description, tags));
        self.id_tracker += 1;
        self.sort_ledger();
    }
    
    /// Removes a transaction from the ledger.
    pub fn remove_transaction(&mut self, id: usize) {
        for (index, transaction) in self.ledger.iter().enumerate() {
            if transaction.get_id() == id { self.ledger.remove(index); return }
        }
        panic!("Transaction not found!")
    }
}