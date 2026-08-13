use std::borrow::Cow;

use iced::keyboard::key::Named;
use iced::widget::operation::{focus_next, focus_previous};
use iced::{Element, Event, Subscription, Task, Theme, event, keyboard};
use iced::widget::text_editor::Content;
use materialui::materials::MaterialThemes;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::bit_vault::bit_bank::BitBank;
use crate::container::signal::Signal;
use crate::container::state::{AppState, BankState, FilterState, RingChartsState, SaveState, SettingsState, TransactionState, TrendsState, TagRegistrationSlipStateManager};
use crate::container::warnings::Warnings;
use crate::pages::confirm_import_page::confirm_import_page;
use crate::pages::confirm_legacy_import_page::confirm_legacy_import_page;
use crate::pages::help_page::{help_button, help_page};
use crate::pages::minor_errors_page::minor_errors_page;
use crate::pages::settings_page::settings_page;
use crate::pages::transaction_management_pages::{add_transaction_page, edit_transaction_page};
use crate::pages::transactions_page::transactions_page;
use crate::pages::tag_registry_page::tag_registry_page;
use crate::pages::critical_errors_page::critical_errors_page;
use crate::pages::trends_page::trends_page;
use crate::pages::warnings_page::warnings_page;
use materialui::components::{DatePickerModes, PageProvider, ThemeProvider, page_pointer};
use crate::vault::bank::{Bank, CurrencyExchange, Filters, TagRegistry};
use crate::vault::parse::CashFlow;
use crate::vault::ring_parse::{FlowDirections, RingParse, Segment};
use crate::vault::save_engine::legacy::load_legacy_from;
use crate::vault::transaction::{Date, Id, Months, Tag, Transaction/*, ValueDisplayFormats*/};
use schrod::Schrod;
use schrod::Schrod::{Pass, Fail};
use crate::vault::trend_parse::{Intervals, TrendParse};
use iced::futures::SinkExt;
use iced::futures::channel::mpsc::Sender;
use crate::vault::save_engine::{self, SaveData, backup, load, load_from, save};

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
            .map(|page| page_pointer(app, &page.name(), &page.icon_name(), app.app_state.get_page() == page, Signal::ChangePageTo(page), true))
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
    fn page_name(&self) -> String { self.app_state.get_page().name().to_string() }
    fn page_icon(&self) -> String { self.app_state.get_page().icon_name() }
}
impl ThemeProvider for App {
    fn material_theme(&self) -> MaterialThemes { self.app_state.get_material_theme() }
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
        (app, Task::done(Signal::Launch))
    }


    
    // getters
    /// The tile of the `App`.
    #[must_use]
    pub fn title(&self) -> String { "Ascent".to_string() }
    
    /// Gets the current `Iced` `Theme`.
    #[must_use]
    pub fn iced_theme(&self) -> Theme { self.app_state.get_iced_theme().clone() }

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
    /// Updates the `App` based on a given `Signal`.
    #[allow(clippy::too_many_lines)] // This is going to be large since it is the central signal handler.
    #[must_use]
    pub fn update(&mut self, signal: Signal) -> Task<Signal> {
        // does not allow any changes if the app did not save or load successfully
        if !self.save_state.saved_successfully() || !self.save_state.loaded_successfully() {
            return Task::none();
        }
    
        // if the app loaded successfully, the app runs as normal
        match signal {
            // keybind
            Signal::FocusNext => { focus_next() }
            
            Signal::FocusPrevious => { focus_previous() }
            
            Signal::AddTransactionKeybind => {
                match self.app_state.get_page() {
                    Pages::Transactions => { Task::done(Signal::StartAddingTransaction) }
                    Pages::AddingTransaction => {
                        if Transaction::are_raw_parts_valid(
                            &self.new_transaction_state.get_value_string(),
                            &self.new_transaction_state.get_currency_string(),
                            &self.new_transaction_state.get_description_content().text(),
                            &self.new_transaction_state.get_tags()) {
                            Task::done(Signal::AddTransaction)
                        }
                        else { Task::none() }
                    }
                    Pages::EditingTransaction => {
                        if Transaction::are_raw_parts_valid(
                            &self.edit_transaction_state.get_value_string(),
                            &self.edit_transaction_state.get_currency_string(),
                            &self.edit_transaction_state.get_description_content().text(),
                            &self.edit_transaction_state.get_tags()) {
                            Task::done(Signal::EditTransaction)
                        }
                        else { Task::none() }
                    }
                    _ => { Task::none() }
                }
            }
            
            Signal::AdvanceYearKeybind => {
                match self.app_state.get_page() {
                    Pages::Transactions => {
                        if let Some(current_year) = self.bank.get_filter(Filters::Primary).get_filter_year() {
                            Task::done(Signal::SetFilterYear(Date::get_advanced_year(current_year), Filters::Primary))
                        }
                        else { Task::done(Signal::SetFilterYear(self.bank.get_latest_date_for_filter(Filters::Primary).get_year(), Filters::Primary)) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.advance_by_year();
                        Task::done(Signal::UpdateNewTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.advance_by_year();
                        Task::done(Signal::UpdateEditTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            Signal::RecedeYearKeybind => {
                match self.app_state.get_page() {
                    Pages::Transactions => {
                        if let Some(current_year) = self.bank.get_filter(Filters::Primary).get_filter_year() {
                            Task::done(Signal::SetFilterYear(Date::get_receded_year(current_year), Filters::Primary))
                        }
                        else { Task::done(Signal::SetFilterYear(self.bank.get_latest_date_for_filter(Filters::Primary).get_year(), Filters::Primary)) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.recede_by_year();
                        Task::done(Signal::UpdateNewTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.recede_by_year();
                        Task::done(Signal::UpdateEditTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            Signal::AdvanceMonthKeybind => {
                match self.app_state.get_page() {
                    Pages::Transactions => {
                        if let Some(current_month) = self.bank.get_filter(Filters::Primary).get_filter_month() {
                            Task::done(Signal::SetFilterMonth(current_month.get_next(), Filters::Primary))
                        }
                        else { Task::done(Signal::SetFilterMonth(self.bank.get_latest_date_for_filter(Filters::Primary).get_month(), Filters::Primary)) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.advance_by_month();
                        Task::done(Signal::UpdateNewTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.advance_by_month();
                        Task::done(Signal::UpdateEditTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            Signal::RecedeMonthKeybind => {
                match self.app_state.get_page() {
                    Pages::Transactions => {
                        if let Some(current_month) = self.bank.get_filter(Filters::Primary).get_filter_month() {
                            Task::done(Signal::SetFilterMonth(current_month.get_previous(), Filters::Primary))
                        }
                        else { Task::done(Signal::SetFilterMonth(self.bank.get_latest_date_for_filter(Filters::Primary).get_month(), Filters::Primary)) }
                    }
                    
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.recede_by_month();
                        Task::done(Signal::UpdateNewTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.recede_by_month();
                        Task::done(Signal::UpdateEditTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            Signal::AdvanceDayKeybind => {
                match self.app_state.get_page() {
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.advance_by_day();
                        Task::done(Signal::UpdateNewTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.advance_by_day();
                        Task::done(Signal::UpdateEditTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            Signal::RecedeDayKeybind => {
                match self.app_state.get_page() {
                    Pages::AddingTransaction => {
                        let mut new_date = self.new_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.recede_by_day();
                        Task::done(Signal::UpdateNewTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    Pages::EditingTransaction => {
                        let mut new_date = self.edit_transaction_state.get_date_picker_state().get_selected_date();
                        new_date.recede_by_day();
                        Task::done(Signal::UpdateEditTransactionSelectedDate(Pass(new_date)))
                    }
                    
                    _ => { Task::none() }
                }
            }
            
            
        
            // general signals
            Signal::Launch => {
                Task::batch(vec![
                    self.refresh_currency_exchange_task(),
                    self.update_tag_registry_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            Signal::FinishedInteraction => {
                self.app_state.finished_interaction(&self.bank);
                Task::none()
            }
            
            Signal::FinishedUpdatingCurrencyExchange(updated_currency_exchange, refresh_result) => {
                self.bank.currency_exchange = updated_currency_exchange;
                if refresh_result.is_fail() { self.app_state.pass_error(refresh_result); }
                Task::none()
            }
            
            Signal::FinishedUpdatingTagRegistry(updated_tag_registry) => {
                self.bank.tag_registry = updated_tag_registry;
                let tags = self.bank.get_tags();
                self.tag_registry_slip_state_manager = TagRegistrationSlipStateManager::new(tags);
                Task::none()
            }
            
            Signal::InvalidAction(_) => {
                eprintln!("Invalid action!");
                Task::none()
            }
            
            Signal::DismissCriticalErrors => {
                self.app_state.clear_critical_errors();
                Task::none()
            }
            
            Signal::DismissMinorErrors => {
                self.app_state.clear_minor_errors();
                self.app_state.update_page(Pages::WarningsPage);
                Task::none()
            }
            
            Signal::DismissWarnings => {
                self.app_state.clear_minor_errors();
                self.app_state.clear_warnings();
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            Signal::ChangePageTo(page) => {
                self.app_state.update_page(page);
                if page == Pages::Settings { self.refresh_currency_exchange_task() }
                else { Task::none() }
            }

            Signal::GoHome => {
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            Signal::HelpMe => {
                self.app_state.update_is_helping(true);
                Task::none()
            }
            
            Signal::DontHelpMe => {
                self.app_state.update_is_helping(false);
                Task::none()
            }
            
            
            
            // filtering
            Signal::SetFilterYear(year, filter) => {
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
            
            Signal::ClearFilterYear(filter) => {
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
            
            Signal::SetFilterMonth(month, filter) => {
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
            
            Signal::ClearFilterMonth(filter) => {
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
            
            Signal::AddFilterTag(tag, filter) => {
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
        
            Signal::RemoveFilterTag(tag, filter) => {
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
            
            Signal::ClearFilterTags(filter) => {
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
            
            Signal::UpdatePrimaryFilterCurrentSearchTermString(term) => {
                self.filter_state.update_primary_filter_current_search_term_string(term);
                Task::none()
            }
            
            Signal::UpdateDeepDive1FilterCurrentSearchTermString(term) => {
                self.filter_state.update_deep_dive_1_filter_current_search_term_string(term);
                Task::none()
            }
            
            Signal::UpdateDeepDive2FilterCurrentSearchTermString(term) => {
                self.filter_state.update_deep_dive_2_filter_current_search_term_string(term);
                Task::none()
            }
            
            Signal::AddFilterSearchTerm(filter) => {
                let term = match filter {
                    Filters::Primary => self.filter_state.get_primary_filter_current_search_term_string(),
                    Filters::DeepDive1 => self.filter_state.get_deep_dive_1_filter_current_search_term_string(),
                    Filters::DeepDive2 => self.filter_state.get_deep_dive_2_filter_current_search_term_string(),
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
            
            Signal::RemoveFilterSearchTerm(term, filter) => {
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
            
            Signal::ClearFilterSearchTerms(filter) => {
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
            
            Signal::ToggleFilterMode(filter) => {
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



            // transactions page signals
            Signal::StartAddingTransaction => {
                let current_date = self.bank.get_latest_date_for_filter(Filters::Primary);
                
                self.new_transaction_state.update_value_string(String::new());
                self.new_transaction_state.update_currency_string(String::new());
                self.new_transaction_state.get_date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                self.new_transaction_state.get_date_picker_state_mut().update_current_year(current_date.get_year());
                self.new_transaction_state.get_date_picker_state_mut().update_current_month(current_date.get_month());
                self.new_transaction_state.get_date_picker_state_mut().update_selected_date(current_date);
                self.new_transaction_state.update_description_content(Content::with_text(""));
                self.new_transaction_state.update_current_tag_string(String::new());
                self.new_transaction_state.update_tags(Vec::new());
                self.app_state.update_page(Pages::AddingTransaction);
                
                Task::none()
            }

            Signal::StartEditingTransaction(id_result) => {
                if id_result.is_fail() {
                    self.app_state.pass_error(id_result);
                    return Task::none();
                }
                let id = id_result.wont_fail("This is past an is_fail() guard clause.", "App::update() - StartEditingTransaction");
                let transaction_result = self.bank.get(id);

                if let Pass(transaction) = transaction_result {
                    self.edit_transaction_state.update_id(Some(id));
                    self.edit_transaction_state.update_value_string(transaction.value.amount().to_string());
                    self.edit_transaction_state.update_currency_string(transaction.value.currency().to_string());
                    self.edit_transaction_state.get_date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                    self.edit_transaction_state.get_date_picker_state_mut().update_current_year(transaction.date.get_year());
                    self.edit_transaction_state.get_date_picker_state_mut().update_current_month(transaction.date.get_month());
                    self.edit_transaction_state.get_date_picker_state_mut().update_selected_date(transaction.date);
                    self.edit_transaction_state.update_description_content(Content::with_text(&transaction.description));
                    self.edit_transaction_state.update_current_tag_string(String::new());
                    self.edit_transaction_state.update_tags(transaction.tags.clone());
                    self.edit_transaction_state.update_is_delete_primed(false);
                    self.app_state.update_page(Pages::EditingTransaction);
                }

                else { self.app_state.pass_error(transaction_result.convert::<String>("App::update() - StartEditingTransaction")); }
                
                Task::none()
            }
            
            Signal::MouseMovedInEarningRingChart(new_pos, layout_size) => {
                // checks if the ring parse is valid
                if self.ring_chart_state.earning_result().is_pass() {
                    // updates hovering
                    let update_hovering_result = self.ring_chart_state.earning_result().wont_fail_ref_mut("This is inside an is_pass() block.", "App::update() - MouseMovedInEarningRingChart").update_hovering(new_pos, layout_size);
                    if update_hovering_result.is_fail() { self.app_state.pass_error(update_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseMovedInEarningRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseMovedInEarningRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.ring_chart_state.update_hovered_segment(None);
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::update() - MouseMovedInEarningRingChart"));
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            Signal::MouseMovedInSpendingRingChart(new_pos, layout_size) => {
                // checks if the ring parse is valid
                if self.ring_chart_state.spending_result().is_pass() {
                    // updates hovering
                    let update_hovering_result = self.ring_chart_state.spending_result().wont_fail_ref_mut("This is inside an is_pass() block.", "App::update() - MouseMovedInSpendingRingChart").update_hovering(new_pos, layout_size);
                    if update_hovering_result.is_fail() { self.app_state.pass_error(update_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseMovedInSpendingRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseMovedInSpendingRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.ring_chart_state.update_hovered_segment(None);
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::update() - MouseMovedInSpendingRingChart"));
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            Signal::MouseExitedEarningRingChart => {
                // checks if the ring parse is valid
                if self.ring_chart_state.earning_result().is_pass() {
                    // updates hovering
                    let stop_hovering_result = self.ring_chart_state.earning_result().wont_fail_ref_mut("This is inside an is_pass() block.", "App::update() - MouseExitedEarningRingChart").stop_hovering();
                    if stop_hovering_result.is_fail() { self.app_state.pass_error(stop_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseExitedEarningRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.earning_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseExitedEarningRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.ring_chart_state.update_hovered_segment(None);
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::update() - MouseExitedEarningRingChart"));
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            Signal::MouseExitedSpendingRingChart => {
                // checks if the ring parse is valid
                if self.ring_chart_state.spending_result().is_pass() {
                    // updates hovering
                    let stop_hovering_result = self.ring_chart_state.spending_result().wont_fail_ref_mut("This is inside an is_pass() block.", "App::update() - MouseExitedSpendingRingChart").stop_hovering();
                    if stop_hovering_result.is_fail() { self.app_state.pass_error(stop_hovering_result); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseExitedSpendingRingChart").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.ring_chart_state.spending_result().wont_fail_ref("This is inside an is_pass() block.", "App::update() - MouseExitedSpendingRingChart").get_segment(&tag);
                            match hovered_segment_result {
                                Schrod::Pass(segment) => {
                                    self.ring_chart_state.update_hovered_segment(Some(segment.clone()));
                                }
                                Schrod::Fail(_) => {
                                    self.ring_chart_state.update_hovered_segment(None);
                                    self.app_state.pass_error(hovered_segment_result.convert::<String>("App::update() - MouseExitedSpendingRingChart"));
                                }
                            }
                        }
                        None => { self.ring_chart_state.update_hovered_segment(None); }
                    }
                }
                
                Task::none()
            }
            
            Signal::OpenTagRegistry => {
                self.app_state.update_page(Pages::TagRegistry);
                Task::none()
            }
            
            Signal::StartedRenderingRingCharts => {
                self.ring_chart_state.update_is_ready(false);
                Task::none()
            }
            
            Signal::FinishedRenderingRingCharts(rendered_earning_ring_parse_result, rendered_spending_ring_parse_result) => {
                let (earning_ring_parse_result, earning_ring_parse_render_results) = *rendered_earning_ring_parse_result;
                let (spending_ring_parse_result, spending_ring_parse_render_results) = *rendered_spending_ring_parse_result;
                self.ring_chart_state.update_earning_result(earning_ring_parse_result);
                self.ring_chart_state.update_spending_result(spending_ring_parse_result);
                if earning_ring_parse_render_results.is_fail() { self.app_state.pass_error(earning_ring_parse_render_results); }
                if spending_ring_parse_render_results.is_fail() { self.app_state.pass_error(spending_ring_parse_render_results); }
                self.ring_chart_state.update_is_ready(true);
                Task::none()
            }
            
            

            // adding transaction page signals
            Signal::AddTransaction => {
                let result = self.bank.add_transaction_from_raw_parts(
                    self.new_transaction_state.get_value_string(),
                    self.new_transaction_state.get_currency_string(),
                    self.new_transaction_state.get_date_picker_state().get_selected_date(),
                    self.new_transaction_state.get_description_content().text(),
                    self.new_transaction_state.get_tags(),
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
            
            Signal::UpdateNewTransactionValueString(new_value_string) => {
                self.new_transaction_state.update_value_string(new_value_string);
                Task::none()
            }

            Signal::UpdateNewTransactionCurrencyString(new_currency_string) => {
                self.new_transaction_state.update_currency_string(new_currency_string);
                Task::none()
            }

            Signal::UpdateNewTransactionDatePickerMode(new_mode) => {
                self.new_transaction_state.get_date_picker_state_mut().update_mode(new_mode);
                Task::none()
            }

            Signal::AdvanceNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_state.get_date_picker_state().get_current_year() >= 9999 { return Task::none(); }
                self.new_transaction_state.get_date_picker_state_mut().update_current_year(self.new_transaction_state.get_date_picker_state().get_current_year() + 1);
                Task::none()
            }

            Signal::RecedeNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_state.get_date_picker_state().get_current_year() <= 1000 { return Task::none(); }
                self.new_transaction_state.get_date_picker_state_mut().update_current_year(self.new_transaction_state.get_date_picker_state().get_current_year() - 1);
                Task::none()
            }

            Signal::UpdateNewTransactionCurrentMonth(new_month) => {
                self.new_transaction_state.get_date_picker_state_mut().update_current_month(new_month);
                self.new_transaction_state.get_date_picker_state_mut().update_mode(DatePickerModes::ShowingDaysInMonth);
                Task::none()
            }
            
            Signal::UpdateNewTransactionSelectedDate(new_date_result) => {
                match new_date_result {
                    Pass(new_date) => {
                        self.new_transaction_state.get_date_picker_state_mut().update_selected_date(new_date);
                        self.new_transaction_state.get_date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                    }
                    Fail(_) => { self.app_state.pass_error(new_date_result); }
                }
                
                Task::none()
            }
            
            Signal::UpdateNewTransactionDescriptionContent(action) => {
                self.new_transaction_state.get_description_content_mut().perform(action);
                Task::none()
            }

            Signal::UpdateNewTransactionCurrentTagString(new_tag_string) => {
                self.new_transaction_state.update_current_tag_string(new_tag_string);
                Task::none()
            }

            Signal::AddNewTransactionTag(tag_string) => {
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

            Signal::RemoveNewTransactionTag(tag) => {
                let mut tags = self.new_transaction_state.get_tags();
                tags.retain(|t| *t != tag);
                self.new_transaction_state.update_tags(tags);
                Task::none()
            }



            // editing transaction page signals
            Signal::EditTransaction => {
                // ensures that the id was set
                let id_result = Schrod::from_option(self.edit_transaction_state.get_id(), "Transaction id was not set!", "App::update() - EditTransaction");
                // fails if it is not
                if id_result.is_fail() {
                    self.app_state.pass_error(id_result);
                    self.flag_finished_interaction_task()
                }
                // continues if it is
                else {
                    let id = id_result.wont_fail("This is past an is_fail() guard clause.", "App::update() - EditTransaction");
                    let result = self.bank.edit_transaction_with_raw_parts(
                        id,
                        self.edit_transaction_state.get_value_string(),
                        self.edit_transaction_state.get_currency_string(),
                        self.edit_transaction_state.get_date_picker_state().get_selected_date(),
                        self.edit_transaction_state.get_description_content().text(),
                        self.edit_transaction_state.get_tags(),
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

            Signal::PrimeRemoveTransaction => {
                self.edit_transaction_state.update_is_delete_primed(true);
                Task::none()
            }

            Signal::UnprimeRemoveTransaction => {
                self.edit_transaction_state.update_is_delete_primed(false);
                Task::none()
            }

            Signal::RemoveTransaction => {
                // ensures that the id was set
                let id_result = Schrod::from_option(self.edit_transaction_state.get_id(), "Transaction id was not set!", "App::update() - RemoveTransaction");
                // fails if it is not
                if id_result.is_fail() {
                    self.edit_transaction_state.update_is_delete_primed(false);
                    self.app_state.pass_error(id_result);
                    self.flag_finished_interaction_task()
                }
                // continues if it is
                else {
                    let id = id_result.wont_fail("This is past an is_fail() guard clause.", "App::update() - RemoveTransaction");
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
            
            Signal::UpdateEditTransactionValueString(new_value_string) => {
                self.edit_transaction_state.update_value_string(new_value_string);
                Task::none()
            }

            Signal::UpdateEditTransactionCurrencyString(new_currency_string) => {
                self.edit_transaction_state.update_currency_string(new_currency_string);
                Task::none()
            }

            Signal::UpdateEditTransactionDatePickerMode(new_mode) => {
                self.edit_transaction_state.get_date_picker_state_mut().update_mode(new_mode);
                Task::none()
            }

            Signal::AdvanceEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_state.get_date_picker_state().get_current_year() >= 9999 { return Task::none(); }
                self.edit_transaction_state.get_date_picker_state_mut().update_current_year(self.edit_transaction_state.get_date_picker_state().get_current_year() + 1);
                Task::none()
            }

            Signal::RecedeEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_state.get_date_picker_state().get_current_year() <= 1000 { return Task::none() }
                self.edit_transaction_state.get_date_picker_state_mut().update_current_year(self.edit_transaction_state.get_date_picker_state().get_current_year() - 1);
                Task::none()
            }

            Signal::UpdateEditTransactionCurrentMonth(new_month) => {
                self.edit_transaction_state.get_date_picker_state_mut().update_current_month(new_month);
                self.edit_transaction_state.get_date_picker_state_mut().update_mode(DatePickerModes::ShowingDaysInMonth);
                Task::none()
            }
            
            Signal::UpdateEditTransactionSelectedDate(edit_date_result) => {
                match edit_date_result {
                    Pass(new_date) => {
                        self.edit_transaction_state.get_date_picker_state_mut().update_selected_date(new_date);
                        self.edit_transaction_state.get_date_picker_state_mut().update_mode(DatePickerModes::Hidden);
                    }
                    Fail(_) => { self.app_state.pass_error(edit_date_result); }
                }
                
                Task::none()
            }
            
            Signal::UpdateEditTransactionDescriptionContent(action) => {
                self.edit_transaction_state.get_description_content_mut().perform(action);
                Task::none()
            }

            Signal::UpdateEditTransactionCurrentTagString(new_tag_string) => {
                self.edit_transaction_state.update_current_tag_string(new_tag_string);
                Task::none()
            }

            Signal::AddEditTransactionTag(tag_string) => {
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
            
            Signal::RemoveEditTransactionTag(tag) => {
                let mut tags = self.edit_transaction_state.get_tags();
                tags.retain(|t| *t != tag);
                self.edit_transaction_state.update_tags(tags);
                Task::none()
            }
            
            
            
            // tag registry page signals
            Signal::ExpandTag(tag) => {
                self.tag_registry_slip_state_manager.expand(&tag);
                Task::none()
            }
            
            Signal::CollapseTag(tag) => { // todo remove if unused
                self.tag_registry_slip_state_manager.collapse(&tag);
                Task::none()
            }
            
            Signal::SetTagColor(tag, color) => {
                self.bank.tag_registry.set(&tag, color);
                self.tag_registry_slip_state_manager.collapse(&tag);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            Signal::ResetTag(tag) => {
                self.bank.tag_registry.remove(&tag);
                self.tag_registry_slip_state_manager.collapse(&tag);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            Signal::SetTrendingInterval(interval) => {
                self.trends_state.update_interval(interval);
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
            
            Signal::ToggleShowBalance => {
                self.trends_state.toggle_show_balance_line();
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            Signal::AddTrendingTag(tag) => {
                self.trends_state.add_tag(tag);
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            Signal::RemoveTrendingTag(tag) => {
                let mut tags = self.trends_state.tags();
                tags.retain(|t| *t != tag);
                self.trends_state.update_tags(tags);;
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            Signal::ExtendTrendingLength => {
                if self.trends_state.length() < 12 { self.trends_state.update_length(self.trends_state.length() + 1); }
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
        
            Signal::ReduceTrendingLength => {
                if self.trends_state.length() > 1 { self.trends_state.update_length(self.trends_state.length() - 1); }
                Task::batch(vec![
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }
            
            Signal::StartedRenderingTrendParse => {
                self.trends_state.update_is_ready(false);
                Task::none()
            }
        
            Signal::FinishedRenderingTrendParse(new_trend_parse, render_results) => {
                self.trends_state.update_trend_parse_result(Pass(new_trend_parse));
                if render_results.is_fail() { self.app_state.pass_error(render_results); }
                self.trends_state.update_is_ready(true);
                Task::none()
            }
        
            Signal::FailedToRenderTrendParse => {
                self.trends_state.update_is_ready(true);
                Task::none()
            }
            
            
            
            // settings page signals
            Signal::ChangeTheme(theme) => {
                self.update_material_theme(theme);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                    self.update_trend_parse_task(),
                    self.flag_finished_interaction_task(),
                ])
            }

            Signal::UpdateNewMainCurrencyString(currency_string) => {
                self.settings_state.update_new_main_currency_string(currency_string);
                Task::none()
            }

            Signal::SetMainCurrency => {
                if Transaction::can_parse_to_currency(self.settings_state.get_new_main_currency_string()) {
                    let set_result = self.bank.currency_exchange.set_main_currency(self.settings_state.get_new_main_currency_string());
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

            Signal::UpdateNewTimePriceString(time_price_string) => {
                self.settings_state.update_new_time_price_string(time_price_string);
                Task::none()
            }
            
            Signal::SetTimePrice => {
                if CurrencyExchange::is_time_price_string_valid(self.settings_state.get_new_time_price_string()) {
                    let set_result = self.bank.currency_exchange.set_time_price(self.settings_state.get_new_time_price_string());
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

            Signal::SetFlowType(flow_type) => {
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

            Signal::UpdateNewExchangeRateString(from_string, to_string, new_rate_string) => {
                let rate_result = self.bank.currency_exchange.get_mut(&from_string, &to_string);
                if rate_result.is_none() {
                    self.app_state.pass_error(Schrod::<()>::new_fail("Failed to get ExchangeRate to update new_rate_string!", "App::update() - UpdateNewExchangeRateString"));
                    return Task::none();
                }
                let rate = Schrod::from_option(rate_result, "Failed to get ExchangeRate!", "App::update() - UpdateNewExchangeRate").wont_fail("This is past an option guard clause.", "App::update() - UpdateNewExchangeRate");
                rate.new_rate_string = new_rate_string;
                Task::none()
            }

            Signal::TrySetNewExchangeRate(from_string, to_string, new_rate_string) => {
                let f64_result = Schrod::from_result(new_rate_string.parse::<f64>(), "Failed to convert new_rate_string to f64!", "App::update() - TrySetNewExchangeRate");
                if f64_result.is_fail() { return Task::none(); }
                let decimal_result = Schrod::from_option(Decimal::from_f64(f64_result.wont_fail("This is past an is_fail() guard clause.", "App::update() - TrySetNewExchangeRate")), "Failed to convert f64 to Decimal!", "App::update() - TrySetNewExchangeRate");
                if decimal_result.is_fail() { return Task::none(); }
                let rate = decimal_result.wont_fail("This is past an option guard clause.", "App::update() - TrySetNewExchangeRate");
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
            
            
            
            // saving and loading signals
            Signal::FinishedSaving(save_result) => {
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
            
            Signal::OpenImportFilePicker => {
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
                        Some(path) => Signal::ImportFileSelected(path),
                        None => Signal::InvalidAction("No file selected".to_string()),
                    },
                )
            }
            
            Signal::ImportFileSelected(path) => {
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
            
            Signal::ConfirmImport => {
                if let Some(import_data) = self.save_state.get_import_data() {
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
            
            Signal::CancelImport => {
                self.save_state.update_import_data(None);
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            Signal::OpenLegacyImportFilePicker => {
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
                        Some(path) => Signal::LegacyImportFileSelected(path),
                        None => Signal::InvalidAction("No file selected".to_string()),
                    },
                )
            }
            
            Signal::LegacyImportFileSelected(path) => {
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
            
            Signal::ConfirmLegacyImport => {
                if let Some(import_data) = self.save_state.get_legacy_import_data() {
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
            
            Signal::CancelLegacyImport => {
                self.save_state.update_legacy_import_data(None);
                self.app_state.update_page(Pages::Transactions);
                Task::none()
            }
            
            Signal::Backup => {
                self.backup_task()
            }
            
            Signal::FinishedBackingup(backup_results) => {
                if backup_results.is_fail() { self.app_state.pass_error(backup_results); }
                Task::none()
            }

            Signal::OpenDataLocation => {
                let result = save_engine::open_data_location_file_explorer();
                if result.is_fail() { self.app_state.pass_error(result); }
                Task::none()
            }
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
                        keyboard::Key::Named(Named::Tab) if modifiers.shift() => Some(Signal::FocusPrevious),
                        keyboard::Key::Named(Named::Tab) => Some(Signal::FocusNext),
                        
                        keyboard::Key::Character(c) => match c.as_str() {
                            "]" if modifiers.command() => Some(Signal::AdvanceDayKeybind),
                            "[" if modifiers.command() => Some(Signal::RecedeDayKeybind),
                            "'" if modifiers.command() => Some(Signal::AdvanceMonthKeybind),
                            ";" if modifiers.command() => Some(Signal::RecedeMonthKeybind),
                            "." if modifiers.command() => Some(Signal::AdvanceYearKeybind),
                            "," if modifiers.command() => Some(Signal::RecedeYearKeybind),
                            
                            "a" if modifiers.command() => Some(Signal::AddTransactionKeybind),
                            
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
        if self.app_state.get_critical_errors().is_empty() {
            // shows the helping page if the user is requesting help
            if self.app_state.is_helping() { help_page(self).into() }

            // displays the regular page system if the user does not want help
            else {
                match self.app_state.get_page() {
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
            sender.send(Signal::FinishedInteraction).await.ok();
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
        let theme = self.app_state.get_material_theme();
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            sender.send(Signal::StartedRenderingRingCharts).await.ok();
            
            let new_earning_ring_parse_result = match earning_ring_parse_result {
                Pass(earning_ring_parse) => RingParse::get_rendered(earning_ring_parse, theme).await,
                Fail(_) => (earning_ring_parse_result, Schrod::new_fail("Cannot rerender failed Ring Parse result!", "App::update_ring_parse_task()")),
            };
            
            let new_spending_ring_parse_result = match spending_ring_parse_result {
                Pass(spending_ring_parse) => RingParse::get_rendered(spending_ring_parse, theme).await,
                Fail(_) => (spending_ring_parse_result, Schrod::new_fail("Cannot rerender failed Ring Parse result!", "App::update_ring_parse_task()")),
            };
            
            sender.send(Signal::FinishedRenderingRingCharts(Box::new(new_earning_ring_parse_result), Box::new(new_spending_ring_parse_result))).await.ok();
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
                sender.send(Signal::FailedToRenderTrendParse).await.ok();
            }))
        }

        else {
            let mut trend_parse = self.trends_state.trend_parse_result().clone().wont_fail("This is past an is_fail() guard clause.", "App::update_trend_parse_task()");
            let tag_resistry_copy = self.bank.tag_registry.clone();
            let theme = self.app_state.get_material_theme();
            
            Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
                sender.send(Signal::StartedRenderingTrendParse).await.ok();
            
                let render_result = trend_parse.render(&tag_resistry_copy, theme);
                sender.send(Signal::FinishedRenderingTrendParse(trend_parse, render_result)).await.ok();
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
            sender.send(Signal::FinishedUpdatingCurrencyExchange(currency_exchange, refresh_result)).await.ok();
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
            sender.send(Signal::FinishedUpdatingTagRegistry(updated_tag_registry)).await.ok();
        }))
    }



    // save data utilities
    /// Returns a `Task` that saves persistent data to the disk.
    #[must_use]
    fn save_task(&mut self) -> Task<Signal> {
        let save_data = SaveData {
            theme: self.app_state.get_material_theme(),
            transactions: self.bank.get_ledger_copy(),
            currency_exchange: self.bank.currency_exchange.clone(),
            tag_registry: self.bank.tag_registry.clone(),
            bit_wallets: self.bit_bank.wallets.clone(),
        };
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let save_result = save(save_data).await;
            
            sender.send(Signal::FinishedSaving(save_result)).await.ok();
        }))
    }
    
    /// Returns a `Task` that backs up persistent data to the disk.
    #[must_use]
    fn backup_task(&mut self) -> Task<Signal> {
        let save_data = SaveData {
            theme: self.app_state.get_material_theme(),
            transactions: self.bank.get_ledger_copy(),
            currency_exchange: self.bank.currency_exchange.clone(),
            tag_registry: self.bank.tag_registry.clone(),
            bit_wallets: self.bit_bank.wallets.clone(),
        };
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let backup_result = backup(save_data).await;
            
            sender.send(Signal::FinishedBackingup(backup_result)).await.ok();
        }))
    }
}