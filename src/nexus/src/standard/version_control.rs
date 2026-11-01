use uuid::Uuid;
use crate::standard::transaction::Transaction;

/// Enumerates the kinds of results possible from a comparison of two `Transaction`s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionComparisonResult {
    /// The `Transaction`s are identical.
    Match,
    /// The `Transaction`s are different
    Unique,
    /// The `Transaction`s are differing duplicates.
    Duplicate,
}
impl TransactionComparisonResult {
    /// Compares `Transaction`s to determine if they are the same.
    #[must_use]
    fn compare(a: &Transaction, b: &Transaction) -> TransactionComparisonResult {
        // comparison values
        let id_match = a.get_id() == b.get_id();
        let value_match = a.value == b.value;
        let date_match = a.date == b.date;
        let description_match = a.description == b.description;
        let tags_match = a.tags == b.tags;
    
        // different transactions
        if !id_match { return TransactionComparisonResult::Unique }
    
        // matching transactions
        if id_match && value_match && date_match && description_match && tags_match { return TransactionComparisonResult::Match }
    
        // duplicate transactions
        TransactionComparisonResult::Duplicate
    }
}



/// Enumerates the possible results from comparing the version of the server to the history of the client in a sync task.
/// The returned result relates to the client's version. For instance, a returned 'Behind' status indicates that the
/// version on the client is behind that of the server.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VersionComparisonResult {
    /// The vesion on the server and client are the same.
    Matching,
    /// The version on the client is ahead of the server's version.
    Ahead,
    /// The version on the client is behind the server's version.
    Behind,
}
impl VersionComparisonResult {
    /// Compares the last version on the server to the version history on the client and returns the result.
    #[must_use]
    pub fn compare(server_version: Uuid, client_version_history: Vec<Uuid>) -> VersionComparisonResult {
        // The client has no history
        if client_version_history.is_empty() { return VersionComparisonResult::Behind }

        // The client and server have matching versions
        let last_client_version = *client_version_history.last().expect("This is guaranteed not empty. - HistoryComparisionResult::compare()");
        if server_version == last_client_version { return VersionComparisonResult::Matching }

        // The client is ahead of the server
        if client_version_history.contains(&server_version) { return VersionComparisonResult::Ahead }

        // The client is behind the server
        VersionComparisonResult::Behind
    }
}



/// The new merged `ledger` that both the server and client will use after a merge conflict.
pub struct MergeResolution {
    pub new_version: Uuid,
    pub new_ledger: Vec<Transaction>,
}



/// Holds `Transaction`s in a form that can be used to reconcile conflicting `SaveData` versions.
/// It can hold either a single `Transaction`, or two in case there are duplicates.
/// Each `QuantumTransaction` is also assigned with its corresponding result from a `Transaction` comparison.
pub struct QuantumTransaction {
    /// The single unique or two duplicate `Transaction`s.
    transactions: Vec<Transaction>,
    /// The result from its corresponding `Transaction` comparison.
    form: TransactionComparisonResult,
    /// Determines which `Transaction` is desired if the given `QuantumTransaction` is a duplicate.
    /// This is always `0` if it represents a unique `Transaction`.
    selection: usize,
}
impl QuantumTransaction {
    // initializing
    /// Creates a new `QuantumTransaction`.
    #[must_use]
    fn new(transactions: Vec<Transaction>, form: TransactionComparisonResult) -> QuantumTransaction {
        if transactions.is_empty() { panic!("Each QuantumTransaction must have at least one Transaction! - QuantumTransaction::new()") }
        QuantumTransaction { transactions, form, selection: 0 }
    }


    
    // getters
    /// Gets the id for the `Transaction`/`Transaction`s being held in superposition.
    #[must_use]
    fn id(&self) -> Uuid { self.transactions[0].get_id() }

    /// Gets the `Transaction`/`Transaction`s being held in superposition.
    #[must_use]
    pub fn list(&self) -> &[Transaction] { &self.transactions }

