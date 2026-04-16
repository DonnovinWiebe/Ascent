use iced::{Element, Task, Theme};
use iced::widget::text_editor::Content;
use crate::container::signal::Signal;
use crate::pages::settings_page::settings_page;
use crate::pages::transaction_management_pages::{add_transaction_page, edit_transaction_page};
use crate::pages::transactions_page::transactions_page;
use crate::pages::tag_registry_page::{TagRegistrationSlipStateManager, tag_registry_page};
use crate::pages::application_errors_page::application_errors_page;
use crate::ui::components::DatePickerModes;
use crate::ui::material::AppThemes;
use crate::vault::bank::{Bank, Filters, TagRegistry};
use crate::vault::transaction::{Date, Id, Months, Tag/*, ValueDisplayFormats*/};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::parse::{CashFlow, FlowDirections, RingParse, Segment};
use iced::futures::SinkExt;
use iced::futures::channel::mpsc::Sender;
use crate::vault::save_engine::{self, SaveData};

/// The available pages in the `App`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    TagRegistry,
    Settings,
}
impl Pages {
    /// Returns the name for a given `Page`.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Pages::Transactions => { "Transactions".to_string() }
            Pages::AddingTransaction => { "Adding Transaction".to_string() }
            Pages::EditingTransaction => { "Editing Transaction".to_string() }
            Pages::TagRegistry => { "Tag Registry".to_string() }
            Pages::Settings => { "Settings".to_string() }
        }
    }
    
    /// Returns the icon name for a given `Page`.
    #[must_use]
    pub fn icon_name(&self) -> &'static str {
        match self {
            Pages::Transactions => "money-bill",
            Pages::AddingTransaction => "plus",
            Pages::EditingTransaction => "pencil",
            Pages::TagRegistry => "tags",
            Pages::Settings => "gear",
        }
    }
}



/// The central application container.
/// This holds the `Bank` and all ui/ux state information.
#[allow(clippy::struct_excessive_bools)] // This is more ergonomic than using enums for bool flags.
pub struct App {
    // basics
    saved_successfully: bool,
    loaded_successfully: bool,
    
    //does_save_file_exist: bool, // todo: implement a notice
    pub bank: Bank,
    
    // bank display state
    pub cash_flow_result: ResultStack<CashFlow>,
    //value_display_format: ValueDisplayFormats, // todo: implement for cash flow information
    
    // app state
    pub theme_selection: AppThemes,
    pub application_failures: Vec<String>,
    theme: Theme,
    pub page: Pages,
    
    // transactions page state
    pub are_ring_charts_ready: bool,
    pub earning_ring_parse_result: ResultStack<RingParse>,
    pub spending_ring_parse_result: ResultStack<RingParse>,
    pub hovered_segment: Option<Segment>,
    
    // filtering
    pub primary_filter_current_search_term_string: String,
    pub deep_dive_1_filter_current_search_term_string: String,
    pub deep_dive_2_filter_current_search_term_string: String,

    // new transaction state information
    pub new_transaction_value_string: String,
    pub new_transaction_currency_string: String,
    pub new_date_picker_mode: DatePickerModes,
    pub new_transaction_current_year: u32,
    pub new_transaction_current_month: Months,
    pub new_transaction_selected_date: Date,
    pub new_transaction_description_content: Content,
    pub new_transaction_current_tag_string: String,
    pub new_transaction_tags: Vec<Tag>,

    // edit transaction state information
    pub edit_transaction_id: Id,
    pub edit_transaction_value_string: String,
    pub edit_transaction_currency_string: String,
    pub edit_date_picker_mode: DatePickerModes,
    pub edit_transaction_current_year: u32,
    pub edit_transaction_current_month: Months,
    pub edit_transaction_selected_date: Date,
    pub edit_transaction_description_content: Content,
    pub edit_transaction_current_tag_string: String,
    pub edit_transaction_tags: Vec<Tag>,
    pub edit_transaction_is_delete_primed: bool,
    
