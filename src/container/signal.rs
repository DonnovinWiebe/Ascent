use std::path::PathBuf;

use iced::widget::text_editor::Action;
use crate::vault::bank::TagRegistry;
use crate::vault::parse::RingParse;
use crate::container::app::Pages;
use crate::vault::bank::Filters;
use crate::ui::components::DatePickerModes;
use crate::ui::material::{AppThemes, MaterialColors};
use crate::vault::result_stack::ResultStack;
use crate::vault::transaction::{Date, Id, Months, Tag};
use iced::{Point, Size};

/// Various signals that allow the `App` to communicate with the ui.
#[derive(Debug, Clone)]
pub enum Signal {
    // keybinds
    /// Tells the `App` to focus the next widget.
    /// Data passed: nothing
    FocusNext,
    
    /// Tells the `App` to focus the previous widget.
    /// Data passed: nothing
    FocusPrevious,
    
    /// Tells the `App` to add a transaction as the context requires.
    /// Data passed: nothing
    AddTransactionKeybind,
    
    /// Tells the `App` to advance the year as the context requires.
    /// Data passed: nothing
    AdvanceYearKeybind,
    
    /// Tells the `App` to recede the year as the context requires.
    /// Data passed: nothing
    RecedeYearKeybind,
    
    /// Tells the `App` to advance the month as the context requires.
    /// Data passed: nothing
    AdvanceMonthKeybind,
    
    /// Tells the `App` to recede the month as the context requires.
    /// Data passed: nothing
    RecedeMonthKeybind,
    
    /// Tells the `App` to advance the day as the context requires.
    /// Data passed: nothing
    AdvanceDayKeybind,
    
    /// Tells the `App` to recede the day as the context requires.
    /// Data passed: nothing
    RecedeDayKeybind,
    
    
    
    // general signals
    /// Tells the `App` that the `TagRegistry` has finished updating.
    /// Data passed: updated `TagRegistry`
    FinishedUpdatingTagRegistry(TagRegistry),
    
    /// Tells the `App` that an action is not allowed.
    /// Data passed: error message
    InvalidAction(String),
    
    /// Tells the `App` to dismiss errors.
    /// Data passed: nothing
    DismissErrors,
    
    /// Tells the `App` to change the `Page`.
    /// Data passed: new `Page`
    ChangePageTo(Pages),

    /// Tells the `App` to return to the home `Page`.
    /// Data passed: nothing
    GoHome,
    
    
    
    // filtering
    /// Tells the `App` to set the filter year.
    /// Data passed: year, `Filter`
    SetFilterYear(u32, Filters),
    
    /// Tells the `App` to clear the filter year.
    /// Data passed: `Filter`
    ClearFilterYear(Filters),
    
    /// Tells the `App` to set the filter month.
    /// Data passed: `Month`, `Filter`
    SetFilterMonth(Months, Filters),
    
    /// Tells the `App` to clear the filter month.
    /// Data passed: `Filter`
    ClearFilterMonth(Filters),
    
    /// Tells the `App` to add a `Tag` to the given `Filter`.
    /// Data passed: `Tag`, `Filter`
    AddFilterTag(Tag, Filters),

    /// Tells the `App` to remove a `Tag` from the given `Filter`.
    /// Data passed: `Tag`, `Filter`
    RemoveFilterTag(Tag, Filters),
    
    /// Tells the `App` to clear all `Tag`s from the given `Filter`.
    /// Data passed: `Filter`
    ClearFilterTags(Filters),
    
    /// Tells the `App` to update the current search term string for the `primary_filter`.
    /// Data passed: new search term
    UpdatePrimaryFilterCurrentSearchTermString(String),
    
    /// Tells the `App` to update the current search term string for the `deep_dive_1_filter`.
    /// Data passed: new search term
    UpdateDeepDive1FilterCurrentSearchTermString(String),
    
    /// Tells the `App` to update the current search term string for the `deep_dive_2_filter`.
    /// Data passed: new search term
    UpdateDeepDive2FilterCurrentSearchTermString(String),
    
