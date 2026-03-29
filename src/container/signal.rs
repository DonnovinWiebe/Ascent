use iced::widget::text_editor::Action;
use crate::vault::parse::RingParse;
use crate::{container::app::*, vault::bank::Filters};
use crate::ui::components::DatePickerModes;
use crate::ui::material::MaterialColors;
use crate::vault::result_stack::ResultStack;
use crate::vault::transaction::*;
use iced::{Point, Size};

/// Various signals that allow the application to communicate with the ui.
#[derive(Debug, Clone)]
pub enum Signal {
    // general signals
    /// Tells the application that an action is not allowed.
    /// Data passed: error message
    InvalidAction(String),
    
    /// Tells the application to dismiss errors.
    /// Data passed: nothing
    DismissErrors,

    /// Tells the application to return to the home page.
    /// Data passed: nothing
    GoHome,

    /// Tells the application to cycle to the next theme.
    /// Data passed: nothing
    CycleTheme,
    
    
    
    // filtering
    /// Tells the application to set the filter year.
    /// Data passed: year, filter
    SetFilterYear(u32, Filters),
    
    /// Tells the application to clear the filter year.
    /// Data passed: filter
    ClearFilterYear(Filters),
    
    /// Tells the application to set the filter month.
    /// Data passed: month, filter
    SetFilterMonth(Months, Filters),
    
    /// Tells the application to clear the filter month.
    /// Data passed: filter
    ClearFilterMonth(Filters),
    
    /// Tells the application to add a tag to the given filter.
    /// Data passed: tag, filter
    AddFilterTag(Tag, Filters),

    /// Tells the application to remove a tag from the given filter.
    /// Data passed: tag, filter
    RemoveFilterTag(Tag, Filters),
    
    /// Tells the application to clear all tags from the given filter.
    /// Data passed: filter
    ClearFilterTags(Filters),
    
    /// Tells the application to update the current search term string for the primary filter.
    /// Data passed: new search term
    UpdatePrimaryFilterCurrentSearchTermString(String),
    
    /// Tells the application to update the current search term string for the deep dive 1 filter.
    /// Data passed: new search term
    UpdateDeepDive1FilterCurrentSearchTermString(String),
    
    /// Tells the application to update the current search term string for the deep dive 2 filter.
    /// Data passed: new search term
    UpdateDeepDive2FilterCurrentSearchTermString(String),
    
    /// Tells the application to add a search term to the given filter.
    /// Data passed: filter
    AddFilterSearchTerm(Filters),
    
    /// Tells the application to remove a search term from the given filter.
    /// Data passed: search term, filter
    RemoveFilterSearchTerm(String, Filters),
    
    /// Tells the application to clear all search terms from the given filter.
    /// Data passed: filter
    ClearFilterSearchTerms(Filters),
    
    /// Tells the application to toggle the filter mode for the given filter.
    /// Data passed: filter
    ToggleFilterMode(Filters),
    
    /// Tells the application that the ring chart has started rendering.
    /// Data passed: nothing
    StartedRenderingRingCharts,
    
    /// Tells the application that the ring chart has finished rendering.
    /// Data passed: rendered ring parse (in a result to match App implementation), render results - one set for each chart
    FinishedRenderingRingCharts((ResultStack<RingParse>, ResultStack<()>), (ResultStack<RingParse>, ResultStack<()>)),

    
    
    // transactions page signals
    /// Tells the application to open a new transaction page.
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the application to open an existing transaction page.
    /// Data passed: transaction id
    StartEditingTransaction(Id),
    
    /// Tells the application that the mouse has moved in the earning ring chart.
    /// Data passed: new mouse position, layout size
    MouseMovedInEarningRingChart(Point, Size),
    
    /// Tells the application that the mouse has moved in the spending ring chart.
    /// Data passed: new mouse position, layout size
    MouseMovedInSpendingRingChart(Point, Size),
    
    /// Tells the application that the mouse has left the earning ring chart.
    /// Data passed: nothing
    MouseExitedEarningRingChart,
    
    /// Tells the application that the mouse has left the spending ring chart.
    /// Data passed: nothing
    MouseExitedSpendingRingChart,
    
    /// Tells the application to open the tag registry.
    /// Data passed: nothing
    OpenTagRegistry,
    

    
    
    // adding transaction page signals
    /// Tells the application to add a new transaction.
    /// Data passed: nothing (everything is set in the app state)
    AddTransaction,

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
    UpdateNewTransactionSelectedDate(ResultStack<Date>),

    /// Updates the description state for transaction adding.
    /// Data passed: editor action
    UpdateNewTransactionDescriptionContent(Action),

    /// Updates the current tag for transaction adding.
    /// Data passed: new tag string
    UpdateNewTransactionCurrentTagString(String),

    /// Adds a tag for transaction adding.
    /// Data passed: tag string to add
    AddNewTransactionTag(String),

    /// Removes a tag for transaction adding.
    /// Data passed: tag to remove
    RemoveNewTransactionTag(Tag),

    

    // editing transaction page signals
    /// Tells the application to edit an existing transaction.
    /// Data passed: nothing (everything is set in the app state)
    EditTransaction,

    /// Tells the application to prime the transaction being edited for deleting.
    /// Data passed: nothing
    PrimeRemoveTransaction,
    
    /// Tells the application to unprime the transaction being edited for deleting.
    /// Data passed: nothing
    UnprimeRemoveTransaction,
    
    /// Tells the application to remove the transaction being edited.
    /// Data passed: nothing
    RemoveTransaction,
    
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
    UpdateEditTransactionSelectedDate(ResultStack<Date>),

    /// Updates the description state for transaction editing.
    /// Data passed: editor action
    UpdateEditTransactionDescriptionContent(Action),

    /// Updates the current tag for transaction editing.
    /// Data passed: new tag string
    UpdateEditTransactionCurrentTagString(String),

    /// Adds a tag for transaction editing.
    /// Data passed: tag string to add
    AddEditTransactionTag(String),
    
    /// Removes a tag for transaction editing.
    /// Data passed: tag to remove
    RemoveEditTransactionTag(Tag),
    
    
    
    // tag registry page signals
    /// Expands a tag in the tag registry page.
    /// Data passed: tag
    ExpandTag(Tag),
    
    /// Collapses a tag in the tag registry page.
    /// Data passed: tag
    CollapseTag(Tag),
    
    /// Tells the application to set the color of a tag in the tag registry.
    /// Data passed: tag, color
    SetTagColor(Tag, MaterialColors),
}