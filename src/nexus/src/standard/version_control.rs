use uuid::Uuid;
use crate::standard::transaction::Transaction;

/// Enumerates the kinds of results possible from a deep comparison of two `Transaction`s.
#[derive(Debug, Clone, Copy, PartialEq)]
enum DeepComparisonResult {
    /// The `Transaction`s are identical.
    Match,
    /// The `Transaction`s are different
    Unique,
    /// The `Transaction`s are differing duplicates.
    Duplicate,
}



/// Enumerates the possible results from comparing the version of the server to the history of the client in a sync task.
/// The returned result relates to the client's version. For instance, a returned 'Behind' status indicates that the
/// version on the client is behind that of the server.
pub enum VersionResult {
    /// The vesion on the server and client are the same.
    Matching,
    /// The version on the client is ahead of the server's version.
    Ahead,
    /// The version on the client is behind the server's version.
    Behind,
}
impl VersionResult {
    /// Compares the last version on the server to the version history on the client and returns the result.
    #[must_use]
    pub fn compare(server_version: Uuid, client_version_history: Vec<Uuid>) -> VersionResult {
        // The client has no history
        if client_version_history.is_empty() { return VersionResult::Behind }

        // The client and server have matching versions
        let last_client_version = *client_version_history.last().expect("This is guaranteed not empty. - HistoryComparisionResult::compare()");
        if server_version == last_client_version { return VersionResult::Matching }

        // The client is ahead of the server
        if client_version_history.contains(&server_version) { return VersionResult::Ahead }

        // The client is behind the server
        VersionResult::Behind
    }
}



/// The new merged `ledger` that both the server and client will use after a merge conflict.
pub struct MergeResolution {
    new_version: Uuid,
    new_ledger: Vec<Transaction>,
}



/// Tracks `Transaction` `id`s in a way that can also differentiate between multiple `Transaction`s that have the same `id`.
struct QuantumId {
    /// The `id` of the duplicated `Transaction`.
    id: Uuid,
    /// Which form of the duplicated `Transaction` is desired.
    pointer: Option<usize>,
}



/// Holds multiple `Transaction`s with the same `id`.
struct QuantumTransaction {
    transactions: Vec<Transaction>,
}
impl QuantumTransaction {
    /// Creates a new `QuantumTransaction`.
    #[must_use]
    fn new(duplicates: Vec<Transaction>) -> QuantumTransaction { QuantumTransaction { transactions: duplicates } }

    /// Checks if a given `Transaction` is also a duplicate.
    #[must_use]
    fn is_differing_duplicate(&self, new_transaction: &Transaction) -> bool {
        if self.transactions.is_empty() { return false }
        MergeResolutionBuilder::deep_compare(&self.transactions[0], new_transaction) == DeepComparisonResult::Duplicate
    }

    /// Adds a given `Transaction` to the internal list of duplicates.
    fn add(&mut self, new_transaction: Transaction) { self.transactions.push(new_transaction); }
}



/// A helper object used to build a `MergeResolution`.
pub struct MergeResolutionBuilder {
    /// The list of `Transaction`s that match between the client and server.
    matching_transactions: Vec<Transaction>,
    /// The list of `Transaction`s that exist in the client or the server, but not both.
    unique_transactions: Vec<Transaction>,
    /// The list of `Transaction`s that exist in both the client and the server, but don't match.
    duplicate_transactions: Vec<QuantumTransaction>,
    /// The list of `Transaction`s that will not be kept.
    black_list: Vec<QuantumId>,
}
impl MergeResolutionBuilder {
    /// Assembles the MergeResolutionBuilder from the client and server `ledger`s.
    #[must_use]
    pub fn assemble(client_ledger: Vec<Transaction>, server_ledger: Vec<Transaction>) {
        let mut matches: Vec<Transaction> = Vec::new();
        let mut uniques: Vec<Transaction> = Vec::new();
        let mut duplicated: Vec<Transaction> = Vec::new();

        for client_transaction in client_ledger {
            let mut is_unique = true;
            for i in 0..server_ledger.len() {
                todo!()
            }
        }
    }

    /// Compares `Transaction`s to determine if they are the same.
    #[must_use]
    pub fn deep_compare(a: &Transaction, b: &Transaction) -> DeepComparisonResult {
        // comparison values
        let id_match = a.get_id() == b.get_id();
        let value_match = a.value == b.value;
        let date_match = a.date == b.date;
        let description_match = a.description == b.description;
        let tags_match = a.tags == b.tags;

        // different transactions
        if !id_match { return DeepComparisonResult::Unique }

        // matching transactions
        if id_match && value_match && date_match && description_match && tags_match { return DeepComparisonResult::Match }

        // duplicate transactions
        DeepComparisonResult::Duplicate
    }
}