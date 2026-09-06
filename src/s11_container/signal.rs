use std::path::PathBuf;

use iced::widget::text_editor::Action;
use uuid::Uuid;
use crate::s21_vault::bank::{CurrencyExchange, TagRegistry};
use crate::s21_vault::parse::FlowTypes;
use crate::s11_container::app::Pages;
use crate::s21_vault::bank::Filters;
use materialui::components::DatePickerModes;
use materialui::materials::{MaterialThemes, MaterialColors};
use crate::s21_vault::ring_parse::RingParse;
use schrod::Schrod;
use crate::s21_vault::transaction::{Date, Id, Months, Tag};
use crate::s21_vault::trend_parse::{Intervals, TrendParse};
use iced::{Point, Size};

/// Various signal carrier types that allow the `App` to communicate with the ui.
#[derive(Debug, Clone)]
pub enum Signal {
    /// Tells the `App` to process signals related to keybinds.
    /// 
    /// Data passed: `KeybindSignal`
    KeybindSignal(KeybindSignal),
    
    /// Tells the `App` to process signals related to genearl `App` functions.
    /// 
    /// Data passed: `GeneralSignal`
    GeneralSignal(GeneralSignal),
    
    /// Tells the `App` to process signals related to filtering.
    /// 
    /// Data passed: `FilterSignal`
    FilterSignal(FilterSignal),

    /// Tells the `App` to process signals related to the transactions page.
    /// 
    /// Data passed: `TransactionsPageSignal`
    TransactionsPageSignal(TransactionsPageSignal),
    
    /// Tells the `App` to process signals related to adding a new `Transaction`.
    /// 
    /// Data passed: `AddTransactionSignal`
    AddTransactionSignal(AddTransactionSignal),
    
    /// Tells the `App` to process signals related to editing an existing `Transaction`.
    /// 
    /// Data passed: `EditTransactionSignal`
    EditTransactionSignal(EditTransactionSignal),
    
    /// Tells the `App` to process signals related to the `TagRegistry`.
    /// 
    /// Data passed: `TagRegistrySignal`
    TagRegistrySignal(TagRegistrySignal),

    /// Tells the `App` to process signals related to the trends page.
    /// 
    /// Data passed: `TrendsSignal`
    TrendsSignal(TrendsSignal),

    /// Tells the `App` to process signals related to the bit wallets page.
    /// 
    /// Data passed: `BitWalletsPageSignal`
    BitWalletsPageSignal(BitWalletsPageSignal),
    
    /// Tells the `App` to process signals related to adding a new `BitWallet`.
    /// 
    /// Data passed: `AddBitWalletSignal`
    AddBitWalletSignal(AddBitWalletSignal),
    
    /// Tells the `App` to process signals related to editing an existing `BitWallet`.
    /// 
    /// Data passed: `EditBitWalletSignal`
    EditBitWalletSignal(EditBitWalletSignal),
    
    
    /// Tells the `App` to process signals related to settings.
    /// 
    /// Data passed: `SettingsSignal`
    SettingsSignal(SettingsSignal),
    
    /// Tells the `App` to process signals related to saving and loading.
    /// 
    /// Data passed: `SaveDataSignal`
    SaveDataSignal(SaveDataSignal),
}



/// Signals relating to keybinds.
#[derive(Debug, Clone)]
pub enum KeybindSignal {
    /// Tells the `App` to focus the next widget.
    /// 
    /// Data passed: nothing
    FocusNext,
    
    /// Tells the `App` to focus the previous widget.
    /// 
    /// Data passed: nothing
    FocusPrevious,
    
    /// Tells the `App` to add a transaction as the context requires.
    /// 
    /// Data passed: nothing
    AddTransactionKeybind,
    
    /// Tells the `App` to advance the year as the context requires.
    /// 
    /// Data passed: nothing
    AdvanceYearKeybind,
    
    /// Tells the `App` to recede the year as the context requires.
    /// 
    /// Data passed: nothing
    RecedeYearKeybind,
    
    /// Tells the `App` to advance the month as the context requires.
    /// 
    /// Data passed: nothing
    AdvanceMonthKeybind,
    
    /// Tells the `App` to recede the month as the context requires.
    /// 
    /// Data passed: nothing
    RecedeMonthKeybind,
    
    /// Tells the `App` to advance the day as the context requires.
    /// 
    /// Data passed: nothing
    AdvanceDayKeybind,
    
    /// Tells the `App` to recede the day as the context requires.
    /// 
    /// Data passed: nothing
    RecedeDayKeybind,
}

