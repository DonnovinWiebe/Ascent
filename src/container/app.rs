use iced::{Application, Element, Task, Theme};
use iced::widget::{button, column, container, text};
use iced::widget::text_editor::Content;
use crate::container::signal::Signal;
use crate::pages::transaction_management_pages::{add_transaction_page, edit_transaction_page};
use crate::pages::transactions_page::transactions_page;
use crate::pages::application_errors_page::application_errors_page;
use crate::ui::components::{DatePickerModes};
use crate::ui::material::{AppThemes};
use crate::vault::bank::*;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Date, Id, Months, Tag, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::parse::*;
use crate::ui::charting::RingChart;

/// The available pages in the app.
#[derive(Debug, Clone, Copy)]
pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    RemovingTransaction,
    Quitting,
}
impl Pages {
    pub fn name(&self) -> String {
        match self {
            Pages::Transactions => { "Transactions".to_string() }
            Pages::AddingTransaction => { "Adding Transaction".to_string() }
            Pages::EditingTransaction => { "Editing Transaction".to_string() }
            Pages::RemovingTransaction => { "Removing Transaction".to_string() }
            Pages::Quitting => { "Quitting".to_string() }
        }
    }
}



/// The central app.
/// This holds the bank and all ui/ux state information.
pub struct App {
    // basics
    pub bank: Bank,
    // app state
    pub theme_selection: AppThemes,
    pub application_failures: Vec<String>,
    theme: Theme,
    pub page: Pages,
    // bank display state
    value_display_format: ValueDisplayFormats,
    
    // transactions page state
    pub earning_ring_chart_result: ResultStack<RingChart>,
    pub spending_ring_chart_result: ResultStack<RingChart>,

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
}
impl Default for App {
    /// Returns a default App initialization.
    /// Used by Iced.
    fn default() -> Self {
        Self::new()
    }
}
impl App {
    // initializing
    /// Creates a new App.
    pub fn new() -> App {
        // initializes the bank
        let mut bank = Bank::new();
        bank.init();

        // creates the app
        let launch_theme = AppThemes::Midnight;
        let mut app = App {
            bank,
            theme_selection: launch_theme.clone(),
            application_failures: Vec::new(),
            theme: launch_theme.generate_iced_palette(&launch_theme),
            page: Pages::Transactions,
            value_display_format: ValueDisplayFormats::Dollars,
            
            earning_ring_chart_result: ResultStack::new_fail("No RingChart has been created."),
            spending_ring_chart_result: ResultStack::new_fail("No RingChart has been created."),

            new_transaction_value_string: "".to_string(),
            new_transaction_currency_string: "".to_string(),
            new_date_picker_mode: DatePickerModes::Hidden,
            new_transaction_current_year: Date::default().get_year(),
            new_transaction_current_month: *Date::default().get_month(),
            new_transaction_selected_date: Date::default(),
            new_transaction_description_content: Content::with_text(""),
            new_transaction_current_tag_string: "".to_string(),
            new_transaction_tags: Vec::new(),

            edit_transaction_id: 0,
            edit_transaction_value_string: "".to_string(),
            edit_transaction_currency_string: "".to_string(),
            edit_date_picker_mode: DatePickerModes::Hidden,
            edit_transaction_current_year: Date::default().get_year(),
            edit_transaction_current_month: *Date::default().get_month(),
            edit_transaction_selected_date: Date::default(),
            edit_transaction_description_content: Content::with_text(""),
            edit_transaction_current_tag_string: "".to_string(),
            edit_transaction_tags: Vec::new(),
            edit_transaction_is_delete_primed: false,
        };
        app.update_ring_parse_results();
        
        app
    }

    /// The tile of the app.
    pub fn title(&self) -> String {
        "Ascent".to_string()
    }