    /// Gets the `form`.
    #[must_use]
    pub fn form(&self) -> TransactionComparisonResult { self.form }

    /// Gets the `selection`.
    #[must_use]
    pub fn selection(&self) -> usize { self.selection }

    /// Gets the selected `Transaction`.
    /// This is only meant to be used when the `MergeResolution` is being built.
    #[must_use]
    fn get_selected(self) -> Transaction { self.transactions[self.selection].clone() }



    // management
    /// Selects a new `Transaction` varient.
    fn select(&mut self, new_selection: usize) {
        if new_selection >= self.transactions.len() { return }
        self.selection = new_selection;
    }
}



/// A helper object used to build a `MergeResolution`.
pub struct MergeResolutionBuilder {
    /// The list of `QuantumTransaction`s that will be interacted with.
    quantum_transactions: Vec<QuantumTransaction>,
}
impl MergeResolutionBuilder {
    /// Assembles the MergeResolutionBuilder from the client and server `ledger`s.
    #[must_use]
    pub fn new(client_ledger: Vec<Transaction>, mut server_ledger: Vec<Transaction>) -> MergeResolutionBuilder {
        // the intermediate building list
        let mut quantum_transactions: Vec<QuantumTransaction> = Vec::new();
        
        // Every transaction in each ledger is guaranteed to have a unique id.
        // This means that if a match or a duplicate is found between the two ledgers,
        // no further comparison is needed since each list can only have that given id
        // one time.

        // checking each transaction in the client ledger
        for client_transaction in client_ledger {
            // each transaction in the client ledger is compared against each transaction
            // in the server ledger until either a match or duplicate is found
            for i in 0..server_ledger.len() {
                match TransactionComparisonResult::compare(&client_transaction, &server_ledger[i]) {
                    TransactionComparisonResult::Match => {
                        let qt = QuantumTransaction::new(vec![client_transaction.clone()], TransactionComparisonResult::Match);
                        quantum_transactions.push(qt);
                        server_ledger.remove(i);
                        break;
                    },
                    TransactionComparisonResult::Duplicate => {
                        let qt = QuantumTransaction::new(vec![client_transaction.clone(), server_ledger.remove(i)], TransactionComparisonResult::Duplicate);
                        quantum_transactions.push(qt);
                        break;
                    },
                    _ => {},
                };
            };
            // if no matches or duplicates were found, the transaction is guaranteed to be unique
            let qt = QuantumTransaction::new(vec![client_transaction.clone()], TransactionComparisonResult::Unique);
            quantum_transactions.push(qt);

            // The use of clone() throughout this section is safe because the client ledger is only passed
            // through one time.
        }

        // after all client transactions have been checked, any remaining server transactions are
        // also guaranteed to be unique
        let qts: Vec<QuantumTransaction> = server_ledger
            .into_iter()
            .map(|t| QuantumTransaction::new(vec![t], TransactionComparisonResult::Unique))
            .collect();
        quantum_transactions.extend(qts);

        // returning the finsihed builder object
        MergeResolutionBuilder { quantum_transactions }
    }

    /// Gets the list of `QuantumTransaction`s.
    #[must_use]
    pub fn list(&self) -> &[QuantumTransaction] { &self.quantum_transactions }

    /// Selects a new `Transaction` varient in the desired `QuantumTransaction`.
    pub fn select(&mut self, transaction_id: Uuid, new_selection: usize) {
        for qt in &mut self.quantum_transactions {
            if qt.id() == transaction_id {
                qt.select(new_selection);
                return
            }
        }
    }
    
    /// Converts the `MergeResolutionBuilder` into a finalized `MergeResolution`.
    #[must_use]
    pub fn build(self) -> MergeResolution {
        let transactions: Vec<Transaction> = self.quantum_transactions
            .into_iter()
            .map(|qt| qt.get_selected())
            .collect();
        
        MergeResolution { new_version: Uuid::new_v4(), new_ledger: transactions }
    }
}