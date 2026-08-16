use std::borrow::Cow;

use iced::keyboard::key::Named;
use iced::widget::operation::{focus_next, focus_previous};
use iced::{Element, Event, Subscription, Task, Theme, event, keyboard};
use iced::widget::text_editor::Content;
use materialui::materials::MaterialThemes;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::bit_vault::bit_bank::BitBank;
use crate::container::signal::{AddTransactionSignal, EditTransactionSignal, FilterSignal, GeneralSignal, KeybindSignal, SaveDataSignal, SettingsSignal, Signal, TagRegistrySignal, TransactionsPageSignal, TrendsSignal};
use crate::container::state::{AppState, BankState, FilterState, RingChartsState, SaveState, SettingsState, TransactionState, TrendsState, TagRegistrationSlipStateManager};
use crate::container::warnings::Warnings;
use crate::settings_pages::confirm_import_page::confirm_import_page;
use crate::settings_pages::confirm_legacy_import_page::confirm_legacy_import_page;
use crate::vault_pages::help_page::{help_button, help_page};
use crate::error_pages::minor_errors_page::minor_errors_page;
use crate::settings_pages::settings_page::settings_page;
use crate::vault_pages::transaction_management_pages::{add_transaction_page, edit_transaction_page};
use crate::vault_pages::transactions_page::transactions_page;
use crate::vault_pages::tag_registry_page::tag_registry_page;
use crate::error_pages::critical_errors_page::critical_errors_page;
use crate::vault_pages::trends_page::trends_page;
use crate::error_pages::warnings_page::warnings_page;
use materialui::components::{DatePickerModes, PageProvider, ThemeProvider, page_pointer};
use crate::vault::bank::{Bank, CurrencyExchange, Filters, TagRegistry};
use crate::vault::parse::CashFlow;
use crate::vault::ring_parse::{FlowDirections, RingParse, Segment};
use crate::container::save_engine::legacy::load_legacy_from;
use crate::vault::transaction::{Date, Id, Months, Tag, Transaction/*, ValueDisplayFormats*/};
use schrod::Schrod;
use schrod::Schrod::{Pass, Fail};
use crate::vault::trend_parse::{Intervals, TrendParse};
use iced::futures::SinkExt;
use iced::futures::channel::mpsc::Sender;
use crate::container::save_engine::{self, SaveData, backup, load, load_from, save};

/// The available pages in the `App`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    Trends,
    TagRegistry,
    Settings,
    ConfirmImport,
    ConfirmLegacyImport,
    WarningsPage,
    MinorErrorsPage,
}
impl Pages {
    /// Returns the name for a given `Page`.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Pages::Transactions => { "Transactions".to_string() }
            Pages::AddingTransaction => { "Adding Transaction".to_string() }
            Pages::EditingTransaction => { "Editing Transaction".to_string() }
            Pages::Trends => { "Trends".to_string() }
            Pages::TagRegistry => { "Tag Registry".to_string() }
            Pages::Settings => { "Settings".to_string() }
            Pages::ConfirmImport => { "Confirm Import".to_string() }
            Pages::ConfirmLegacyImport => { "Confirm Legacy Import".to_string() }
            Pages::WarningsPage => { "Warnings".to_string() }
            Pages::MinorErrorsPage => { "Minor Errors".to_string() }
        }
    }
    
    /// Returns the icon name for a given `Page`.
    #[must_use]
    pub fn icon_name(&self) -> String {
        match self {
            Pages::Transactions => "money-bill".to_string(),
            Pages::AddingTransaction => "plus".to_string(),
            Pages::EditingTransaction => "pencil".to_string(),
            Pages::Trends => "arrow-trend-up".to_string(),
            Pages::TagRegistry => "tags".to_string(),
            Pages::Settings => "gear".to_string(),
            Pages::ConfirmImport | Pages::ConfirmLegacyImport => "file-import".to_string(),
            Pages::WarningsPage => "triangle-exclamation".to_string(),
            Pages::MinorErrorsPage => "circle-exclamation".to_string(),
        }
    }

    /// Returns the list of selectable `Page`s.
    #[must_use]
    pub fn page_pointers<'a>(app: &'a App) -> Vec<Element<'a, Signal>> {
        let pages = vec![
            Pages::Transactions,
            Pages::Trends,
            Pages::TagRegistry,
            Pages::Settings,
        ];
        
        let mut page_pionters: Vec<_> = pages
            .into_iter()
            .map(|page| page_pointer(app, &page.name(), &page.icon_name(), app.app_state.page() == page, Signal::GeneralSignal(GeneralSignal::ChangePageTo(page)), true))
            .collect();
        page_pionters.push(help_button(app));
        page_pionters
    }
}



/// The central application container.
/// This holds the `Bank` and all ui/ux state information.
#[allow(clippy::struct_excessive_bools)] // This is more ergonomic than using enums for bool flags.
pub struct App {
    app_state: AppState,
    save_state: SaveState,
    settings_state: SettingsState,
    
    bank: Bank,
    bit_bank: BitBank,
    bank_state: BankState,
    new_transaction_state: TransactionState,
    edit_transaction_state: TransactionState,
    tag_registry_slip_state_manager: TagRegistrationSlipStateManager,
    
    filter_state: FilterState,
    ring_chart_state: RingChartsState,
    trends_state: TrendsState,
}
impl PageProvider for App {
    fn page_name(&self) -> String { self.app_state.page().name().to_string() }
    fn page_icon(&self) -> String { self.app_state.page().icon_name() }
}
impl ThemeProvider for App {
    fn material_theme(&self) -> MaterialThemes { self.app_state.material_theme() }
}
impl App {
    // initializing
    /// Creates a new `App`.
    #[must_use]
    pub fn new() -> (App, Task<Signal>) {
        // loading failure tracking
        let mut loaded_successfully = true;
        let mut initializing_failures = Vec::<Schrod<()>>::new();
        
        // general failure tracking
        let mut general_failures = Vec::<Schrod<()>>::new();
        
        // getting the save data
        let save_data_result = load();
        if save_data_result.is_fail() {
            loaded_successfully = false;
            initializing_failures.push(save_data_result.convert("App::new()").clone());
        }
        
        // loading the theme
        let theme = match &save_data_result {
            Schrod::Pass(save_data) => save_data.theme,
            Schrod::Fail(_) => MaterialThemes::Midnight,
        };
        
        // loading the transactions
        let transactions = match &save_data_result {
            Schrod::Pass(save_data) => save_data.transactions.clone(),
            Schrod::Fail(_) => Vec::new(),
        };

        // loading the currency exchange
        let currency_exchange = match &save_data_result {
            Schrod::Pass(save_data) => save_data.currency_exchange.clone(),
            Schrod::Fail(_) => CurrencyExchange::default(),
        };
        
        // loading the tag registry
        let tag_registry = match &save_data_result {
            Schrod::Pass(save_data) => save_data.tag_registry.clone(),
            Schrod::Fail(_) => TagRegistry::default(),
        };

        // loading the bit wallets
        let bit_wallets = match &save_data_result {
            Schrod::Pass(save_data) => save_data.bit_wallets.clone(),
            Schrod::Fail(_) => Vec::new(),
        };
        
        // loading the bank
        let mut bank = Bank::default();
        let bank_init_result = bank.init(transactions, currency_exchange, tag_registry);
        if bank_init_result.is_fail() { initializing_failures.push(bank_init_result.convert("App::new()").clone()); }
        let tags = bank.get_tags();
        
        // bank display state
        let cash_flow_result = CashFlow::new(&bank, &bank.get_filtered_ids(Filters::Primary));
        if cash_flow_result.is_fail() { general_failures.push(cash_flow_result.convert("App::new()").clone()); }

        // the latest date in the bank's ledger
        let latest_date = bank.get_latest_date();

        // loading the bit bank
        let mut bit_bank = BitBank::default();
        let bit_bank_init_result = bit_bank.init(bit_wallets);
        if bit_bank_init_result.is_fail() { initializing_failures.push(bit_bank_init_result.convert("App::new()").clone()); }
        
        
        // creates the app
        let mut app = App {
            app_state: AppState::new(theme, theme.generate_iced_palette()),
            save_state: SaveState::new(loaded_successfully),
            settings_state: SettingsState::new(),
            
            bank: bank,
            bit_bank: bit_bank,
            bank_state: BankState::new(cash_flow_result),
            new_transaction_state: TransactionState::new(None, latest_date),
            edit_transaction_state: TransactionState::new(None, latest_date),
            tag_registry_slip_state_manager: TagRegistrationSlipStateManager::new(tags),
            
            filter_state: FilterState::new(),
            ring_chart_state: RingChartsState::new(),
            trends_state: TrendsState::new(latest_date),
        };
        
        // checking for loading failures
        for error in initializing_failures { app.get_app_state_mut().pass_error(error); }
        
        // checking for other failures
        for error in general_failures { app.get_app_state_mut().pass_error(error); }
        
        // returning the app
        (app, Task::done(Signal::GeneralSignal(GeneralSignal::Launch)))
    }


    
    // getters
    /// The tile of the `App`.
    #[must_use]
    pub fn title(&self) -> String { "Ascent".to_string() }
    