    // tag registry page state information
    pub tag_registry_slip_state_manager: TagRegistrationSlipStateManager,
}
impl Default for App {
    /// Returns a default `App` initialization.
    /// Used by Iced.
    fn default() -> App {
        App::new()
    }
}
impl App {
    // initializing
    /// Creates a new `App`.
    #[must_use]
    pub fn new() -> App {
        // loading failure tracking
        let mut loaded_successfully = true;
        let mut initializing_failures = Vec::new();
        
        // general failure tracking
        let mut general_failures = Vec::new();
        
        // getting the save data
        let save_data_result = save_engine::load();
        if save_data_result.is_fail() {
            loaded_successfully = false;
            initializing_failures.extend(save_data_result.results());
        }
        
        // loading the theme
        let theme = match &save_data_result {
            ResultStack::Pass(save_data) => save_data.theme,
            ResultStack::Fail(_) => AppThemes::Midnight,
        };
        
        // loading the transactions
        let transactions = match &save_data_result {
            ResultStack::Pass(save_data) => save_data.transactions.clone(),
            ResultStack::Fail(_) => Vec::new(),
        };
        
        // loading the tag registry
        let tag_registry = match &save_data_result {
            ResultStack::Pass(save_data) => save_data.tag_registry.clone(),
            ResultStack::Fail(_) => TagRegistry::default(),
        };
        
        // loading the bank
        let mut bank = Bank::default();
        bank.init(transactions, tag_registry);
        let tags = bank.get_tags();
        
        // bank display state
        let cash_flow_result = CashFlow::new(&bank.get_filtered_ids(Filters::Primary), &bank, 1.0);
        if cash_flow_result.is_fail() { general_failures.extend(cash_flow_result.results()); }
        
        // creates the app
        let mut app = App {
            saved_successfully: true,
            loaded_successfully,
            //does_save_file_exist: save_engine::does_save_file_exist(),
            bank,
            
            cash_flow_result,
            
            theme_selection: theme,
            application_failures: Vec::new(),
            theme: theme.generate_iced_palette(),
            page: Pages::Transactions,
            //value_display_format: ValueDisplayFormats::Dollars,
            
            are_ring_charts_ready: false,
            earning_ring_parse_result: ResultStack::new_fail("No RingParse has been created."),
            spending_ring_parse_result: ResultStack::new_fail("No RingParse has been created."),
            hovered_segment: None,
            
            primary_filter_current_search_term_string: String::new(),
            deep_dive_1_filter_current_search_term_string: String::new(),
            deep_dive_2_filter_current_search_term_string: String::new(),

            new_transaction_value_string: String::new(),
            new_transaction_currency_string: String::new(),
            new_date_picker_mode: DatePickerModes::Hidden,
            new_transaction_current_year: Date::default().get_year(),
            new_transaction_current_month: Date::default().get_month(),
            new_transaction_selected_date: Date::default(),
            new_transaction_description_content: Content::with_text(""),
            new_transaction_current_tag_string: String::new(),
            new_transaction_tags: Vec::new(),

            edit_transaction_id: 0,
            edit_transaction_value_string: String::new(),
            edit_transaction_currency_string: String::new(),
            edit_date_picker_mode: DatePickerModes::Hidden,
            edit_transaction_current_year: Date::default().get_year(),
            edit_transaction_current_month: Date::default().get_month(),
            edit_transaction_selected_date: Date::default(),
            edit_transaction_description_content: Content::with_text(""),
            edit_transaction_current_tag_string: String::new(),
            edit_transaction_tags: Vec::new(),
            edit_transaction_is_delete_primed: false,
            
            tag_registry_slip_state_manager: TagRegistrationSlipStateManager::new(tags),
        };
        
        // updating the ring chart
        app.update_ring_parse_results();
        if let Pass(earning_ring_parse) = &mut app.earning_ring_parse_result {
            let earning_render_result = earning_ring_parse.render(theme);
            if earning_render_result.is_fail() { app.application_failures.extend(earning_render_result.results()); }
            let earning_stop_hovering_result = earning_ring_parse.stop_hovering();
            if earning_stop_hovering_result.is_fail() { app.application_failures.extend(earning_stop_hovering_result.results()); }
        }
        if let Pass(spending_ring_parse) = &mut app.spending_ring_parse_result {
            let spending_render_result = spending_ring_parse.render(theme);
            if spending_render_result.is_fail() { app.application_failures.extend(spending_render_result.results()); }
            let spending_stop_hovering_result = spending_ring_parse.stop_hovering();
            if spending_stop_hovering_result.is_fail() { app.application_failures.extend(spending_stop_hovering_result.results()); }
        }
        app.are_ring_charts_ready = true;
        
        // checking for loading failures
        if !loaded_successfully { app.application_failures = initializing_failures; }
        
        // checking for other failures
        if !general_failures.is_empty() { app.application_failures.extend(general_failures); }
        
        // returning the app
        app
    }