    /// Tells the `App` to add a search term to the given `Filter`.
    /// Data passed: `Filter`
    AddFilterSearchTerm(Filters),
    
    /// Tells the `App` to remove a search term from the given `Filter`.
    /// Data passed: search term, `Filter`
    RemoveFilterSearchTerm(String, Filters),
    
    /// Tells the `App` to clear all search terms from the given `Filter`.
    /// Data passed: `Filter`
    ClearFilterSearchTerms(Filters),
    
    /// Tells the `App` to toggle the `Filter` mode for the given `Filter`.
    /// Data passed: `Filter`
    ToggleFilterMode(Filters),
    
    /// Tells the `App` that the `RingChart` has started rendering.
    /// Data passed: nothing
    StartedRenderingRingCharts,
    
    /// Tells the `App` that the `RingChart` has finished rendering.
    /// Data passed: rendered `RingParse` (in a `Result` to match `App` implementation), render results - one set for each chart
    FinishedRenderingRingCharts(Box<(ResultStack<RingParse>, ResultStack<()>)>, Box<(ResultStack<RingParse>, ResultStack<()>)>),

    
    
    // transactions page signals
    /// Tells the `App` to start adding a new `Transaction`.
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the `App` to start editing a `Transaction`.
    /// Data passed: transaction id
    StartEditingTransaction(ResultStack<Id>),
    
    /// Tells the `App` that the mouse has moved in the earning `RingChart`.
    /// Data passed: new mouse position, layout size
    MouseMovedInEarningRingChart(Point, Size),
    
    /// Tells the `App` that the mouse has moved in the spending `RingChart`.
    /// Data passed: new mouse position, layout size
    MouseMovedInSpendingRingChart(Point, Size),
    
    /// Tells the `App` that the mouse has left the earning `RingChart`.
    /// Data passed: nothing
    MouseExitedEarningRingChart,
    
    /// Tells the `App` that the mouse has left the spending `RingChart`.
    /// Data passed: nothing
    MouseExitedSpendingRingChart,
    
    /// Tells the `App` to open the tag registry page.
    /// Data passed: nothing
    OpenTagRegistry,
    

    
    
    // adding transaction page signals
    /// Tells the `App` to add a new `Transaction`.
    /// Data passed: nothing
    AddTransaction,

    /// Updates the value state for `Transaction` addition.
    /// Data passed: new `Value` `String`
    UpdateNewTransactionValueString(String),

    /// Updates the currency state for `Transaction` addition.
    /// Data passed: new `Currency` `String`
    UpdateNewTransactionCurrencyString(String),

    /// Updates the date picker mode in `Transaction` addition.
    /// Data passed: new date picker mode
    UpdateNewTransactionDatePickerMode(DatePickerModes),

    /// Goes to the next year state for the date picker in `Transaction` addition.
    /// Data passed: nothing
    AdvanceNewTransactionCurrentYear,

    /// Goes to the previous year state for the date picker in `Transaction` addition.
    /// Data passed: nothing
    RecedeNewTransactionCurrentYear,

    /// Updates the `Month` state for the date picker in `Transaction` addition.
    /// Data passed: new `Month`
    UpdateNewTransactionCurrentMonth(Months),

    /// Updates the date state for `Transaction` addition.
    /// Data passed: new `Date`
    UpdateNewTransactionSelectedDate(ResultStack<Date>),

    /// Updates the description state for `Transaction` adding.
    /// Data passed: editor `Action`
    UpdateNewTransactionDescriptionContent(Action),

    /// Updates the current tag for `Transaction` adding.
    /// Data passed: new `Tag` `String`
    UpdateNewTransactionCurrentTagString(String),

    /// Adds a tag for `Transaction` adding.
    /// Data passed: `Tag` `String` to add
    AddNewTransactionTag(String),

    /// Removes a `Tag` for `Transaction` adding.
    /// Data passed: `Tag` to remove
    RemoveNewTransactionTag(Tag),

    

    // editing transaction page signals
    /// Tells the `App` to edit an existing `Transaction`.
    /// Data passed: nothing
    EditTransaction,

