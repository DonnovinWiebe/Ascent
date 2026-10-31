use uuid::Uuid;


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