    // running
    /// Updates the app based on a given signal.
    /// Used by Iced.
    pub fn update(&mut self, signal: Signal) -> Task<Signal> {
        match signal {
            // general signals
            Signal::InvalidAction(_) => {
                eprintln!("Invalid action!");
            }
            
            Signal::DismissErrors => {
                self.application_failures.clear();
            }

            Signal::GoHome => {
                self.page = Pages::Transactions;
            }

            Signal::CycleTheme => {
                match self.theme_selection {
                    AppThemes::Peach => {
                        self.update_theme(AppThemes::Midnight);
                    }
                    AppThemes::Midnight => {
                        self.update_theme(AppThemes::Peach);
                    }
                }
            }



            // transactions page signals
            Signal::StartAddingTransaction => {
                self.new_transaction_value_string = "".to_string();
                self.new_transaction_currency_string = "".to_string();
                self.new_date_picker_mode = DatePickerModes::Hidden;
                self.edit_transaction_current_year = Date::default().get_year();
                self.edit_transaction_current_month = *Date::default().get_month();
                self.new_transaction_selected_date = Date::default();
                self.new_transaction_description_content = Content::with_text("");
                self.new_transaction_current_tag_string = "".to_string();
                self.new_transaction_tags = Vec::new();
                self.page = Pages::AddingTransaction;
            }

            Signal::StartEditingTransaction(id) => {
                let transaction_result = self.bank.get(id);
                
                if let Pass(transaction) = transaction_result {
                    self.edit_transaction_id = id;
                    self.edit_transaction_value_string = transaction.value.amount().to_string();
                    self.edit_transaction_currency_string = transaction.value.currency().to_string();
                    self.edit_date_picker_mode = DatePickerModes::Hidden;
                    self.edit_transaction_current_year = transaction.date.get_year();
                    self.edit_transaction_current_month = *transaction.date.get_month();
                    self.edit_transaction_selected_date = transaction.date.clone();
                    self.edit_transaction_description_content = Content::with_text(&transaction.description);
                    self.edit_transaction_current_tag_string = "".to_string();
                    self.edit_transaction_tags = transaction.tags.clone();
                    self.edit_transaction_is_delete_primed = false;
                    self.page = Pages::EditingTransaction;
                }
                else { self.application_failures.extend(transaction_result.results()); }
            }
            
            Signal::OpenTagRegistry => {
                
            }



            // adding transaction page signals
            Signal::AddTransaction => {
                let result = self.bank.add_transaction_from_raw_parts(
                    self.new_transaction_value_string.clone(),
                    self.new_transaction_currency_string.clone(),
                    self.new_transaction_selected_date.clone(),
                    self.new_transaction_description_content.text(),
                    self.new_transaction_tags.clone(),
                );
                
                if let Pass(_) = result {
                    self.update_ring_parse_results();
                    self.page = Pages::Transactions;
                }
                else { self.application_failures.extend(result.results()); }
            }
            
            Signal::UpdateNewTransactionValueString(new_value_string) => {
                self.new_transaction_value_string = new_value_string;
            }

            Signal::UpdateNewTransactionCurrencyString(new_currency_string) => {
                self.new_transaction_currency_string = new_currency_string;
            }

            Signal::UpdateNewTransactionDatePickerMode(new_mode) => {
                self.new_date_picker_mode = new_mode;
            }

            Signal::AdvanceNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_current_year >= 9999 { return Task::none(); }

                self.new_transaction_current_year += 1;
            }

            Signal::RecedeNewTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.new_transaction_current_year <= 1000 { return Task::none(); }

                self.new_transaction_current_year -= 1;
            }

            Signal::UpdateNewTransactionCurrentMonth(new_month) => {
                self.new_transaction_current_month = new_month;
                self.new_date_picker_mode = DatePickerModes::ShowingDaysInMonth;
            }
            
            Signal::UpdateNewTransactionSelectedDate(new_date) => {
                let new_date_result = new_date;
                
                if let Pass(new_date) = new_date_result {
                    self.new_transaction_selected_date = new_date;
                    self.new_date_picker_mode = DatePickerModes::Hidden;
                }
                else { self.application_failures.extend(new_date_result.results()); }
            }
            
            Signal::UpdateNewTransactionDescriptionContent(action) => {
                self.new_transaction_description_content.perform(action);
            }

            Signal::UpdateNewTransactionCurrentTagString(new_tag) => {
                self.new_transaction_current_tag_string = new_tag;
            }

            Signal::AddNewTransactionTag(tag_string) => {
                let new_tag_result = Tag::new(tag_string);
                
                if let Pass(new_tag) = new_tag_result {
                    self.new_transaction_tags.push(new_tag);
                    self.new_transaction_current_tag_string = "".to_string();
                    self.new_transaction_tags = Tag::sorted(self.edit_transaction_tags.clone());
                }
                else { self.application_failures.extend(new_tag_result.results()); }
            }

            Signal::RemoveNewTransactionTag(tag) => {
                self.new_transaction_tags.retain(|t| *t != tag);
            }



            // editing transaction page signals
            Signal::EditTransaction => {
                let result = self.bank.edit_transaction_with_raw_parts(
                    self.edit_transaction_id,
                    self.edit_transaction_value_string.clone(),
                    self.edit_transaction_currency_string.clone(),
                    self.edit_transaction_selected_date.clone(),
                    self.edit_transaction_description_content.text(),
                    self.edit_transaction_tags.clone(),
                );
                
                if let Pass(_) = result {
                    self.update_ring_parse_results();
                    self.page = Pages::Transactions;
                }
                else { self.application_failures.extend(result.results()); }
            }

            Signal::PrimeRemoveTransaction => {
                self.edit_transaction_is_delete_primed = true;
            }