/// Signals relating to general `App` functions.
#[derive(Debug, Clone)]
pub enum GeneralSignal {
    /// Tells the `App` to run launch tasks.
    /// 
    /// Data passed: nothing
    Launch,

    /// Tells the `App` that an interaction event loop has concluded.
    /// 
    /// Data passed: nothing
    FinishedInteraction,
    
    /// Tells the `App` that the `CurrencyExchange` has finished updating.
    /// 
    /// Data passed: updated `CurrencyExchange`, refresh result
    FinishedUpdatingCurrencyExchange(CurrencyExchange, Schrod<()>),
    
    /// Tells the `App` that the `TagRegistry` has finished updating.
    /// 
    /// Data passed: updated `TagRegistry`
    FinishedUpdatingTagRegistry(TagRegistry),
    
    /// Tells the `App` that an action is not allowed.
    /// 
    /// Data passed: error message
    InvalidAction(String),
    
    /// Tells the `App` to dismiss all critical errors.
    /// 
    /// Data passed: nothing
    DismissCriticalErrors,
    
    /// Tells the `App` to dismiss all minor errors.
    /// 
    /// Data passed: nothing
    DismissMinorErrors,
    
    /// Tells the `App` to dismiss all warnings.
    /// 
    /// Data passed: nothing
    DismissWarnings,
    
    /// Tells the `App` to change the `Page`.
    /// 
    /// Data passed: new `Page`
    ChangePageTo(Pages),

    /// Tells the `App` to return to the home `Page`.
    /// 
    /// Data passed: nothing
    GoHome,
    
    /// Tells the `App` to show the help page.
    /// 
    /// Data passed: nothing
    HelpMe,
    
    /// Tells the `App` to hide the help page.
    /// 
    /// Data passed: nothing
    DontHelpMe,
}

/// Signals relating to filtering.
#[derive(Debug, Clone)]
pub enum FilterSignal {
    /// Tells the `App` to set the filter year.
    /// 
    /// Data passed: year, `Filter`
    SetFilterYear(u32, Filters),
    
    /// Tells the `App` to clear the filter year.
    /// 
    /// Data passed: `Filter`
    ClearFilterYear(Filters),
    
    /// Tells the `App` to set the filter month.
    /// 
    /// Data passed: `Month`, `Filter`
    SetFilterMonth(Months, Filters),
    
    /// Tells the `App` to clear the filter month.
    /// 
    /// Data passed: `Filter`
    ClearFilterMonth(Filters),
    
    /// Tells the `App` to add a `Tag` to the given `Filter`.
    /// 
    /// Data passed: `Tag`, `Filter`
    AddFilterTag(Tag, Filters),

    /// Tells the `App` to remove a `Tag` from the given `Filter`.
    /// 
    /// Data passed: `Tag`, `Filter`
    RemoveFilterTag(Tag, Filters),
    
    /// Tells the `App` to clear all `Tag`s from the given `Filter`.
    /// 
    /// Data passed: `Filter`
    ClearFilterTags(Filters),
    
    /// Tells the `App` to update the current search term string for the `primary_filter`.
    /// 
    /// Data passed: new search term
    UpdatePrimaryFilterCurrentSearchTermString(String),
    
    /// Tells the `App` to update the current search term string for the `deep_dive_1_filter`.
    /// 
    /// Data passed: new search term
    UpdateDeepDive1FilterCurrentSearchTermString(String),
    
    /// Tells the `App` to update the current search term string for the `deep_dive_2_filter`.
    /// 
    /// Data passed: new search term
    UpdateDeepDive2FilterCurrentSearchTermString(String),
    
    /// Tells the `App` to add a search term to the given `Filter`.
    /// 
    /// Data passed: `Filter`
    AddFilterSearchTerm(Filters),
    
    /// Tells the `App` to remove a search term from the given `Filter`.
    /// 
    /// Data passed: search term, `Filter`
    RemoveFilterSearchTerm(String, Filters),
    
    /// Tells the `App` to clear all search terms from the given `Filter`.
    /// 
    /// Data passed: `Filter`
    ClearFilterSearchTerms(Filters),
    
    /// Tells the `App` to toggle the `Filter` mode for the given `Filter`.
    /// 
    /// Data passed: `Filter`
    ToggleFilterMode(Filters),
}

/// Signals relating to the transactions page.
#[derive(Debug, Clone)]
pub enum TransactionsPageSignal {
    /// Tells the `App` to start adding a new `Transaction`.
    /// 
    /// Data passed: nothing
    StartAddingTransaction,

    /// Tells the `App` to start editing a `Transaction`.
    /// 
    /// Data passed: transaction id
    StartEditingTransaction(Schrod<Id>),
    