    /// Gets the current `Iced` `Theme`.
    #[must_use]
    pub fn iced_theme(&self) -> Theme { self.app_state.iced_theme().clone() }

    /// Gets the `Bank` (immutable).
    #[must_use]
    pub fn get_bank(&self) -> &Bank { &self.bank }

    /// Gets the `BitBank` (immutable).
    #[must_use]
    pub fn get_bit_bank(&self) -> &BitBank { &self.bit_bank }
    
    /// Gets the `AppState` (immutable).
    #[must_use]
    pub fn get_app_state(&self) -> &AppState { &self.app_state }
    
    /// Gets the `AppState` (mutable).
    #[must_use]
    pub fn get_app_state_mut(&mut self) -> &mut AppState { &mut self.app_state }

    /// Gets the `SaveState` (immutable).
    #[must_use]
    pub fn get_save_state(&self) -> &SaveState { &self.save_state }

    /// Gets the `SaveState` (mutable).
    #[must_use]
    pub fn get_save_state_mut(&mut self) -> &mut SaveState { &mut self.save_state }
    
    /// Gets the `SettingsState` (immutable).
    #[must_use]
    pub fn get_settings_state(&self) -> &SettingsState { &self.settings_state }

    /// Gets the `SettingsState` (mutable).
    #[must_use]
    pub fn get_settings_state_mut(&mut self) -> &mut SettingsState { &mut self.settings_state }

    /// Gets the `BankState` (immutable).
    #[must_use]
    pub fn get_bank_state(&self) -> &BankState { &self.bank_state }

    /// Gets the `BankState` (mutable).
    #[must_use]
    pub fn get_bank_state_mut(&mut self) -> &mut BankState { &mut self.bank_state }

    /// Gets the `TransactionState` for adding a new `Transaction` (immutable).
    #[must_use]
    pub fn get_new_transaction_state(&self) -> &TransactionState { &self.new_transaction_state }

    /// Gets the `TransactionState` for adding a new `Transaction` (mutable).
    #[must_use]
    pub fn get_new_transaction_state_mut(&mut self) -> &mut TransactionState { &mut self.new_transaction_state }
    
    /// Gets the `TransactionState` for editing an existing `Transaction` (immutable).
    #[must_use]
    pub fn get_edit_transaction_state(&self) -> &TransactionState { &self.edit_transaction_state }

    /// Gets the `TransactionState` for editing an existing `Transaction` (mutable).
    #[must_use]
    pub fn get_edit_transaction_state_mut(&mut self) -> &mut TransactionState { &mut self.edit_transaction_state }
    
    /// Gets the `TagRegistrationSlipStateManager` (immutable).
    #[must_use]
    pub fn get_tag_registry_slip_state_manager(&self) -> &TagRegistrationSlipStateManager { &self.tag_registry_slip_state_manager }

    /// Gets the `TagRegistrationSlipStateManager` (mutable).
    #[must_use]
    pub fn get_tag_registry_slip_state_manager_mut(&mut self) -> &mut TagRegistrationSlipStateManager { &mut self.tag_registry_slip_state_manager }
    
    /// Gets the `FilterState` (immutable).
    #[must_use]
    pub fn get_filter_state(&self) -> &FilterState { &self.filter_state }

    /// Gets the `FilterState` (mutable).
    #[must_use]
    pub fn get_filter_state_mut(&mut self) -> &mut FilterState { &mut self.filter_state }

    /// Gets the `RingChartsState` (immutable).
    #[must_use]
    pub fn get_ring_chart_state(&self) -> &RingChartsState { &self.ring_chart_state }

    /// Gets the `RingChartsState` (mutable).
    #[must_use]
    pub fn get_ring_chart_state_mut(&mut self) -> &mut RingChartsState { &mut self.ring_chart_state }

    /// Gets the `TrendsState` (immutable).
    #[must_use]
    pub fn get_trends_state(&self) -> &TrendsState { &self.trends_state }

