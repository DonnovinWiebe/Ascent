use crate::container::app::*;
use crate::vault::transaction::*;

/// Various signals that allow the application to communicate with the ui.
#[derive(Debug, Clone)]
pub enum Signal {
    /// Tells the application that an action is not allowed.
    /// Data passed: error message
    InvalidAction(String),

    /// Tells the application to open a new transaction page.
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the application to open an existing transaction page.
    /// Data passed: transaction id
    StartEditingTransaction(Id),

    /// Tells the application to open the transaction removal page.
    /// Data passed: transaction id
    StartRemovingTransaction(Id),

    /// Tells the application to return to a previous page.
    /// Data passed: new page
    Cancel(Pages),

    /// Tells the application to add a new transaction.
    /// Data passed: value, date, description, tags
    AddTransaction(Value, Date, Tag, Vec<Tag>),

    /// Tells the application to edit an existing transaction.
    /// Data passed: new value, new date, new description, new tags
    EditTransaction(Value, Date, Tag, Vec<Tag>),
}