    /// Tells the `App` that the `RingChart` has started rendering.
    /// 
    /// Data passed: nothing
    StartedRenderingRingCharts,
    
    /// Tells the `App` that the `RingChart` has finished rendering.
    /// 
    /// Data passed: rendered `RingParse` (in a `ResultStack` to match `App` implementation), render results - one set for each chart
    FinishedRenderingRingCharts(Box<(Schrod<RingParse>, Schrod<()>)>, Box<(Schrod<RingParse>, Schrod<()>)>),
    
    /// Tells the `App` that the mouse has moved in the earning `RingChart`.
    /// 
    /// Data passed: new mouse position, layout size
    MouseMovedInEarningRingChart(Point, Size),
    
    /// Tells the `App` that the mouse has moved in the spending `RingChart`.
    /// 
    /// Data passed: new mouse position, layout size
    MouseMovedInSpendingRingChart(Point, Size),
    
    /// Tells the `App` that the mouse has left the earning `RingChart`.
    /// 
    /// Data passed: nothing
    MouseExitedEarningRingChart,
    
    /// Tells the `App` that the mouse has left the spending `RingChart`.
    /// 
    /// Data passed: nothing
    MouseExitedSpendingRingChart,
    
    /// Tells the `App` to open the tag registry page.
    /// 
    /// Data passed: nothing
    OpenTagRegistry,
}

/// Signals relating to the adding a `Transaction`.
#[derive(Debug, Clone)]
pub enum AddTransactionSignal {
    /// Tells the `App` to add a new `Transaction`.
    /// 
    /// Data passed: nothing
    AddTransaction,

    /// Updates the value state for `Transaction` addition.
    /// 
    /// Data passed: new `Value` `String`
    UpdateNewTransactionValueString(String),

    /// Updates the currency state for `Transaction` addition.
    /// 
    /// Data passed: new `Currency` `String`
    UpdateNewTransactionCurrencyString(String),

    /// Updates the date picker mode in `Transaction` addition.
    /// 
    /// Data passed: new date picker mode
    UpdateNewTransactionDatePickerMode(DatePickerModes),

    /// Goes to the next year state for the date picker in `Transaction` addition.
    /// 
    /// Data passed: nothing
    AdvanceNewTransactionCurrentYear,

    /// Goes to the previous year state for the date picker in `Transaction` addition.
    /// 
    /// Data passed: nothing
    RecedeNewTransactionCurrentYear,

    /// Updates the `Month` state for the date picker in `Transaction` addition.
    /// 
    /// Data passed: new `Month`
    UpdateNewTransactionCurrentMonth(Months),

    /// Updates the date state for `Transaction` addition.
    /// 
    /// Data passed: new `Date`
    UpdateNewTransactionSelectedDate(Schrod<Date>),

    /// Updates the description state for `Transaction` adding.
    /// 
    /// Data passed: editor `Action`
    UpdateNewTransactionDescriptionContent(Action),

    /// Updates the current tag for `Transaction` adding.
    /// 
    /// Data passed: new `Tag` `String`
    UpdateNewTransactionCurrentTagString(String),

    /// Adds a tag for `Transaction` adding.
    /// 
    /// Data passed: `Tag` `String` to add
    AddNewTransactionTag(String),

    /// Removes a `Tag` for `Transaction` adding.
    /// 
    /// Data passed: `Tag` to remove
    RemoveNewTransactionTag(Tag),
}

/// Signals relating to the editing a `Transaction`.
#[derive(Debug, Clone)]
pub enum EditTransactionSignal {
    /// Tells the `App` to edit an existing `Transaction`.
    /// 
    /// Data passed: nothing
    EditTransaction,

    /// Tells the `App` to prime the `Transaction` being edited for deleting.
    /// 
    /// Data passed: nothing
    PrimeRemoveTransaction,
    
    /// Tells the `App` to unprime the `Transaction` being edited for deleting.
    /// 
    /// Data passed: nothing
    UnprimeRemoveTransaction,
    
    /// Tells the `App` to remove the `Transaction` being edited.
    /// 
    /// Data passed: nothing
    RemoveTransaction,
    
    /// Updates the value state for `Transaction` editing.
    /// 
    /// Data passed: new `Value` `String`
    UpdateEditTransactionValueString(String),

    /// Updates the currency state for `Transaction` editing.
    /// 
    /// Data passed: new `Currency` `String`
    UpdateEditTransactionCurrencyString(String),

    /// Updates the date picker mode in `Transaction` editing.
    /// 
    /// Data passed: new date picker mode
    UpdateEditTransactionDatePickerMode(DatePickerModes),

