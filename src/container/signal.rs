use crate::container::app::*;
use crate::vault::transaction::*;

/// Various signals that allow the application to communicate with the ui.
pub enum Signal {
    /// Tells the ui to open a new transaction page.
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the ui to open an existing transaction page.
    /// Data passed: transaction id
    StartEditingTransaction(usize),

    /// Tells the ui to open the transaction removal page.
    /// Data passed: transaction id
    StartRemovingTransaction(usize),

    /// Tells the ui to return to a previous page.
    /// Data passed: new page
    Cancel(Pages),

    /// Tells the application to add a new transaction.
    /// Data passed: value, date, description, tags
    AddTransaction(Value, Date, Tag, Vec<Tag>),

    /// Tells the application to edit an existing transaction.
    /// Data passed: new value, new date, new description, new tags
    EditTransaction(Value, Date, Tag, Vec<Tag>),
}