    /// Gets the `TrendsState` (mutable).
    #[must_use]
    pub fn get_trends_state_mut(&mut self) -> &mut TrendsState { &mut self.trends_state }


    
    // running
    /// Processes signals related to keybinds.
    #[must_use]
    fn process_keybind_signal(&mut self, signal: KeybindSignal) -> Task<Signal> {
        match signal {
            KeybindSignal::FocusNext => { focus_next() }
            
            KeybindSignal::FocusPrevious => { focus_previous() }
            
            KeybindSignal::AddTransactionKeybind => {
                match self.app_state.page() {
                    Pages::Transactions => { Task::done(Signal::TransactionsPageSignal(TransactionsPageSignal::StartAddingTransaction)) }
                    Pages::AddingTransaction => {
                        if Transaction::are_raw_parts_valid(
                            &self.new_transaction_state.value_string(),
                            &self.new_transaction_state.currency_string(),
                            &self.new_transaction_state.description_content().text(),
                            &self.new_transaction_state.tags()) {
                            Task::done(Signal::AddTransactionSignal(AddTransactionSignal::AddTransaction))
                        }
                        else { Task::none() }
                    }
                    Pages::EditingTransaction => {
                        if Transaction::are_raw_parts_valid(
                            &self.edit_transaction_state.value_string(),
                            &self.edit_transaction_state.currency_string(),
                            &self.edit_transaction_state.description_content().text(),
                            &self.edit_transaction_state.tags()) {
                            Task::done(Signal::EditTransactionSignal(EditTransactionSignal::EditTransaction))
                        }
                        else { Task::none() }
                    }
                    _ => { Task::none() }
                }
            }
            
            KeybindSignal::AdvanceYearKeybind => {
                match self.app_state.page() {
                    Pages::Transactions => {
                        if let Some(current_year) = self.bank.get_filter(Filters::Primary).get_filter_year() {
                            Task::done(Signal::FilterSignal(FilterSignal::SetFilterYear(Date::get_advanced_year(current_year), Filters::Primary)))
                        }
                        else { Task::done(Signal::FilterSignal(FilterSignal::SetFilterYear(self.bank.get_latest_date_for_filter(Filters::Primary).get_year(), Filters::Primary))) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.date_picker_state().selected_date();
                        new_date.advance_by_year();
                        Task::done(Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.date_picker_state().selected_date();
                        new_date.advance_by_year();
                        Task::done(Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            KeybindSignal::RecedeYearKeybind => {
                match self.app_state.page() {
                    
                    Pages::Transactions => {
                        if let Some(current_year) = self.bank.get_filter(Filters::Primary).get_filter_year() {
                            Task::done(Signal::FilterSignal(FilterSignal::SetFilterYear(Date::get_receded_year(current_year), Filters::Primary)))
                        }
                        else { Task::done(Signal::FilterSignal(FilterSignal::SetFilterYear(self.bank.get_latest_date_for_filter(Filters::Primary).get_year(), Filters::Primary))) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.date_picker_state().selected_date();
                        new_date.recede_by_year();
                        Task::done(Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.date_picker_state().selected_date();
                        new_date.recede_by_year();
                        Task::done(Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            KeybindSignal::AdvanceMonthKeybind => {
                match self.app_state.page() {
                    Pages::Transactions => {
                        if let Some(current_month) = self.bank.get_filter(Filters::Primary).get_filter_month() {
                            Task::done(Signal::FilterSignal(FilterSignal::SetFilterMonth(current_month.get_next(), Filters::Primary)))
                        }
                        else { Task::done(Signal::FilterSignal(FilterSignal::SetFilterMonth(self.bank.get_latest_date_for_filter(Filters::Primary).get_month(), Filters::Primary))) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.date_picker_state().selected_date();
                        new_date.advance_by_month();
                        Task::done(Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.date_picker_state().selected_date();
                        new_date.advance_by_month();
                        Task::done(Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            KeybindSignal::RecedeMonthKeybind => {
                match self.app_state.page() {
                    Pages::Transactions => {
                        if let Some(current_month) = self.bank.get_filter(Filters::Primary).get_filter_month() {
                            Task::done(Signal::FilterSignal(FilterSignal::SetFilterMonth(current_month.get_previous(), Filters::Primary)))
                        }
                        else { Task::done(Signal::FilterSignal(FilterSignal::SetFilterMonth(self.bank.get_latest_date_for_filter(Filters::Primary).get_month(), Filters::Primary))) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.date_picker_state().selected_date();
                        new_date.recede_by_month();
                        Task::done(Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.date_picker_state().selected_date();
                        new_date.recede_by_month();
                        Task::done(Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            KeybindSignal::AdvanceDayKeybind => {
                match self.app_state.page() {
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.date_picker_state().selected_date();
                        new_date.advance_by_day();
                        Task::done(Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.date_picker_state().selected_date();
                        new_date.advance_by_day();
                        Task::done(Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            KeybindSignal::RecedeDayKeybind => {
                match self.app_state.page() {
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.date_picker_state().selected_date();
                        new_date.recede_by_day();
                        Task::done(Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.date_picker_state().selected_date();
                        new_date.recede_by_day();
                        Task::done(Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Pass(new_date))))
                    }
                    
                    _ => { Task::none() }
                }
            }
        }
    }

    /// Processes signals related to general `App` functions.
    #[must_use]
    fn process_general_signal(&mut self, signal: GeneralSignal) -> Task<Signal> {
        match signal {
            GeneralSignal::Launch => {
                Task::batch(vec![
                    self.refresh_currency_exchange_task(),
                    self.update_tag_registry_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            GeneralSignal::FinishedInteraction => {
                self.app_state.finished_interaction(&self.bank);
                Task::none()
            }
            
            GeneralSignal::FinishedUpdatingCurrencyExchange(updated_currency_exchange, refresh_result) => {
                self.bank.currency_exchange = updated_currency_exchange;
                if refresh_result.is_fail() { self.app_state.pass_error(refresh_result); }
                Task::none()
            }
            
            GeneralSignal::FinishedUpdatingTagRegistry(updated_tag_registry) => {
                self.bank.tag_registry = updated_tag_registry;
                let tags = self.bank.get_tags();
                self.tag_registry_slip_state_manager = TagRegistrationSlipStateManager::new(tags);
                Task::none()
            }
            
            GeneralSignal::InvalidAction(_) => {
                eprintln!("Invalid action!");
                Task::none()
            }
            
            GeneralSignal::DismissCriticalErrors => {
                self.app_state.clear_critical_errors();
                Task::none()
            }
            
            GeneralSignal::DismissMinorErrors => {
                self.app_state.clear_minor_errors();
                self.app_state.update_page(Pages::WarningsPage);
                Task::none()
            }
            
            GeneralSignal::DismissWarnings => {
                self.app_state.clear_minor_errors();
                self.app_state.clear_warnings();
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            GeneralSignal::ChangePageTo(page) => {
                self.app_state.update_page(page);
                if page == Pages::Settings { self.refresh_currency_exchange_task() }
                else { Task::none() }
            }

            GeneralSignal::GoHome => {
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            GeneralSignal::HelpMe => {
                self.app_state.update_is_helping(true);
                Task::none()
            }
            
            GeneralSignal::DontHelpMe => {
                self.app_state.update_is_helping(false);
                Task::none()
            }
        }
    }

    /// Processes signals related to filtering.
    #[must_use]
    fn process_filter_signal(&mut self, signal: FilterSignal) -> Task<Signal> {
        match signal {
            FilterSignal::SetFilterYear(year, filter) => {
                let filter_result = self.bank.set_filter_year(year, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        
            FilterSignal::ClearFilterYear(filter) => {
                let filter_result = self.bank.clear_filter_year(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        
            FilterSignal::SetFilterMonth(month, filter) => {
                let filter_result = self.bank.set_filter_month(month, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        
            FilterSignal::ClearFilterMonth(filter) => {
                let filter_result = self.bank.clear_filter_month(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
            
            FilterSignal::AddFilterTag(tag, filter) => {
                let filter_result = self.bank.add_filter_tag(&tag, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
    
            FilterSignal::RemoveFilterTag(tag, filter) => {
                let filter_result = self.bank.remove_filter_tag(&tag, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
            
            FilterSignal::ClearFilterTags(filter) => {
                let filter_result = self.bank.clear_filter_tags(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        
            FilterSignal::UpdatePrimaryFilterCurrentSearchTermString(term) => {
                self.filter_state.update_primary_filter_current_search_term_string(term);
                Task::none()
            }
            
            FilterSignal::UpdateDeepDive1FilterCurrentSearchTermString(term) => {
                self.filter_state.update_deep_dive_1_filter_current_search_term_string(term);
                Task::none()
            }
            
            FilterSignal::UpdateDeepDive2FilterCurrentSearchTermString(term) => {
                self.filter_state.update_deep_dive_2_filter_current_search_term_string(term);
                Task::none()
            }
        
            FilterSignal::AddFilterSearchTerm(filter) => {
                let term = match filter {
                    Filters::Primary => self.filter_state.primary_filter_current_search_term_string(),
                    Filters::DeepDive1 => self.filter_state.deep_dive_1_filter_current_search_term_string(),
                    Filters::DeepDive2 => self.filter_state.deep_dive_2_filter_current_search_term_string(),
                };
                
                match filter {
                    Filters::Primary => self.filter_state.update_primary_filter_current_search_term_string(String::new()),
                    Filters::DeepDive1 => self.filter_state.update_deep_dive_1_filter_current_search_term_string(String::new()),
                    Filters::DeepDive2 => self.filter_state.update_deep_dive_2_filter_current_search_term_string(String::new()),
                }
                
                let filter_result = self.bank.add_filter_search_term(&term, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
            
            FilterSignal::RemoveFilterSearchTerm(term, filter) => {
                let filter_result = self.bank.remove_filter_search_term(&term, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        
            FilterSignal::ClearFilterSearchTerms(filter) => {
                let filter_result = self.bank.clear_filter_search_terms(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
            
            FilterSignal::ToggleFilterMode(filter) => {
                let filter_result = self.bank.toggle_filter_mode(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_ring_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(filter_result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        }
    }

    /// Processes signals related to the transactions page.
    #[must_use]
    fn process_transactions_page_signal(&mut self, signal: TransactionsPageSignal) -> Task<Signal> {
        match signal {
            TransactionsPageSignal::StartAddingTransaction => {
                let current_date = self.bank.get_latest_date_for_filter(Filters::Primary);
            
                self.new_transaction_state.update_value_string(String::new());
                self.new_transaction_state.update_currency_string(String::new());
                self.new_transaction_state.date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                self.new_transaction_state.date_picker_state_mut().update_current_year(current_date.get_year());
                self.new_transaction_state.date_picker_state_mut().update_current_month(current_date.get_month());
                self.new_transaction_state.date_picker_state_mut().update_selected_date(current_date);
                self.new_transaction_state.update_description_content(Content::with_text(""));
                self.new_transaction_state.update_current_tag_string(String::new());
                self.new_transaction_state.update_tags(Vec::new());
                self.app_state.update_page(Pages::AddingTransaction);
            
                Task::none()
            }

            TransactionsPageSignal::StartEditingTransaction(id_result) => {
                if id_result.is_fail() {
                    self.app_state.pass_error(id_result);
                    return Task::none();
                }
                let id = id_result.wont_fail("This is past an is_fail() guard clause.", "App::process_transaction_page_signal() - StartEditingTransaction");
                let transaction_result = self.bank.get(id);
    
                if let Pass(transaction) = transaction_result {
                    self.edit_transaction_state.update_id(Some(id));
                    self.edit_transaction_state.update_value_string(transaction.value.amount().to_string());
                    self.edit_transaction_state.update_currency_string(transaction.value.currency().to_string());
                    self.edit_transaction_state.date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                    self.edit_transaction_state.date_picker_state_mut().update_current_year(transaction.date.get_year());
                    self.edit_transaction_state.date_picker_state_mut().update_current_month(transaction.date.get_month());
                    self.edit_transaction_state.date_picker_state_mut().update_selected_date(transaction.date);
                    self.edit_transaction_state.update_description_content(Content::with_text(&transaction.description));
                    self.edit_transaction_state.update_current_tag_string(String::new());
                    self.edit_transaction_state.update_tags(transaction.tags.clone());
                    self.edit_transaction_state.update_is_delete_primed(false);
                    self.app_state.update_page(Pages::EditingTransaction);
                }
    
                else { self.app_state.pass_error(transaction_result.convert::<String>("App::process_transaction_page_signal() - StartEditingTransaction")); }
                
                Task::none()
            }

            TransactionsPageSignal::StartedRenderingRingCharts => {
                self.ring_chart_state.update_is_ready(false);
                Task::none()
            }
            
            TransactionsPageSignal::FinishedRenderingRingCharts(rendered_earning_ring_parse_result, rendered_spending_ring_parse_result) => {
                let (earning_ring_parse_result, earning_ring_parse_render_results) = *rendered_earning_ring_parse_result;
                let (spending_ring_parse_result, spending_ring_parse_render_results) = *rendered_spending_ring_parse_result;
                self.ring_chart_state.update_earning_result(earning_ring_parse_result);
                self.ring_chart_state.update_spending_result(spending_ring_parse_result);
                if earning_ring_parse_render_results.is_fail() { self.app_state.pass_error(earning_ring_parse_render_results); }
                if spending_ring_parse_render_results.is_fail() { self.app_state.pass_error(spending_ring_parse_render_results); }
                self.ring_chart_state.update_is_ready(true);
                Task::none()
            }
            
            TransactionsPageSignal::MouseMovedInEarningRingChart(new_pos, layout_size) => {
                // checks if the ring parse is valid
                if self.ring_chart_state.earning_result().is_pass() {
                    // updates hovering
                    let update_hovering_result = self.ring_chart_state.earning_result_mut().wont_fail_ref_mut("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseMovedInEarningRingChart").update_hovering(new_pos, layout_size);
                    if update_hovering_result.is_fail() { self.app_state.pass_error(update_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseMovedInEarningRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseMovedInEarningRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::process_transaction_page_signal() - MouseMovedInEarningRingChart"));
                                    self.ring_chart_state.update_hovered_segment(None);
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            TransactionsPageSignal::MouseMovedInSpendingRingChart(new_pos, layout_size) => {
                // checks if the ring parse is valid
                if self.ring_chart_state.spending_result().is_pass() {
                    // updates hovering
                    let update_hovering_result = self.ring_chart_state.spending_result_mut().wont_fail_ref_mut("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseMovedInSpendingRingChart").update_hovering(new_pos, layout_size);
                    if update_hovering_result.is_fail() { self.app_state.pass_error(update_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseMovedInSpendingRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseMovedInSpendingRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::process_transaction_page_signal() - MouseMovedInSpendingRingChart"));
                                    self.ring_chart_state.update_hovered_segment(None);
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            TransactionsPageSignal::MouseExitedEarningRingChart => {
                // checks if the ring parse is valid
                if self.ring_chart_state.earning_result().is_pass() {
                    // updates hovering
                    let stop_hovering_result = self.ring_chart_state.earning_result_mut().wont_fail_ref_mut("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseExitedEarningRingChart").stop_hovering();
                    if stop_hovering_result.is_fail() { self.app_state.pass_error(stop_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseExitedEarningRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseExitedEarningRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::process_transaction_page_signal() - MouseExitedEarningRingChart"));
                                    self.ring_chart_state.update_hovered_segment(None);
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            TransactionsPageSignal::MouseExitedSpendingRingChart => {
                // checks if the ring parse is valid
                if self.ring_chart_state.spending_result().is_pass() {
                    // updates hovering
                    let stop_hovering_result = self.ring_chart_state.spending_result_mut().wont_fail_ref_mut("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseExitedSpendingRingChart").stop_hovering();
                    if stop_hovering_result.is_fail() { self.app_state.pass_error(stop_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseExitedSpendingRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::process_transaction_page_signal() - MouseExitedSpendingRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::process_transaction_page_signal() - MouseExitedSpendingRingChart"));
                                    self.ring_chart_state.update_hovered_segment(None);
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            TransactionsPageSignal::OpenTagRegistry => {
                self.app_state.update_page(Pages::TagRegistry);
                Task::none()
            }
        }
    }

    /// Processes signals related to adding a new `Transaction`.
    #[must_use]
    fn process_add_transaction_signal(&mut self, signal: AddTransactionSignal) -> Task<Signal> {
        match signal {
            AddTransactionSignal::AddTransaction => {
                let result = self.bank.add_transaction_from_raw_parts(
                    self.new_transaction_state.value_string(),
                    self.new_transaction_state.currency_string(),
                    self.new_transaction_state.date_picker_state().selected_date(),
                    self.new_transaction_state.description_content().text(),
                    self.new_transaction_state.tags(),
                );
            
                match result {
                    Pass(()) => {
                        self.app_state.update_page(Pages::Transactions);
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.refresh_currency_exchange_task(),
                            self.update_tag_registry_task(),
                            self.save_task(),
                            self.update_ring_parse_task(),
                            self.update_trend_parse_task(),
                            self.flag_finished_interaction_task(),
                        ])
                    }
                    Fail(_) => {
                        self.app_state.pass_error(result);
                        self.flag_finished_interaction_task()
                    }
                }
            }
        
            AddTransactionSignal::UpdateNewTransactionValueString(new_value_string) => {
                self.new_transaction_state.update_value_string(new_value_string);
                Task::none()
            }
    
            AddTransactionSignal::UpdateNewTransactionCurrencyString(new_currency_string) => {
                self.new_transaction_state.update_currency_string(new_currency_string);
                Task::none()
            }
    
            AddTransactionSignal::UpdateNewTransactionDatePickerMode(new_mode) => {
                self.new_transaction_state.date_picker_state_mut().update_mode(new_mode);
                Task::none()
            }
    
            AddTransactionSignal::AdvanceNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_state.date_picker_state().current_year() >= 9999 { return Task::none(); }
                let new_year = self.new_transaction_state.date_picker_state().current_year() + 1;
                self.new_transaction_state.date_picker_state_mut().update_current_year(new_year);
                Task::none()
            }
    
            AddTransactionSignal::RecedeNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_state.date_picker_state().current_year() <= 1000 { return Task::none(); }
                let new_year = self.new_transaction_state.date_picker_state().current_year() - 1;
                self.new_transaction_state.date_picker_state_mut().update_current_year(new_year);
                Task::none()
            }
    
            AddTransactionSignal::UpdateNewTransactionCurrentMonth(new_month) => {
                self.new_transaction_state.date_picker_state_mut().update_current_month(new_month);
                self.new_transaction_state.date_picker_state_mut().update_mode(DatePickerModes::ShowingDaysInMonth);
                Task::none()
            }
            
            AddTransactionSignal::UpdateNewTransactionSelectedDate(new_date_result) => {
                match new_date_result {
                    Pass(new_date) => {
                        self.new_transaction_state.date_picker_state_mut().update_selected_date(new_date);
                        self.new_transaction_state.date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                    }
                    Fail(_) => { self.app_state.pass_error(new_date_result); }
                }
                
                Task::none()
            }
            
            AddTransactionSignal::UpdateNewTransactionDescriptionContent(action) => {
                self.new_transaction_state.description_content_mut().perform(action);
                Task::none()
            }
    
            AddTransactionSignal::UpdateNewTransactionCurrentTagString(new_tag_string) => {
                self.new_transaction_state.update_current_tag_string(new_tag_string);
                Task::none()
            }
    
            AddTransactionSignal::AddNewTransactionTag(tag_string) => {
                let new_tag_result = Tag::new(&tag_string);
                
                match new_tag_result {
                    Pass(new_tag) => {
                        self.new_transaction_state.add_tag(new_tag);
                        self.new_transaction_state.update_current_tag_string(String::new());
                    }
                    Fail(_) => { self.app_state.pass_error(new_tag_result); }
                }
                
                Task::none()
            }
    
            AddTransactionSignal::RemoveNewTransactionTag(tag) => {
                let mut tags = self.new_transaction_state.tags();
                tags.retain(|t| *t != tag);
                self.new_transaction_state.update_tags(tags);
                Task::none()
            }
        }
    }

    /// Processes signals related to editing an existing `Transaction`.
    #[must_use]
    fn process_edit_transaction_signal(&mut self, signal: EditTransactionSignal) -> Task<Signal> {
        match signal {
            EditTransactionSignal::EditTransaction => {
                // ensures that the id was set
                let id_result = Schrod::from_option(self.edit_transaction_state.id(), "Transaction id was not set!", "App::process_edit_transaction_signal() - EditTransaction");
                // fails if it is not
                if id_result.is_fail() {
                    self.app_state.pass_error(id_result);
                    self.flag_finished_interaction_task()
                }
                // continues if it is
                else {
                    let id = id_result.wont_fail("This is past an is_fail() guard clause.", "App::process_edit_transaction_signal() - EditTransaction");
                    let result = self.bank.edit_transaction_with_raw_parts(
                        id,
                        self.edit_transaction_state.value_string(),
                        self.edit_transaction_state.currency_string(),
                        self.edit_transaction_state.date_picker_state().selected_date(),
                        self.edit_transaction_state.description_content().text(),
                        self.edit_transaction_state.tags(),
                    );
                    
                    match result {
                        Pass(()) => {
                            self.app_state.update_page(Pages::Transactions);
                            self.update_cash_flow_result();
                            Task::batch(vec![
                                self.refresh_currency_exchange_task(),
                                self.update_tag_registry_task(),
                                self.save_task(),
                                self.update_ring_parse_task(),
                                self.update_trend_parse_task(),
                                self.flag_finished_interaction_task(),
                            ])
                        }
                        Fail(_) => {
                            self.app_state.pass_error(result);
                            self.flag_finished_interaction_task()
                        }
                    }
                }
            }

            EditTransactionSignal::PrimeRemoveTransaction => {
                self.edit_transaction_state.update_is_delete_primed(true);
                Task::none()
            }

            EditTransactionSignal::UnprimeRemoveTransaction => {
                self.edit_transaction_state.update_is_delete_primed(false);
                Task::none()
            }

            EditTransactionSignal::RemoveTransaction => {
                // ensures that the id was set
                let id_result = Schrod::from_option(self.edit_transaction_state.id(), "Transaction id was not set!", "App::process_edit_transaction_signal() - RemoveTransaction");
                // fails if it is not
                if id_result.is_fail() {
                    self.edit_transaction_state.update_is_delete_primed(false);
                    self.app_state.pass_error(id_result);
                    self.flag_finished_interaction_task()
                }
                // continues if it is
                else {
                    let id = id_result.wont_fail("This is past an is_fail() guard clause.", "App::process_edit_transaction_signal() - RemoveTransaction");
                    let result = self.bank.remove_transaction(id);
                    
                    match result {
                        Pass(()) => {
                            self.edit_transaction_state.update_is_delete_primed(false);
                            self.app_state.update_page(Pages::Transactions);
                            self.update_cash_flow_result();
                            Task::batch(vec![
                                self.refresh_currency_exchange_task(),
                                self.update_tag_registry_task(),
                                self.save_task(),
                                self.update_ring_parse_task(),
                                self.update_trend_parse_task(),
                                self.flag_finished_interaction_task(),
                            ])
                        }
                        Fail(_) => {
                            self.app_state.pass_error(result);
                            self.flag_finished_interaction_task()
                        }
                    }
                }
            }
            
            EditTransactionSignal::UpdateEditTransactionValueString(new_value_string) => {
                self.edit_transaction_state.update_value_string(new_value_string);
                Task::none()
            }

            EditTransactionSignal::UpdateEditTransactionCurrencyString(new_currency_string) => {
                self.edit_transaction_state.update_currency_string(new_currency_string);
                Task::none()
            }

            EditTransactionSignal::UpdateEditTransactionDatePickerMode(new_mode) => {
                self.edit_transaction_state.date_picker_state_mut().update_mode(new_mode);
                Task::none()
            }

            EditTransactionSignal::AdvanceEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_state.date_picker_state().current_year() >= 9999 { return Task::none(); }
                let new_year = self.edit_transaction_state.date_picker_state().current_year() + 1;
                self.edit_transaction_state.date_picker_state_mut().update_current_year(new_year);
                Task::none()
            }

            EditTransactionSignal::RecedeEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_state.date_picker_state().current_year() <= 1000 { return Task::none() }
                let new_year = self.edit_transaction_state.date_picker_state().current_year() - 1;
                self.edit_transaction_state.date_picker_state_mut().update_current_year(new_year);
                Task::none()
            }

            EditTransactionSignal::UpdateEditTransactionCurrentMonth(new_month) => {
                self.edit_transaction_state.date_picker_state_mut().update_current_month(new_month);
                self.edit_transaction_state.date_picker_state_mut().update_mode(DatePickerModes::ShowingDaysInMonth);
                Task::none()
            }
            
            EditTransactionSignal::UpdateEditTransactionSelectedDate(edit_date_result) => {
                match edit_date_result {
                    Pass(new_date) => {
                        self.edit_transaction_state.date_picker_state_mut().update_selected_date(new_date);
                        self.edit_transaction_state.date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                    }
                    Fail(_) => { self.app_state.pass_error(edit_date_result); }
                }
                
                Task::none()
            }
            
            EditTransactionSignal::UpdateEditTransactionDescriptionContent(action) => {
                self.edit_transaction_state.description_content_mut().perform(action);
                Task::none()
            }

            EditTransactionSignal::UpdateEditTransactionCurrentTagString(new_tag_string) => {
                self.edit_transaction_state.update_current_tag_string(new_tag_string);
                Task::none()
            }

            EditTransactionSignal::AddEditTransactionTag(tag_string) => {
                let new_tag_result = Tag::new(&tag_string);
                
                match new_tag_result {
                    Pass(new_tag) => {
                        self.edit_transaction_state.add_tag(new_tag);
                        self.edit_transaction_state.update_current_tag_string(String::new());
                    }
                    Fail(_) => { self.app_state.pass_error(new_tag_result); }
                }
                
                Task::none()
            }
            
            EditTransactionSignal::RemoveEditTransactionTag(tag) => {
                let mut tags = self.edit_transaction_state.tags();
                tags.retain(|t| *t != tag);
                self.edit_transaction_state.update_tags(tags);
                Task::none()
            }
        }
    }

    /// Processes signals related to the `TagRegistry`.
    #[must_use]
    fn process_tag_registry_signal(&mut self, signal: TagRegistrySignal) -> Task<Signal> {
        match signal {
            TagRegistrySignal::ExpandTag(tag) => {
                self.tag_registry_slip_state_manager.expand(&tag);
                Task::none()
            }
            
            TagRegistrySignal::CollapseTag(tag) => { // todo remove if unused
                self.tag_registry_slip_state_manager.collapse(&tag);
                Task::none()
            }
            
            TagRegistrySignal::SetTagColor(tag, color) => {
                self.bank.tag_registry.set(&tag, color);
                self.tag_registry_slip_state_manager.collapse(&tag);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            TagRegistrySignal::ResetTag(tag) => {
                self.bank.tag_registry.remove(&tag);
                self.tag_registry_slip_state_manager.collapse(&tag);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        }
    }

    /// Processes signals related to the trends page.
    #[must_use]
    fn process_trends_signal(&mut self, signal: TrendsSignal) -> Task<Signal> {
        match signal {
            TrendsSignal::SetTrendingInterval(interval) => {
                self.trends_state.update_interval(interval);
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
            
            TrendsSignal::ToggleShowBalance => {
                self.trends_state.toggle_show_balance_line();
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            TrendsSignal::AddTrendingTag(tag) => {
                self.trends_state.add_tag(tag);
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            TrendsSignal::RemoveTrendingTag(tag) => {
                let mut tags = self.trends_state.tags();
                tags.retain(|t| *t != tag);
                self.trends_state.update_tags(tags);;
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            TrendsSignal::ExtendTrendingLength => {
                if self.trends_state.length() < 12 { self.trends_state.update_length(self.trends_state.length() + 1); }
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            TrendsSignal::ReduceTrendingLength => {
                if self.trends_state.length() > 1 { self.trends_state.update_length(self.trends_state.length() - 1); }
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
            
            TrendsSignal::StartedRenderingTrendParse => {
                self.trends_state.update_is_ready(false);
                Task::none()
            }
        
            TrendsSignal::FinishedRenderingTrendParse(new_trend_parse, render_results) => {
                self.trends_state.update_trend_parse_result(Pass(new_trend_parse));
                if render_results.is_fail() { self.app_state.pass_error(render_results); }
                self.trends_state.update_is_ready(true);
                Task::none()
            }
        
            TrendsSignal::FailedToRenderTrendParse => {
                self.trends_state.update_is_ready(true);
                Task::none()
            }
        }
    }

    /// Processes signals related to settings.
    #[must_use]
    fn process_settings_signal(&mut self, signal: SettingsSignal) -> Task<Signal> {
        match signal {
            SettingsSignal::ChangeTheme(theme) => {
                self.update_material_theme(theme);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            SettingsSignal::UpdateNewMainCurrencyString(currency_string) => {
                self.settings_state.update_new_main_currency_string(currency_string);
                Task::none()
            }

            SettingsSignal::SetMainCurrency => {
                if Transaction::can_parse_to_currency(self.settings_state.new_main_currency_string()) {
                    let set_result = self.bank.currency_exchange.set_main_currency(self.settings_state.new_main_currency_string());
                    self.settings_state.update_new_main_currency_string(String::new());
                    if set_result.is_fail() { self.app_state.pass_error(set_result); }
                    
                    self.update_cash_flow_result();
                    Task::batch(vec![
                        self.refresh_currency_exchange_task(),
                        self.save_task(),
                        self.update_ring_parse_task(),
                        self.update_trend_parse_task(),
                        self.flag_finished_interaction_task(),
                    ])
                }

                else { self.flag_finished_interaction_task() }
            }

            SettingsSignal::UpdateNewTimePriceString(time_price_string) => {
                self.settings_state.update_new_time_price_string(time_price_string);
                Task::none()
            }
            
            SettingsSignal::SetTimePrice => {
                if CurrencyExchange::is_time_price_string_valid(self.settings_state.new_time_price_string()) {
                    let set_result = self.bank.currency_exchange.set_time_price(self.settings_state.new_time_price_string());
                    self.settings_state.update_new_time_price_string(String::new());
                    if set_result.is_fail() { self.app_state.pass_error(set_result); }
                    
                    self.update_cash_flow_result();
                    Task::batch(vec![
                        self.refresh_currency_exchange_task(),
                        self.save_task(),
                        self.update_ring_parse_task(),
                        self.update_trend_parse_task(),
                        self.flag_finished_interaction_task(),
                    ])
                }

                else { self.flag_finished_interaction_task() }
            }

            SettingsSignal::SetFlowType(flow_type) => {
                self.bank.currency_exchange.set_flow_type(flow_type);
                
                self.update_cash_flow_result();
                Task::batch(vec![
                    self.refresh_currency_exchange_task(),
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            SettingsSignal::UpdateNewExchangeRateString(from_string, to_string, new_rate_string) => {
                let rate_result = self.bank.currency_exchange.get_mut(&from_string, &to_string);
                if rate_result.is_none() {
                    self.app_state.pass_error(Schrod::<()>::new_fail("Failed to get ExchangeRate to update new_rate_string!", "App::process_settings_signal() - UpdateNewExchangeRateString"));
                    return Task::none();
                }
                let rate = Schrod::from_option(rate_result, "Failed to get ExchangeRate!", "App::process_settings_signal() - UpdateNewExchangeRateString").wont_fail("This is past an option guard clause.", "App::process_settings_signal() - UpdateNewExchangeRateString");
                rate.new_rate_string = new_rate_string;
                Task::none()
            }

            SettingsSignal::TrySetNewExchangeRate(from_string, to_string, new_rate_string) => {
                let f64_result = Schrod::from_result(new_rate_string.parse::<f64>(), "Failed to convert new_rate_string to f64!", "App::process_settings_signal() - TrySetNewExchangeRate");
                if f64_result.is_fail() { return Task::none(); }
                let decimal_result = Schrod::from_option(Decimal::from_f64(f64_result.wont_fail("This is past an is_fail() guard clause.", "App::process_settings_signal() - TrySetNewExchangeRateString")), "Failed to convert f64 to Decimal!", "App::process_settings_signal() - TrySetNewExchangeRateString");
                if decimal_result.is_fail() { return Task::none(); }
                let rate = decimal_result.wont_fail("This is past an option guard clause.", "App::process_settings_signal() - TrySetNewExchangeRate");
                if rate <= Decimal::from(0) { return Task::none(); }

                let set_result = self.bank.currency_exchange.set(&from_string, &to_string, rate);
                if set_result.is_fail() { self.app_state.pass_error(set_result); }
                
                self.update_cash_flow_result();
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        }
    }

    /// Processes signals related to saving and loading.
    #[must_use]
    fn process_save_data_signal(&mut self, signal: SaveDataSignal) -> Task<Signal> {
        match signal {
            SaveDataSignal::FinishedSaving(save_result) => {
                match save_result {
                    Pass(()) => {
                        self.save_state.update_saved_successfully(true);
                    }
                    Fail(_) => {
                        self.save_state.update_saved_successfully(false);
                        self.app_state.pass_error(save_result);
                    }
                }
                
                Task::none()
            }
            
            SaveDataSignal::OpenImportFilePicker => {
                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("Import File")
                            .add_filter("JSON", &["json"])
                            .add_filter("All Files", &["*"])
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    |result| match result {
                        Some(path) => Signal::SaveDataSignal(SaveDataSignal::ImportFileSelected(path)),
                        None => Signal::GeneralSignal(GeneralSignal::InvalidAction("No file selected".to_string())),
                    },
                )
            }
            
            SaveDataSignal::ImportFileSelected(path) => {
                let import_data_result = load_from(&path);
                if let Pass(import_data) = import_data_result {
                    self.save_state.update_import_data(Some(import_data));
                    self.app_state.update_page(Pages::ConfirmImport);
                    Task::none()
                }
                else {
                    self.app_state.pass_error(import_data_result);
                    self.save_state.update_import_data(None);
                    Task::none()
                }
            }
            
            SaveDataSignal::ConfirmImport => {
                if let Some(import_data) = self.save_state.import_data() {
                    let transactions = import_data.transactions.clone();
                    let currency_exchange = import_data.currency_exchange.clone();
                    let tag_registry = import_data.tag_registry.clone();
                    let mut new_bank = Bank::default();
                    new_bank.init(transactions, currency_exchange, tag_registry);
                    self.bank = new_bank;
                    self.save_state.update_import_data(None);
                    self.app_state.update_page(Pages::Transactions);
                    
                    self.update_cash_flow_result();
                    Task::batch(vec![
                        self.refresh_currency_exchange_task(),
                        self.update_tag_registry_task(),
                        self.save_task(),
                        self.update_ring_parse_task(),
                        self.update_trend_parse_task(),
                        self.flag_finished_interaction_task(),
                    ])
                }
                
                else { self.flag_finished_interaction_task() }
            }
            
            SaveDataSignal::CancelImport => {
                self.save_state.update_import_data(None);
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            SaveDataSignal::OpenLegacyImportFilePicker => {
                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("Import File")
                            .add_filter("txrctr", &["txrctr"])
                            .add_filter("All Files", &["*"])
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    |result| match result {
                        Some(path) => Signal::SaveDataSignal(SaveDataSignal::LegacyImportFileSelected(path)),
                        None => Signal::GeneralSignal(GeneralSignal::InvalidAction("No file selected".to_string())),
                    },
                )
            }
            
            SaveDataSignal::LegacyImportFileSelected(path) => {
                let legacy_import_data_result = load_legacy_from(&path);
                if let Pass(import_data) = legacy_import_data_result {
                    self.save_state.update_legacy_import_data(Some(import_data));
                    self.app_state.update_page(Pages::ConfirmLegacyImport);
                    Task::none()
                }
                else {
                    self.app_state.pass_error(legacy_import_data_result);
                    self.save_state.update_legacy_import_data(None);
                    Task::none()
                }
            }
            
            SaveDataSignal::ConfirmLegacyImport => {
                if let Some(import_data) = self.save_state.legacy_import_data() {
                    let load_result = self.bank.load_transactions(import_data.clone());
                    if load_result.is_fail() { self.app_state.pass_error(load_result); }
                    let init_filter_dates_result = self.bank.init_filter_dates();
                    if init_filter_dates_result.is_fail() { self.app_state.pass_error(init_filter_dates_result); }
                    self.save_state.update_legacy_import_data(None);
                    self.app_state.update_page(Pages::Transactions);
                    
                    self.update_cash_flow_result();
                    Task::batch(vec![
                        self.refresh_currency_exchange_task(),
                        self.update_tag_registry_task(),
                        self.save_task(),
                        self.update_ring_parse_task(),
                        self.update_trend_parse_task(),
                        self.flag_finished_interaction_task(),
                    ])
                }
                
                else { self.flag_finished_interaction_task() }
            }
            
            SaveDataSignal::CancelLegacyImport => {
                self.save_state.update_legacy_import_data(None);
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            SaveDataSignal::Backup => {
                self.backup_task()
            }
            
            SaveDataSignal::FinishedBackingup(backup_results) => {
                if backup_results.is_fail() { self.app_state.pass_error(backup_results); }
                Task::none()
            }

            SaveDataSignal::OpenDataLocation => {
                let result = save_engine::open_data_location_file_explorer();
                if result.is_fail() { self.app_state.pass_error(result); }
                Task::none()
            }
        }
    }
    
    /// Updates the `App` based on a given `Signal`.
    #[must_use]
    pub fn update(&mut self, signal: Signal) -> Task<Signal> {
        // does not allow any changes if the app did not save or load successfully
        if !self.save_state.saved_successfully() || !self.save_state.loaded_successfully() {
            return Task::none();
        }
    
        // if the app loaded successfully, the app runs as normal
        match signal {
            Signal::KeybindSignal(signal) => self.process_keybind_signal(signal),
            Signal::GeneralSignal(signal) => self.process_general_signal(signal),
            Signal::FilterSignal(signal) => self.process_filter_signal(signal),
            Signal::TransactionsPageSignal(signal) => self.process_transactions_page_signal(signal),
            Signal::AddTransactionSignal(signal) => self.process_add_transaction_signal(signal),
            Signal::EditTransactionSignal(signal) => self.process_edit_transaction_signal(signal),
            Signal::TagRegistrySignal(signal) => self.process_tag_registry_signal(signal),
            Signal::TrendsSignal(signal) => self.process_trends_signal(signal),
            Signal::SettingsSignal(signal) => self.process_settings_signal(signal),
            Signal::SaveDataSignal(signal) => self.process_save_data_signal(signal),
        }
    }
    
    /// Manages keybind input.
    #[must_use]
    pub fn subscription(&self) -> Subscription<Signal> {
        event::listen_with(|event, status, _window| {
            if status == event::Status::Captured { return None }
            
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                    match key {
                        keyboard::Key::Named(Named::Tab) if modifiers.shift() => Some(Signal::KeybindSignal(KeybindSignal::FocusPrevious)),
                        keyboard::Key::Named(Named::Tab) => Some(Signal::KeybindSignal(KeybindSignal::FocusNext)),
                        
                        keyboard::Key::Character(c) => match c.as_str() {
                            "]" if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::AdvanceDayKeybind)),
                            "[" if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::RecedeDayKeybind)),
                            "'" if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::AdvanceMonthKeybind)),
                            ";" if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::RecedeMonthKeybind)),
                            "." if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::AdvanceYearKeybind)),
                            "," if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::RecedeYearKeybind)),
                            
                            "a" if modifiers.command() => Some(Signal::KeybindSignal(KeybindSignal::AddTransactionKeybind)),
                            
                            _ => None,
                        },
                        
                        _ => None,
                    }
                }
                _ => None,
            }
        })
    }

    /// Renders the `App`.
    #[must_use]
    pub fn view<'a>(&'a self) -> Element<'a, Signal> {
        // runs the normal pages system if there are no critical errors
        if self.app_state.critical_errors().is_empty() {
            // shows the helping page if the user is requesting help
            if self.app_state.is_helping() { help_page(self).into() }

            // displays the regular page system if the user does not want help
            else {
                match self.app_state.page() {
                    Pages::Transactions => { transactions_page(self).into() }
                    Pages::AddingTransaction => { add_transaction_page(self).into() }
                    Pages::EditingTransaction => { edit_transaction_page(self).into() }
                    Pages::Trends => { trends_page(self).into() }
                    Pages::TagRegistry => { tag_registry_page(self).into() }
                    Pages::Settings => { settings_page(self).into() }
                    Pages::ConfirmImport => { confirm_import_page(self).into() }
                    Pages::ConfirmLegacyImport => { confirm_legacy_import_page(self).into() }
                    Pages::WarningsPage => { warnings_page(self).into() }
                    Pages::MinorErrorsPage => { minor_errors_page(self).into() }
                }
            }
        }

        // if critical errors were found, an error log page is displayed
        else { critical_errors_page(self).into() }
    }



    // basic utilities
    /// Updates the material theme of the `App`.
    pub fn update_material_theme(&mut self, new_theme_selection: MaterialThemes) {
        self.app_state.update_material_theme(new_theme_selection);
        self.app_state.update_iced_theme(new_theme_selection.generate_iced_palette());
    }

    /// Returns a `Task` that backs up persistent data to the disk.
    #[must_use]
    fn flag_finished_interaction_task(&mut self) -> Task<Signal> {
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            sender.send(Signal::GeneralSignal(GeneralSignal::FinishedInteraction)).await.ok();
        }))
    }



    // data parsing utilities
    /// Updates the `cash_flow_result` for the `App`.
    fn update_cash_flow_result(&mut self) {
        let new_cash_flow_result = CashFlow::new(&self.bank, &self.bank.get_filtered_ids(Filters::Primary));
        if new_cash_flow_result.is_fail() { self.app_state.pass_error(new_cash_flow_result.clone()); }
        self.bank_state.update_cash_flow_result(new_cash_flow_result);
    }
    
    /// Updates the `ring_parse_result`s for the earning and spending rings.
    fn update_ring_parse_results(&mut self) {
        let new_earning_ring_parse_result = RingParse::new(self, &self.bank, Filters::Primary, FlowDirections::Earning);
        if new_earning_ring_parse_result.is_fail() { self.app_state.pass_error(new_earning_ring_parse_result.clone()); }
        self.ring_chart_state.update_earning_result(new_earning_ring_parse_result);
        
        let new_spending_ring_parse_result = RingParse::new(self, &self.bank, Filters::Primary, FlowDirections::Spending);
        if new_spending_ring_parse_result.is_fail() { self.app_state.pass_error(new_spending_ring_parse_result.clone()); }
        self.ring_chart_state.update_spending_result(new_spending_ring_parse_result);
    }

    /// Returns a `Task` that updates the `RingParse` results for the earning and spending rings.
    #[must_use]
    fn update_ring_parse_task(&mut self) -> Task<Signal> {
        self.update_ring_parse_results();
        
        let earning_ring_parse_result = self.ring_chart_state.earning_result().clone();
        let spending_ring_parse_result = self.ring_chart_state.spending_result().clone();
        let theme = self.app_state.material_theme();
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            sender.send(Signal::TransactionsPageSignal(TransactionsPageSignal::StartedRenderingRingCharts)).await.ok();
            
            let new_earning_ring_parse_result = match earning_ring_parse_result {
                Pass(earning_ring_parse) => RingParse::get_rendered(earning_ring_parse, theme).await,
                Fail(_) => (earning_ring_parse_result, Schrod::new_fail("Cannot rerender failed Ring Parse result!", "App::update_ring_parse_task()")),
            };
            
            let new_spending_ring_parse_result = match spending_ring_parse_result {
                Pass(spending_ring_parse) => RingParse::get_rendered(spending_ring_parse, theme).await,
                Fail(_) => (spending_ring_parse_result, Schrod::new_fail("Cannot rerender failed Ring Parse result!", "App::update_ring_parse_task()")),
            };
            
            sender.send(Signal::TransactionsPageSignal(TransactionsPageSignal::FinishedRenderingRingCharts(Box::new(new_earning_ring_parse_result), Box::new(new_spending_ring_parse_result)))).await.ok();
        }))
    }

    /// Updates the `trend_parse_result`.
    fn update_trend_parse_result(&mut self) {
        let transactions = self.bank.get_ledger();
        let new_trend_parse_result = TrendParse::new(
            &self.bank,
            transactions,
            self.trends_state.show_balance_line(),
            self.trends_state.tags(),
            self.trends_state.interval(),
            self.trends_state.last_trending_date(),
            self.trends_state.length(),
        );
        if new_trend_parse_result.is_fail() { self.app_state.pass_error(new_trend_parse_result.clone()); }
        self.trends_state.update_trend_parse_result(new_trend_parse_result);
    }
    
    /// Returns a `Task` that updates the `TrendParse` result.
    #[must_use]
    fn update_trend_parse_task(&mut self) -> Task<Signal> {
        self.update_trend_parse_result();

        if self.trends_state.trend_parse_result().is_fail() {
            Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
                sender.send(Signal::TrendsSignal(TrendsSignal::FailedToRenderTrendParse)).await.ok();
            }))
        }

        else {
            let mut trend_parse = self.trends_state.trend_parse_result().clone().wont_fail("This is past an is_fail() guard clause.", "App::update_trend_parse_task()");
            let tag_resistry_copy = self.bank.tag_registry.clone();
            let theme = self.app_state.material_theme();
            
            Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
                sender.send(Signal::TrendsSignal(TrendsSignal::StartedRenderingTrendParse)).await.ok();
            
                let render_result = trend_parse.render(&tag_resistry_copy, theme);
                sender.send(Signal::TrendsSignal(TrendsSignal::FinishedRenderingTrendParse(trend_parse, render_result))).await.ok();
            }))
        }
    }
    
    /// Returns a `Task` that refreshes the `ExchangeRates` in the `CurrencyExchange` based
    /// on their ages and all the `Currency`s used by the `Bank`.
    #[must_use]
    fn refresh_currency_exchange_task(&mut self) -> Task<Signal> {
        let mut currency_exchange = self.bank.currency_exchange.clone();
        let ledger_copy = self.bank.get_ledger_copy();
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let refresh_result = currency_exchange.refresh(ledger_copy).await;
            sender.send(Signal::GeneralSignal(GeneralSignal::FinishedUpdatingCurrencyExchange(currency_exchange, refresh_result))).await.ok();
        }))
    }
    
    /// Returns a `Task` that updates the `TagRegistry` based on the current `Tag`s in the `Bank`.
    #[must_use]
    fn update_tag_registry_task(&mut self) -> Task<Signal> {
        let old_tag_registry = self.bank.tag_registry.clone();
        let tags = self.bank.get_tags();
        let verify_filtered_tags_result = self.bank.verify_filtered_tags();
        if verify_filtered_tags_result.is_fail() { self.app_state.pass_error(verify_filtered_tags_result); }
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let updated_tag_registry = Bank::get_updated_tag_registry(old_tag_registry, tags);
            sender.send(Signal::GeneralSignal(GeneralSignal::FinishedUpdatingTagRegistry(updated_tag_registry))).await.ok();
        }))
    }



    // save data utilities
    /// Returns a `Task` that saves persistent data to the disk.
    #[must_use]
    fn save_task(&mut self) -> Task<Signal> {
        let save_data = SaveData {
            theme: self.app_state.material_theme(),
            transactions: self.bank.get_ledger_copy(),
            currency_exchange: self.bank.currency_exchange.clone(),
            tag_registry: self.bank.tag_registry.clone(),
            bit_wallets: self.bit_bank.wallets.clone(),
        };
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let save_result = save(save_data).await;
            
            sender.send(Signal::SaveDataSignal(SaveDataSignal::FinishedSaving(save_result))).await.ok();
        }))
    }
    
    /// Returns a `Task` that backs up persistent data to the disk.
    #[must_use]
    fn backup_task(&mut self) -> Task<Signal> {
        let save_data = SaveData {
            theme: self.app_state.material_theme(),
            transactions: self.bank.get_ledger_copy(),
            currency_exchange: self.bank.currency_exchange.clone(),
            tag_registry: self.bank.tag_registry.clone(),
            bit_wallets: self.bit_bank.wallets.clone(),
        };
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let backup_result = backup(save_data).await;
            
            sender.send(Signal::SaveDataSignal(SaveDataSignal::FinishedBackingup(backup_result))).await.ok();
        }))
    }
}