    /// Goes to the previous year state for the date picker in `Transaction` editing.
    /// 
    /// Data passed: nothing
    RecedeEditTransactionCurrentYear,

    /// Goes to the next year state for the date picker in `Transaction` editing.
    /// 
    /// Data passed: nothing
    AdvanceEditTransactionCurrentYear,

    /// Updates the `Month` state for the date picker in `Transaction` editing.
    /// 
    /// Data passed: new `Month`
    UpdateEditTransactionCurrentMonth(Months),

    /// Updates the date state for `Transaction` editing.
    /// 
    /// Data passed: new `Date`
    UpdateEditTransactionSelectedDate(Schrod<Date>),

    /// Updates the description state for `Transaction` editing.
    /// 
    /// Data passed: editor `Action`
    UpdateEditTransactionDescriptionContent(Action),

    /// Updates the current `Tag` for `Transaction` editing.
    /// 
    /// Data passed: new `Tag` `String`
    UpdateEditTransactionCurrentTagString(String),

    /// Adds a `Tag` for `Transaction` editing.
    /// 
    /// Data passed: `Tag` `String` to add
    AddEditTransactionTag(String),
    
    /// Removes a `Tag` for `Transaction` editing.
    /// 
    /// Data passed: `Tag` to remove
    RemoveEditTransactionTag(Tag),
}

/// Signals relating to the `TagRegistry`.
#[derive(Debug, Clone)]
pub enum TagRegistrySignal {
    /// Expands a `Tag` in the tag registry page.
    /// 
    /// Data passed: `Tag`
    ExpandTag(Tag),
    
    /// Collapses a `Tag` in the tag registry page.
    /// 
    /// Data passed: `Tag`
    CollapseTag(Tag),
    
    /// Tells the `App` to set the color of a `Tag` in the `TagRegistry`.
    /// 
    /// Data passed: `Tag`, `MaterialColor`
    SetTagColor(Tag, MaterialColors),

    /// Tells the `App` to reset the color of a `Tag` in the `TagRegistry`.
    /// 
    /// Data passed: `Tag`
    ResetTag(Tag),
}

/// Signals relating to the trends page.
#[derive(Debug, Clone)]
pub enum TrendsSignal {
    /// Tells the `App` to set the interval of the `TrendParse`.
    /// 
    /// Data passed: new interval
    SetTrendingInterval(Intervals),
    
    /// Tells the `App` to toggle the visibility of the overall balance line in the `TrendParse`.
    /// 
    /// Data passed: nothing
    ToggleShowBalance,

    /// Tells the `App` to add a `Tag` to the `TrendParse`.
    /// 
    /// Data passed: `Tag`
    AddTrendingTag(Tag),

    /// Tells the `App` to remove a `Tag` from the `TrendParse`.
    /// 
    /// Data passed: `Tag`
    RemoveTrendingTag(Tag),

    /// Tells the `App` to extend the trending length of the `TrendParse`.
    /// 
    /// Data passed: nothing
    ExtendTrendingLength,

    /// Tells the `App` to reduce the trending length of the `TrendParse`.
    /// 
    /// Data passed: nothing
    ReduceTrendingLength,
    
    /// Tells the `App` that the `TrendParse` has started rendering.
    /// 
    /// Data passed: nothing
    StartedRenderingTrendParse,

    /// Tells the `App` that the `TrendParse` has finished rendering.
    /// 
    /// Data passed: rendered `TrendParse`, render results
    FinishedRenderingTrendParse(TrendParse, Schrod<()>),

    /// Tells the `App` that the `TrendParse` failed to render.
    /// 
    /// Data passed: nothing
    FailedToRenderTrendParse,
}

/// Signals relating to the bit wallets page.
#[derive(Debug, Clone)]
pub enum BitWalletsPageSignal {
    /// Tells the `App` to start adding a new `BitWallet`.
    /// 
    /// Data passed: nothing
    StartAddingBitWallet,

    /// Tells the `App` to start editing a `BitWallet`.
    /// 
    /// Data passed: transaction id
    StartEditingBitWallet(Uuid),
    
    /// Tells the `App` to open a given `BitWallet`.
    /// 
    /// Data passed: `BitWallet` id
    OpenBitWallet(Uuid),
}

/// Signals relating to the adding a `BitWallet`.
#[derive(Debug, Clone)]
pub enum AddBitWalletSignal {
    /// Tells the `App` to add a new `BitWallet`.
    /// 
    /// Data passed: nothing
    AddBitWallet,

    /// Updates the name state for `BitWallet` addition.
    /// 
    /// Data passed: new `name` `String`
    UpdateNewBitWalletNameString(String),

