use crate::container::app::*;
use crate::ui::components::DatePickerModes;
use crate::vault::transaction::*;

/// Various signals that allow the application to communicate with the ui.
#[derive(Debug, Clone)]
pub enum Signal {
    // general signals
    /// Tells the application that an action is not allowed.
    /// Data passed: error message
    InvalidAction(String),

    /// Tells the application to return to a previous page.
    /// Data passed: new page
    Cancel(Pages),

    
    
    // transactions page signals
    /// Tells the application to open a new transaction page.
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the application to open an existing transaction page.
    /// Data passed: transaction id
    StartEditingTransaction(Id),

    
    
    // adding transaction page signals
    /// Tells the application to add a new transaction.
    /// Data passed: value, date, description, tags
    AddTransaction(Value, Date, String, Vec<Tag>),

    /// Updates the value state for transaction addition.
    /// Data passed: new value string
    UpdateNewValueString(String),

    /// Updates the currency state for transaction addition.
    /// Data passed: new currency string
    UpdateNewCurrencyString(String),

    /// Updates the date picker mode in transaction addition.
    /// Data passed: new date picker mode
    UpdateNewDatePickerMode(DatePickerModes),

    /// Goes to the previous year state for the date picker in transaction addition.
    /// Data passed: nothing
    GoToPreviousNewDatePickerSelectedYear,

    /// Goes to the next year state for the date picker in transaction addition.
    /// Data passed: nothing
    GoToNextNewDatePickerSelectedYear,

    /// Updates the month state for the date picker in transaction addition.
    /// Data passed: new month
    UpdateNewDatePickerSelectedMonth(Months),

    /// Updates the date state for transaction addition.
    /// Data passed: new date
    UpdateNewDate(Date),

    /// Updates the description state for transaction adding.
    /// Data passed: new description string
    UpdateNewDescriptionString(String),

    /// Updates the tags state for transaction adding.
    /// Data passed: new tags
    UpdateNewTags(Vec<Tag>),

    

    // editing transaction page signals
    /// Tells the application to edit an existing transaction.
    /// Data passed: new value, new date, new description, new tags
    EditTransaction(Value, Date, String, Vec<Tag>),
    
    /// Tells the application to open the transaction removal page.
    /// Data passed: transaction id
    StartRemovingTransaction(Id),
    
    /// Updates the value state for transaction editing.
    /// Data passed: new value string
    UpdateEditValueString(String),

    /// Updates the currency state for transaction editing.
    /// Data passed: new currency string
    UpdateEditCurrencyString(String),

    /// Updates the date picker mode in transaction editing.
    /// Data passed: new date picker mode
    UpdateEditDatePickerMode(DatePickerModes),

    /// Goes to the previous year state for the date picker in transaction editing.
    /// Data passed: nothing
    GoToPreviousEditDatePickerSelectedYear,

    /// Goes to the next year state for the date picker in transaction editing.
    /// Data passed: nothing
    GoToNextEditDatePickerSelectedYear,

    /// Updates the month state for the date picker in transaction editing.
    /// Data passed: new month
    UpdateEditDatePickerSelectedMonth(Months),

    /// Updates the date state for transaction editing.
    /// Data passed: new date
    UpdateEditDate(Date),

    /// Updates the description state for transaction editing.
    /// Data passed: new description string
    UpdateEditDescriptionString(String),
    
    /// Updates the tags state for transaction editing.
    /// Data passed: new tags
    UpdateEditTags(Vec<Tag>),
}