            Signal::UnprimeRemoveTransaction => {
                self.edit_transaction_is_delete_primed = false;
            }

            Signal::RemoveTransaction => {
                let result = self.bank.remove_transaction(self.edit_transaction_id);
                
                if let Pass(_) = result {
                    self.update_ring_parse_results();
                    self.edit_transaction_is_delete_primed = false;
                    self.page = Pages::Transactions;
                }
                else { self.application_failures.extend(result.results()); }
            }
            
            Signal::UpdateEditTransactionValueString(new_value_string) => {
                self.edit_transaction_value_string = new_value_string;
            }

            Signal::UpdateEditTransactionCurrencyString(new_currency_string) => {
                self.edit_transaction_currency_string = new_currency_string;
            }

            Signal::UpdateEditTransactionDatePickerMode(new_mode) => {
                self.edit_date_picker_mode = new_mode;
            }

            Signal::AdvanceEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_current_year >= 9999 { return Task::none(); }

                self.edit_transaction_current_year += 1;
            }

            Signal::RecedeEditTransactionCurrentYear => {
                // do to technical reasons in how dates can be used, a date year must be four digits long
                if self.edit_transaction_current_year <= 1000 { return Task::none(); }

                self.edit_transaction_current_year -= 1;
            }

            Signal::UpdateEditTransactionCurrentMonth(new_month) => {
                self.edit_transaction_current_month = new_month;
                self.edit_date_picker_mode = DatePickerModes::ShowingDaysInMonth;
            }
            
            Signal::UpdateEditTransactionSelectedDate(new_date) => {
                let edit_date_result = new_date;
                
                if let Pass(new_date) = edit_date_result {
                    self.edit_transaction_selected_date = new_date;
                    self.edit_date_picker_mode = DatePickerModes::Hidden;
                }
                else { self.application_failures.extend(edit_date_result.results()); }
            }
            
            Signal::UpdateEditTransactionDescriptionContent(action) => {
                self.edit_transaction_description_content.perform(action);
            }

            Signal::UpdateEditTransactionCurrentTagString(new_tag) => {
                self.edit_transaction_current_tag_string = new_tag;
            }

            Signal::AddEditTransactionTag(tag_string) => {
                let new_tag_result = Tag::new(tag_string);
                
                if let Pass(new_tag) = new_tag_result {
                    self.edit_transaction_tags.push(new_tag);
                    self.edit_transaction_current_tag_string = "".to_string();
                    self.edit_transaction_tags = Tag::sorted(self.edit_transaction_tags.clone());
                }
                else { self.application_failures.extend(new_tag_result.results()); }
            }
            
            Signal::RemoveEditTransactionTag(tag) => {
                self.edit_transaction_tags.retain(|t| *t != tag);
            }
        }
        
        Task::none()
    }

    /// Renders the app.
    /// Used by Iced.
    pub fn view(&self) -> Element<Signal> {
        if !self.application_failures.is_empty() {
            return application_errors_page(self).into();
        }
        
        else {
            match self.page {
                Pages::Transactions => { transactions_page(self).into() }
                Pages::AddingTransaction => { add_transaction_page(self).into() }
                Pages::EditingTransaction => { edit_transaction_page(self).into() }
                Pages::RemovingTransaction => { transactions_page(self).into() }
                Pages::Quitting => { transactions_page(self).into() }
            }
        }
    }

    /// Gets the current theme.
    /// Used by Iced
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// Updates the theme of the app.
    pub fn update_theme(&mut self, new_theme_selection: AppThemes) {
        self.theme_selection = new_theme_selection;
        self.theme = self.theme_selection.generate_iced_palette(&self.theme_selection);
    }
    
    /// Updates the ring parse result for the earning and spending rings.
    fn update_ring_parse_results(&mut self) {
        self.update_earning_ring_parse_result();
        self.update_spending_ring_parse_result();
    }
    
    /// Updates the ring parse result for the earning ring.
    fn update_earning_ring_parse_result(&mut self) {
        let new_earning_ring_chart_result = RingChart::new(self, &self.bank, Filters::Primary, FlowDirections::Earning);
        if new_earning_ring_chart_result.is_fail() { self.application_failures.extend(new_earning_ring_chart_result.results()); }
        self.earning_ring_chart_result = new_earning_ring_chart_result;
    }

    /// Updates the ring parse result for the spending ring.
    fn update_spending_ring_parse_result(&mut self) {
        let new_spending_ring_chart_result = RingChart::new(self, &self.bank, Filters::Primary, FlowDirections::Spending);
        if new_spending_ring_chart_result.is_fail() { self.application_failures.extend(new_spending_ring_chart_result.results()); }
        self.spending_ring_chart_result = new_spending_ring_chart_result;
    }
}