    /// Updates the coin state for `BitWallet` addition.
    /// 
    /// Data passed: new `Coin` `String`
    UpdateNewBitWalletCoinString(String),
}

/// Signals relating to the editing a `BitWallet`.
#[derive(Debug, Clone)]
pub enum EditBitWalletSignal {
    /// Tells the `App` to edit an existing `BitWallet`.
    /// 
    /// Data passed: nothing
    EditBitWallet,

    /// Tells the `App` to prime the `BitWallet` being edited for deleting.
    /// 
    /// Data passed: nothing
    PrimeRemoveBitWallet,
    
    /// Tells the `App` to unprime the `BitWallet` being edited for deleting.
    /// 
    /// Data passed: nothing
    UnprimeRemoveBitWallet,
    
    /// Tells the `App` to remove the `BitWallet` being edited.
    /// 
    /// Data passed: nothing
    RemoveBitWallet,
    
    /// Updates the name state for `BitWallet` editing.
    /// 
    /// Data passed: new `name` `String`
    UpdateEditBitWalletNameString(String),

    /// Updates the coin state for `BitWallet` editing.
    /// 
    /// Data passed: new `Coin` `String`
    UpdateEditBitWalletCoinString(String),
}

/// Signals relating to the settings page.
#[derive(Debug, Clone)]
pub enum SettingsSignal {
    /// Tells the `App` to change the `Theme`.
    /// 
    /// Data passed: new theme
    ChangeTheme(MaterialThemes),

    /// Tells the `App` to toggle the `developer_mode` setting.
    /// 
    /// Data passed: nothing
    ToggleDeveloperMode,

    /// Tells the `App` to update the `new_main_currency_string` of the `App`.
    /// 
    /// Data passed: new `Currency` `String`
    UpdateNewMainCurrencyString(String),

    /// Tells the `App` to set the main `Currency`.
    /// 
    /// Data passed: nothing
    SetMainCurrency,

    /// Tells the `App` to update the `new_time_price_string` of the `App`.
    /// 
    /// Data passed: new time price `String`
    UpdateNewTimePriceString(String),

    /// Tells the `App` to set the time price.
    /// 
    /// Data passed: nothing
    SetTimePrice,

    /// Tells the `App` to set the flow type in the `CurrencyExchange`.
    /// 
    /// Data passed: new flow type
    SetFlowType(FlowTypes),

    /// Tells the `App` to update the `new_rate_string` of an `ExchangeRate`.
    /// 
    /// Data passed: `from_string`, `to_string`, `new_rate_string`
    UpdateNewExchangeRateString(String, String, String),

    /// Tells the `App` to try setting a new `ExchangeRate`.
    /// 
    /// Data passed: `from_string`, `to_string`, `new_rate_string`
    TrySetNewExchangeRate(String, String, String),
}

/// Signals relating to saving and loading.
#[derive(Debug, Clone)]
pub enum SaveDataSignal {
    /// Tells the `App` that saving has finished.
    /// 
    /// Data passed: save result
    FinishedSaving(Schrod<()>),
    
    /// Tells the `App` to open the import file picker.
    /// 
    /// Data passed: nothing
    OpenImportFilePicker,
    
    /// Tells the `App` that an import file has been selected.
    /// 
    /// Data passed: `PathBuf` of the selected file
    ImportFileSelected(PathBuf),
    
    /// Tells the `App` to confirm an import.
    /// 
    /// Data passed: 
    ConfirmImport,
    
    /// Tells the `App` to cancel an import.
    /// 
    /// Data passed: nothing
    CancelImport,
    
    /// Tells the `App` to open the legacy import file picker.
    /// 
    /// Data passed: nothing
    OpenLegacyImportFilePicker,
    
    /// Tells the `App` that a legacy import file has been selected.
    /// 
    /// Data passed: `PathBuf` of the selected file
    LegacyImportFileSelected(PathBuf),
    
    /// Tells the `App` to confirm a legacy import.
    /// 
    /// Data passed: 
    ConfirmLegacyImport,
    
    /// Tells the `App` to cancel a legacy import.
    /// 
    /// Data passed: nothing
    CancelLegacyImport,
    
    /// Tells the `App` to create a backup.
    /// 
    /// Data passed: nothing
    Backup,
    
    /// Tells the `App` that a backup has finished.
    /// 
    /// Data passed: backup result
    FinishedBackingup(Schrod<()>),

    /// Tells the `App` to open a file explorer at the location of the save data.
    /// 
    /// Data passed: nothing
    OpenDataLocation,
}