    /// The tile of the `App`.
    #[must_use]
    pub fn title(&self) -> String {
        "Ascent".to_string()
    }



    // running
    /// Updates the `App` based on a given `Signal`.
    /// Used by Iced.
    #[allow(clippy::too_many_lines)] // This is going to be large since it is the central signal handler.
    pub fn update(&mut self, signal: Signal) -> Task<Signal> {
        // does not allow any changes if the app did not save or load successfully
        if !self.saved_successfully || !self.loaded_successfully {
            return Task::none();
        }
    
        // if the app loaded successfully, the app runs as normal
        match signal {
            // general signals
            Signal::FinishedUpdatingTagRegistry(updated_tag_registry) => {
                self.bank.tag_registry = updated_tag_registry;
                let tags = self.bank.get_tags();
                self.tag_registry_slip_state_manager = TagRegistrationSlipStateManager::new(tags);
                Task::none()
            }
            
            Signal::StartedSaving => {
                Task::none()
            }
            
            Signal::FinishedSaving(save_result) => {
                match save_result {
                    Pass(()) => {
                        self.saved_successfully = true;
                    }
                    Fail(_) => {
                        self.saved_successfully = false;
                        self.application_failures.extend(save_result.results());
                    }
                }
                
                Task::none()
            }
            
            Signal::InvalidAction(_) => {
                eprintln!("Invalid action!");
                Task::none()
            }
            
            Signal::DismissErrors => {
                self.application_failures.clear();
                Task::none()
            }
            
            Signal::ChangePageTo(page) => {
                self.page = page;
                Task::none()
            }

            Signal::GoHome => {
                self.page = Pages::Transactions;
                Task::none()
            }
            
            
            
            // filtering
            Signal::SetFilterYear(year, filter) => {
                let filter_result = self.bank.set_filter_year(year, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::ClearFilterYear(filter) => {
                let filter_result = self.bank.clear_filter_year(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::SetFilterMonth(month, filter) => {
                let filter_result = self.bank.set_filter_month(month, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::ClearFilterMonth(filter) => {
                let filter_result = self.bank.clear_filter_month(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::AddFilterTag(tag, filter) => {
                let filter_result = self.bank.add_filter_tag(&tag, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
        
            Signal::RemoveFilterTag(tag, filter) => {
                let filter_result = self.bank.remove_filter_tag(&tag, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::ClearFilterTags(filter) => {
                let filter_result = self.bank.clear_filter_tags(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::UpdatePrimaryFilterCurrentSearchTermString(term) => {
                self.primary_filter_current_search_term_string = term;
                Task::none()
            }
            
            Signal::UpdateDeepDive1FilterCurrentSearchTermString(term) => {
                self.deep_dive_1_filter_current_search_term_string = term;
                Task::none()
            }
            
            Signal::UpdateDeepDive2FilterCurrentSearchTermString(term) => {
                self.deep_dive_2_filter_current_search_term_string = term;
                Task::none()
            }
            
            Signal::AddFilterSearchTerm(filter) => {
                let term = match filter {
                    Filters::Primary => self.primary_filter_current_search_term_string.clone(),
                    Filters::DeepDive1 => self.deep_dive_1_filter_current_search_term_string.clone(),
                    Filters::DeepDive2 => self.deep_dive_2_filter_current_search_term_string.clone(),
                };
                
                match filter {
                    Filters::Primary => self.primary_filter_current_search_term_string = String::new(),
                    Filters::DeepDive1 => self.deep_dive_1_filter_current_search_term_string = String::new(),
                    Filters::DeepDive2 => self.deep_dive_2_filter_current_search_term_string = String::new(),
                }
                
                let filter_result = self.bank.add_filter_search_term(&term, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::RemoveFilterSearchTerm(term, filter) => {
                let filter_result = self.bank.remove_filter_search_term(&term, filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::ClearFilterSearchTerms(filter) => {
                let filter_result = self.bank.clear_filter_search_terms(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::ToggleFilterMode(filter) => {
                let filter_result = self.bank.toggle_filter_mode(filter);
                match filter_result {
                    Pass(()) => {
                        self.update_cash_flow_result();
                        self.update_ring_parse_task()
                    }
                    Fail(_) => {
                        self.application_failures.extend(filter_result.results());
                        Task::none()
                    }
                }
            }



            // transactions page signals
            Signal::StartAddingTransaction => {
                self.new_transaction_value_string = String::new();
                self.new_transaction_currency_string = String::new();
                self.new_date_picker_mode = DatePickerModes::Hidden;
                self.edit_transaction_current_year = Date::default().get_year();
                self.edit_transaction_current_month = Date::default().get_month();
                self.new_transaction_selected_date = Date::default();
                self.new_transaction_description_content = Content::with_text("");
                self.new_transaction_current_tag_string = String::new();
                self.new_transaction_tags = Vec::new();
                self.page = Pages::AddingTransaction;
                
                Task::none()
            }

            Signal::StartEditingTransaction(id_result) => {
                if id_result.is_fail() {
                    self.application_failures.extend(id_result.results());
                    return Task::none();
                }
                let id = id_result.wont_fail("This is past an is_fail() guard clause.");
                let transaction_result = self.bank.get(id);
                
                match transaction_result {
                    Pass(transaction) => {
                        self.edit_transaction_id = id;
                        self.edit_transaction_value_string = transaction.value.amount().to_string();
                        self.edit_transaction_currency_string = transaction.value.currency().to_string();
                        self.edit_date_picker_mode = DatePickerModes::Hidden;
                        self.edit_transaction_current_year = transaction.date.get_year();
                        self.edit_transaction_current_month = transaction.date.get_month();
                        self.edit_transaction_selected_date = transaction.date;
                        self.edit_transaction_description_content = Content::with_text(&transaction.description);
                        self.edit_transaction_current_tag_string = String::new();
                        self.edit_transaction_tags = transaction.tags.clone();
                        self.edit_transaction_is_delete_primed = false;
                        self.page = Pages::EditingTransaction;
                    }
                    Fail(_) => { self.application_failures.extend(transaction_result.results()); }
                }
                
                Task::none()
            }
            
            Signal::MouseMovedInEarningRingChart(new_pos, layout_size) => {
                // checks if the ring parse is valid
                if self.earning_ring_parse_result.is_pass() {
                    // updates hovering
                    let update_hovering_result = self.earning_ring_parse_result.wont_fail_ref_mut("This is inside an is_pass() block.").update_hovering(new_pos, layout_size);
                    if update_hovering_result.is_fail() { self.application_failures.extend(update_hovering_result.results()); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.earning_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.earning_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_segment(&tag);
                            match hovered_segment_result {
                                ResultStack::Pass(segment) => {
                                    self.hovered_segment = Some(segment.clone());
                                }
                                ResultStack::Fail(_) => {
                                    self.hovered_segment = None;
                                    self.application_failures.extend(hovered_segment_result.results());
                                }
                            }
                        }
                        None => { self.hovered_segment = None; }
                    }
                }
                
                Task::none()
            }
            
            Signal::MouseMovedInSpendingRingChart(new_pos, layout_size) => {
                // checks if the ring parse is valid
                if self.spending_ring_parse_result.is_pass() {
                    // updates hovering
                    let update_hovering_result = self.spending_ring_parse_result.wont_fail_ref_mut("This is inside an is_pass() block.").update_hovering(new_pos, layout_size);
                    if update_hovering_result.is_fail() { self.application_failures.extend(update_hovering_result.results()); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.spending_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.spending_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_segment(&tag);
                            match hovered_segment_result {
                                ResultStack::Pass(segment) => {
                                    self.hovered_segment = Some(segment.clone());
                                }
                                ResultStack::Fail(_) => {
                                    self.hovered_segment = None;
                                    self.application_failures.extend(hovered_segment_result.results());
                                }
                            }
                        }
                        None => { self.hovered_segment = None; }
                    }
                }
                
                Task::none()
            }
            
            Signal::MouseExitedEarningRingChart => {
                // checks if the ring parse is valid
                if self.earning_ring_parse_result.is_pass() {
                    // updates hovering
                    let stop_hovering_result = self.earning_ring_parse_result.wont_fail_ref_mut("This is inside an is_pass() block.").stop_hovering();
                    if stop_hovering_result.is_fail() { self.application_failures.extend(stop_hovering_result.results()); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.earning_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.earning_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_segment(&tag);
                            match hovered_segment_result {
                                ResultStack::Pass(segment) => {
                                    self.hovered_segment = Some(segment.clone());
                                }
                                ResultStack::Fail(_) => {
                                    self.hovered_segment = None;
                                    self.application_failures.extend(hovered_segment_result.results());
                                }
                            }
                        }
                        None => { self.hovered_segment = None; }
                    }
                }
                
                Task::none()
            }
            
            Signal::MouseExitedSpendingRingChart => {
                // checks if the ring parse is valid
                if self.spending_ring_parse_result.is_pass() {
                    // updates hovering
                    let stop_hovering_result = self.spending_ring_parse_result.wont_fail_ref_mut("This is inside an is_pass() block.").stop_hovering();
                    if stop_hovering_result.is_fail() { self.application_failures.extend(stop_hovering_result.results()); }
                    
                    // updates the hovered segment
                    let hovered_tag = self.spending_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_hovered_tag();
                    match hovered_tag {
                        Some(tag) => {
                            let hovered_segment_result = self.spending_ring_parse_result.wont_fail_ref("This is inside an is_pass() block.").get_segment(&tag);
                            match hovered_segment_result {
                                ResultStack::Pass(segment) => {
                                    self.hovered_segment = Some(segment.clone());
                                }
                                ResultStack::Fail(_) => {
                                    self.hovered_segment = None;
                                    self.application_failures.extend(hovered_segment_result.results());
                                }
                            }
                        }
                        None => { self.hovered_segment = None; }
                    }
                }
                
                Task::none()
            }
            
            Signal::OpenTagRegistry => {
                self.page = Pages::TagRegistry;
                Task::none()
            }
            
            Signal::StartedRenderingRingCharts => {
                self.are_ring_charts_ready = false;
                Task::none()
            }
            
            Signal::FinishedRenderingRingCharts(rendered_earning_ring_parse_result, rendered_spending_ring_parse_result) => {
                let (earning_ring_parse_result, earning_ring_parse_render_results) = *rendered_earning_ring_parse_result;
                let (spending_ring_parse_result, spending_ring_parse_render_results) = *rendered_spending_ring_parse_result;
                self.earning_ring_parse_result = earning_ring_parse_result;
                self.spending_ring_parse_result = spending_ring_parse_result;
                if earning_ring_parse_render_results.is_fail() { self.application_failures.extend(earning_ring_parse_render_results.results()); }
                if spending_ring_parse_render_results.is_fail() { self.application_failures.extend(spending_ring_parse_render_results.results()); }
                self.are_ring_charts_ready = true;
                Task::none()
            }
            
            

            // adding transaction page signals
            Signal::AddTransaction => {
                let result = self.bank.add_transaction_from_raw_parts(
                    &self.new_transaction_value_string,
                    &self.new_transaction_currency_string,
                    self.new_transaction_selected_date,
                    self.new_transaction_description_content.text(),
                    self.new_transaction_tags.clone(),
                );
                
                match result {
                    Pass(()) => {
                        self.page = Pages::Transactions;
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_tag_registry_task(),
                            self.save_task(),
                            self.update_ring_parse_task(),
                        ])
                    }
                    Fail(_) => {
                        self.application_failures.extend(result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::UpdateNewTransactionValueString(new_value_string) => {
                self.new_transaction_value_string = new_value_string;
                Task::none()
            }

            Signal::UpdateNewTransactionCurrencyString(new_currency_string) => {
                self.new_transaction_currency_string = new_currency_string;
                Task::none()
            }

            Signal::UpdateNewTransactionDatePickerMode(new_mode) => {
                self.new_date_picker_mode = new_mode;
                Task::none()
            }

            Signal::AdvanceNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_current_year >= 9999 { return Task::none(); }
                self.new_transaction_current_year += 1;
                Task::none()
            }

            Signal::RecedeNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_current_year <= 1000 { return Task::none(); }
                self.new_transaction_current_year -= 1;
                Task::none()
            }

            Signal::UpdateNewTransactionCurrentMonth(new_month) => {
                self.new_transaction_current_month = new_month;
                self.new_date_picker_mode = DatePickerModes::ShowingDaysInMonth;
                Task::none()
            }
            
            Signal::UpdateNewTransactionSelectedDate(new_date_result) => {
                match new_date_result {
                    Pass(new_date) => {
                        self.new_transaction_selected_date = new_date;
                        self.new_date_picker_mode = DatePickerModes::Hidden;
                    }
                    Fail(_) => { self.application_failures.extend(new_date_result.results()); }
                }
                
                Task::none()
            }
            
            Signal::UpdateNewTransactionDescriptionContent(action) => {
                self.new_transaction_description_content.perform(action);
                Task::none()
            }

            Signal::UpdateNewTransactionCurrentTagString(new_tag) => {
                self.new_transaction_current_tag_string = new_tag;
                Task::none()
            }

            Signal::AddNewTransactionTag(tag_string) => {
                let new_tag_result = Tag::new(&tag_string);
                
                match new_tag_result {
                    Pass(new_tag) => {
                        self.new_transaction_tags.push(new_tag);
                        self.new_transaction_current_tag_string = String::new();
                        self.new_transaction_tags = Tag::sorted(&self.new_transaction_tags);
                    }
                    Fail(_) => { self.application_failures.extend(new_tag_result.results()); }
                }
                
                Task::none()
            }

            Signal::RemoveNewTransactionTag(tag) => {
                self.new_transaction_tags.retain(|t| *t != tag);
                Task::none()
            }



            // editing transaction page signals
            Signal::EditTransaction => {
                let result = self.bank.edit_transaction_with_raw_parts(
                    self.edit_transaction_id,
                    &self.edit_transaction_value_string,
                    &self.edit_transaction_currency_string,
                    self.edit_transaction_selected_date,
                    self.edit_transaction_description_content.text(),
                    self.edit_transaction_tags.clone(),
                );
                
                match result {
                    Pass(()) => {
                        self.page = Pages::Transactions;
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_tag_registry_task(),
                            self.save_task(),
                            self.update_ring_parse_task(),
                        ])
                    }
                    Fail(_) => {
                        self.application_failures.extend(result.results());
                        Task::none()
                    }
                }
            }

            Signal::PrimeRemoveTransaction => {
                self.edit_transaction_is_delete_primed = true;
                Task::none()
            }

            Signal::UnprimeRemoveTransaction => {
                self.edit_transaction_is_delete_primed = false;
                Task::none()
            }

            Signal::RemoveTransaction => {
                let result = self.bank.remove_transaction(self.edit_transaction_id);
                
                match result {
                    Pass(()) => {
                        self.edit_transaction_is_delete_primed = false;
                        self.page = Pages::Transactions;
                        self.update_cash_flow_result();
                        Task::batch(vec![
                            self.update_tag_registry_task(),
                            self.save_task(),
                            self.update_ring_parse_task(),
                        ])
                    }
                    Fail(_) => {
                        self.application_failures.extend(result.results());
                        Task::none()
                    }
                }
            }
            
            Signal::UpdateEditTransactionValueString(new_value_string) => {
                self.edit_transaction_value_string = new_value_string;
                Task::none()
            }

            Signal::UpdateEditTransactionCurrencyString(new_currency_string) => {
                self.edit_transaction_currency_string = new_currency_string;
                Task::none()
            }

            Signal::UpdateEditTransactionDatePickerMode(new_mode) => {
                self.edit_date_picker_mode = new_mode;
                Task::none()
            }

            Signal::AdvanceEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_current_year >= 9999 { return Task::none(); }
                self.edit_transaction_current_year += 1;
                Task::none()
            }

            Signal::RecedeEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_current_year <= 1000 { return Task::none() }
                self.edit_transaction_current_year -= 1;
                Task::none()
            }

            Signal::UpdateEditTransactionCurrentMonth(new_month) => {
                self.edit_transaction_current_month = new_month;
                self.edit_date_picker_mode = DatePickerModes::ShowingDaysInMonth;
                Task::none()
            }
            
            Signal::UpdateEditTransactionSelectedDate(edit_date_result) => {
                match edit_date_result {
                    Pass(new_date) => {
                        self.edit_transaction_selected_date = new_date;
                        self.edit_date_picker_mode = DatePickerModes::Hidden;
                    }
                    Fail(_) => { self.application_failures.extend(edit_date_result.results()); }
                }
                
                Task::none()
            }
            
            Signal::UpdateEditTransactionDescriptionContent(action) => {
                self.edit_transaction_description_content.perform(action);
                Task::none()
            }

            Signal::UpdateEditTransactionCurrentTagString(new_tag) => {
                self.edit_transaction_current_tag_string = new_tag;
                Task::none()
            }

            Signal::AddEditTransactionTag(tag_string) => {
                let new_tag_result = Tag::new(&tag_string);
                
                match new_tag_result {
                    Pass(new_tag) => {
                        self.edit_transaction_tags.push(new_tag);
                        self.edit_transaction_current_tag_string = String::new();
                        self.edit_transaction_tags = Tag::sorted(&self.edit_transaction_tags);
                    }
                    Fail(_) => { self.application_failures.extend(new_tag_result.results()); }
                }
                
                Task::none()
            }
            
            Signal::RemoveEditTransactionTag(tag) => {
                self.edit_transaction_tags.retain(|t| *t != tag);
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
                ])
            }
            
            // settings page signals
            Signal::ChangeTheme(theme) => {
                self.update_theme(theme);
                Task::batch(vec![
                    self.save_task(),
                    self.update_ring_parse_task(),
                ])
            }
        }
    }

    /// Renders the `App`.
    /// Used by Iced.
    pub fn view<'a>(&'a self) -> Element<'a, Signal> {
        if self.application_failures.is_empty() {
            match self.page {
                Pages::Transactions => { transactions_page(self).into() }
                Pages::AddingTransaction => { add_transaction_page(self).into() }
                Pages::EditingTransaction => { edit_transaction_page(self).into() }
                Pages::TagRegistry => { tag_registry_page(self).into() }
                Pages::Settings => { settings_page(self).into() }
            }
        }
        
        else { application_errors_page(self).into() }
    }

    /// Gets the current `Theme`.
    /// Used by Iced
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// Updates the `Theme` of the `App`.
    pub fn update_theme(&mut self, new_theme_selection: AppThemes) {
        self.theme_selection = new_theme_selection;
        self.theme = self.theme_selection.generate_iced_palette();
    }
    
    /// Updates the `cash_flow_result` for the `App`.
    fn update_cash_flow_result(&mut self) {
        let new_cash_flow_result = CashFlow::new(&self.bank.get_filtered_ids(Filters::Primary), &self.bank, 1.0);
        if new_cash_flow_result.is_fail() { self.application_failures.extend(new_cash_flow_result.results()); }
        self.cash_flow_result = new_cash_flow_result;
    }
    
    /// Updates the ring parse result for the earning and spending rings.
    fn update_ring_parse_results(&mut self) {
        let new_earning_ring_parse_result = RingParse::new(self, &self.bank, Filters::Primary, FlowDirections::Earning);
        if new_earning_ring_parse_result.is_fail() { self.application_failures.extend(new_earning_ring_parse_result.results()); }
        self.earning_ring_parse_result = new_earning_ring_parse_result;
        
        let new_spending_ring_parse_result = RingParse::new(self, &self.bank, Filters::Primary, FlowDirections::Spending);
        if new_spending_ring_parse_result.is_fail() { self.application_failures.extend(new_spending_ring_parse_result.results()); }
        self.spending_ring_parse_result = new_spending_ring_parse_result;
    }
    
    /// Returns a `Task` that updates the `RingParse` results for the earning and spending rings.
    fn update_ring_parse_task(&mut self) -> Task<Signal> {
        self.update_ring_parse_results();
        
        let earning_ring_parse_result = self.earning_ring_parse_result.clone();
        let spending_ring_parse_result = self.spending_ring_parse_result.clone();
        let theme = self.theme_selection;
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            sender.send(Signal::StartedRenderingRingCharts).await.ok();
            
            let new_earning_ring_parse_result = match earning_ring_parse_result {
                Pass(earning_ring_parse) => RingParse::get_rendered(earning_ring_parse, theme).await,
                Fail(_) => (earning_ring_parse_result, ResultStack::new_fail("Cannot rerender failed Ring Parse result!")),
            };
            
            let new_spending_ring_parse_result = match spending_ring_parse_result {
                Pass(spending_ring_parse) => RingParse::get_rendered(spending_ring_parse, theme).await,
                Fail(_) => (spending_ring_parse_result, ResultStack::new_fail("Cannot rerender failed Ring Parse result!")),
            };
            
            sender.send(Signal::FinishedRenderingRingCharts(Box::new(new_earning_ring_parse_result), Box::new(new_spending_ring_parse_result))).await.ok();
        }))
    }
    
    
    /// Returns a `Task` that updates the `TagRegistry` based on the current `Tag`s in the `Bank`.
    fn update_tag_registry_task(&mut self) -> Task<Signal> {
        let old_tag_registry = self.bank.tag_registry.clone();
        let tags = self.bank.get_tags();
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            let updated_tag_registry = Bank::get_updated_tag_registry(old_tag_registry, tags);
            sender.send(Signal::FinishedUpdatingTagRegistry(updated_tag_registry)).await.ok();
        }))
    }
    
    /// Returns a `Task` that saves persistent data to the disk.
    fn save_task(&mut self) -> Task<Signal> {
        let save_data = SaveData {
            theme: self.theme_selection,
            transactions: self.bank.get_ledger_copy(),
            tag_registry: self.bank.tag_registry.clone(),
        };
        
        Task::stream(iced::stream::channel(16, move |mut sender: Sender<Signal>| async move {
            sender.send(Signal::StartedSaving).await.ok();
            
            let save_results = save_engine::save(save_data).await;
            
            sender.send(Signal::FinishedSaving(save_results)).await.ok();
        }))
    }
}