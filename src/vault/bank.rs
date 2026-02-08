use crate::vault::filter::Filter;
use crate::vault::transaction::*;

/// Holds a list of all the transactions.
pub struct Bank<'bank> {
    /// The central list of all transactions.
    ledger: Vec<Transaction>,
    /// The central id tracker for new transactions.
    id_tracker: usize,
    primary_filter: Filter<'bank>,
    deep_dive_1_filter: Filter<'bank>,
    deep_dive_2_filter: Filter<'bank>,
}
impl<'bank> Bank<'bank> {
    // initializing
    /// Creates a new bank object.
    pub fn new() -> Bank<'bank> {
        Bank { ledger: Vec::new(), id_tracker: 0, primary_filter: Filter::new(), deep_dive_1_filter: Filter::new(), deep_dive_2_filter: Filter::new() }
    }
    /// Initializes the bank.
    pub fn init(&'bank mut self) {
        self.init_filter_sources();
    }
    /// Sets the source collection for each filter.
    fn init_filter_sources(&'bank mut self) {
        self.primary_filter.set_source(&self.ledger);
        self.deep_dive_1_filter.set_source(&self.ledger);
        self.deep_dive_2_filter.set_source(&self.ledger);
    }



    // management
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
    
    
    
    // data retrieval and parsing
    /// Returns a mutable reference to the ledger.
    pub fn ledger(&mut self) -> &Vec<Transaction> {
        &mut self.ledger
    }
    /// Returns a mutable reference to a transaction.
    pub fn get(&mut self, id: usize) -> &mut Transaction {
        for transaction in &mut self.ledger {
            if transaction.get_id() == id { return transaction }
        }
        panic!("Transaction not found!")
    }
}