    /// Tells the `App` to prime the `Transaction` being edited for deleting.
    /// Data passed: nothing
    PrimeRemoveTransaction,
    
    /// Tells the `App` to unprime the `Transaction` being edited for deleting.
    /// Data passed: nothing
    UnprimeRemoveTransaction,
    
    /// Tells the `App` to remove the `Transaction` being edited.
    /// Data passed: nothing
    RemoveTransaction,
    
    /// Updates the value state for `Transaction` editing.
    /// Data passed: new `Value` `String`
    UpdateEditTransactionValueString(String),

    /// Updates the currency state for `Transaction` editing.
    /// Data passed: new `Currency` `String`
    UpdateEditTransactionCurrencyString(String),

    /// Updates the date picker mode in `Transaction` editing.
    /// Data passed: new date picker mode
    UpdateEditTransactionDatePickerMode(DatePickerModes),

    /// Goes to the previous year state for the date picker in `Transaction` editing.
    /// Data passed: nothing
    RecedeEditTransactionCurrentYear,

    /// Goes to the next year state for the date picker in `Transaction` editing.
    /// Data passed: nothing
    AdvanceEditTransactionCurrentYear,

    /// Updates the `Month` state for the date picker in `Transaction` editing.
    /// Data passed: new `Month`
    UpdateEditTransactionCurrentMonth(Months),

    /// Updates the date state for `Transaction` editing.
    /// Data passed: new `Date`
    UpdateEditTransactionSelectedDate(ResultStack<Date>),

    /// Updates the description state for `Transaction` editing.
    /// Data passed: editor `Action`
    UpdateEditTransactionDescriptionContent(Action),

    /// Updates the current `Tag` for `Transaction` editing.
    /// Data passed: new `Tag` `String`
    UpdateEditTransactionCurrentTagString(String),

    /// Adds a `Tag` for `Transaction` editing.
    /// Data passed: `Tag` `String` to add
    AddEditTransactionTag(String),
    
    /// Removes a `Tag` for `Transaction` editing.
    /// Data passed: `Tag` to remove
    RemoveEditTransactionTag(Tag),
    
    
    
    // tag registry page signals
    /// Expands a `Tag` in the tag registry page.
    /// Data passed: `Tag`
    ExpandTag(Tag),
    
    /// Collapses a `Tag` in the tag registry page.
    /// Data passed: `Tag`
    CollapseTag(Tag),
    
    /// Tells the `App` to set the color of a `Tag` in the `TagRegistry`.
    /// Data passed: `Tag`, `MaterialColor`
    SetTagColor(Tag, MaterialColors),
    
    
    
    // settings page signals
    /// Tells the `App` to change the `Theme`.
    /// Data passed: new theme
    ChangeTheme(AppThemes),
    
    
    
    // saving and loading signals
    /// Tells the `App` that saving has finished.
    /// Data passed: save result
    FinishedSaving(ResultStack<()>),
    
    /// Tells the `App` to open the import file picker.
    /// Data passed: nothing
    OpenImportFilePicker,
    
    /// Tells the `App` that an import file has been selected.
    /// Data passed: `PathBuf` of the selected file
    ImportFileSelected(PathBuf),
    
    /// Tells the `App` to confirm an import.
    /// Data passed: 
    ConfirmImport,
    
    /// Tells the `App` to cancel an import.
    /// Data passed: nothing
    CancelImport,
    
    /// Tells the `App` to open the legacy import file picker.
    /// Data passed: nothing
    OpenLegacyImportFilePicker,
    
    /// Tells the `App` that a legacy import file has been selected.
    /// Data passed: `PathBuf` of the selected file
    LegacyImportFileSelected(PathBuf),
    
    /// Tells the `App` to confirm a legacy import.
    /// Data passed: 
    ConfirmLegacyImport,
    
    /// Tells the `App` to cancel a legacy import.
    /// Data passed: nothing
    CancelLegacyImport,
    
    /// Tells the `App` to create a backup.
    /// Data passed: nothing
    Backup,
    
    /// Tells the `App` that a backup has finished.
    /// Data passed: backup result
    FinishedBackingup(ResultStack<()>),
}