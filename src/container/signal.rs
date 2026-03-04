use iced::widget::text_editor::Action;
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

    /// Tells the application to return to the home page.
    /// Data passed: nothing
    GoHome,

    /// Tells the application to cycle to the next theme.
    /// Data passed: nothing
    CycleTheme,

    
    
    // transactions page signals
    /// Tells the application to open a new transaction page.
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the application to open an existing transaction page.
    /// Data passed: transaction id
    StartEditingTransaction(Id),

    
    
    // adding transaction page signals
    /// Tells the application to add a new transaction.
    /// Data passed: value string, currency string, date, description, tags
    AddTransaction(String, String, Date, String, Vec<Tag>),

    /// Updates the value state for transaction addition.
    /// Data passed: new value string
    UpdateNewTransactionValueString(String),

    /// Updates the currency state for transaction addition.
    /// Data passed: new currency string
    UpdateNewTransactionCurrencyString(String),

    /// Updates the date picker mode in transaction addition.
    /// Data passed: new date picker mode
    UpdateNewTransactionDatePickerMode(DatePickerModes),

    /// Goes to the next year state for the date picker in transaction addition.
    /// Data passed: nothing
    AdvanceNewTransactionCurrentYear,

    /// Goes to the previous year state for the date picker in transaction addition.
    /// Data passed: nothing
    RecedeNewTransactionCurrentYear,

    /// Updates the month state for the date picker in transaction addition.
    /// Data passed: new month
    UpdateNewTransactionCurrentMonth(Months),

    /// Updates the date state for transaction addition.
    /// Data passed: new date
    UpdateNewTransactionSelectedDate(Date),

    /// Updates the description state for transaction adding.
    /// Data passed: editor action
    UpdateNewTransactionDescriptionContent(Action),

    /// Updates the tags state for transaction adding.
    /// Data passed: new tags
    UpdateNewTransactionTags(Vec<Tag>),

    

    // editing transaction page signals
    /// Tells the application to edit an existing transaction.
    /// Data passed: value string, currency string, date, description, tags
    EditTransaction(String, String, Date, String, Vec<Tag>),
    
    /// Tells the application to open the transaction removal page.
    /// Data passed: transaction id
    StartRemovingTransaction(Id),
    
    /// Updates the value state for transaction editing.
    /// Data passed: new value string
    UpdateEditTransactionValueString(String),

    /// Updates the currency state for transaction editing.
    /// Data passed: new currency string
    UpdateEditTransactionCurrencyString(String),

    /// Updates the date picker mode in transaction editing.
    /// Data passed: new date picker mode
    UpdateEditTransactionDatePickerMode(DatePickerModes),

    /// Goes to the previous year state for the date picker in transaction editing.
    /// Data passed: nothing
    RecedeEditTransactionCurrentYear,

    /// Goes to the next year state for the date picker in transaction editing.
    /// Data passed: nothing
    AdvanceEditTransactionCurrentYear,

    /// Updates the month state for the date picker in transaction editing.
    /// Data passed: new month
    UpdateEditTransactionCurrentMonth(Months),

    /// Updates the date state for transaction editing.
    /// Data passed: new date
    UpdateEditTransactionSelectedDate(Date),

    /// Updates the description state for transaction editing.
    /// Data passed: editor action
    UpdateEditTransactionDescriptionContent(Action),
    
    /// Updates the tags state for transaction editing.
    /// Data passed: new tags
    UpdateEditTags(Vec<Tag>),
}