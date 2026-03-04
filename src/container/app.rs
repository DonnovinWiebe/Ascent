use iced::{Application, Element, Task, Theme};
use iced::widget::{button, column, container, text};
use iced::widget::text_editor::Content;
use crate::container::signal::Signal;
use crate::pages::edit_transaction_page::edit_transaction_page;
use crate::pages::transactions_page::transactions_page;
use crate::ui::components::{DatePickerModes};
use crate::ui::material::{AppThemes};
use crate::vault::bank::*;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Date, Id, Months, Tag, Transaction, ValueDisplayFormats};

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
    theme: Theme,
    pub page: Pages,
    // bank display state
    value_display_format: ValueDisplayFormats,

    // new transaction state information
    pub new_transaction_value_string: String,
    pub new_transaction_currency_string: String,
    pub new_date_picker_mode: DatePickerModes,
    pub new_transaction_current_year: u32,
    pub new_transaction_current_month: Months,
    pub new_transaction_selected_date: Date,
    pub new_transaction_description_content: Content,
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
    pub edit_transaction_tags: Vec<Tag>,
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
        App {
            bank,
            theme_selection: launch_theme.clone(),
            theme: launch_theme.generate_iced_palette(&launch_theme),
            page: Pages::Transactions,
            value_display_format: ValueDisplayFormats::Dollars,

            new_transaction_value_string: "".to_string(),
            new_transaction_currency_string: "".to_string(),
            new_date_picker_mode: DatePickerModes::Hidden,
            new_transaction_current_year: Date::default().get_year(),
            new_transaction_current_month: *Date::default().get_month(),
            new_transaction_selected_date: Date::default(),
            new_transaction_description_content: Content::with_text(""),
            new_transaction_tags: Vec::new(),

            edit_transaction_id: 0,
            edit_transaction_value_string: "".to_string(),
            edit_transaction_currency_string: "".to_string(),
            edit_date_picker_mode: DatePickerModes::Hidden,
            edit_transaction_current_year: Date::default().get_year(),
            edit_transaction_current_month: *Date::default().get_month(),
            edit_transaction_selected_date: Date::default(),
            edit_transaction_description_content: Content::with_text(""),
            edit_transaction_tags: Vec::new(),
        }
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
                eprintln!("Starting adding transaction...");
                self.new_transaction_value_string = "".to_string();
                self.new_transaction_currency_string = "".to_string();
                self.new_date_picker_mode = DatePickerModes::Hidden;
                self.edit_transaction_current_year = Date::default().get_year();
                self.edit_transaction_current_month = *Date::default().get_month();
                self.new_transaction_selected_date = Date::default();
                self.new_transaction_description_content = Content::with_text("");
                self.new_transaction_tags = Vec::new();
            }

            Signal::StartEditingTransaction(id) => {
                let transaction = self.bank.get(id);
                self.edit_transaction_id = id;
                self.edit_transaction_value_string = transaction.value.amount().to_string();
                self.edit_transaction_currency_string = transaction.value.currency().to_string();
                self.edit_date_picker_mode = DatePickerModes::Hidden;
                self.edit_transaction_current_year = transaction.date.get_year();
                self.edit_transaction_current_month = *transaction.date.get_month();
                self.edit_transaction_selected_date = transaction.date.clone();
                self.edit_transaction_description_content = Content::with_text(&transaction.description);
                self.edit_transaction_tags = transaction.tags.clone();
                self.page = Pages::EditingTransaction;
            }



            // adding transaction page signals
            Signal::AddTransaction(value_string, currency_string, date, description, tags) => {
                if Transaction::are_raw_parts_valid(&value_string, &currency_string, &description, &tags) {
                    self.bank.add_transaction_from_raw_parts(value_string, currency_string, date, description, tags);
                    self.page = Pages::Transactions;
                }
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
                self.new_transaction_selected_date = new_date;
                self.new_date_picker_mode = DatePickerModes::Hidden;
            }
            
            Signal::UpdateNewTransactionDescriptionContent(action) => {
                self.new_transaction_description_content.perform(action);
            }
            
            Signal::UpdateNewTransactionTags(new_tags) => {
                self.new_transaction_tags = new_tags;
            }



            // editing transaction page signals
            Signal::EditTransaction(value_string, currency_string, date, description, tags) => {
                if Transaction::are_raw_parts_valid(&value_string, &currency_string, &description, &tags) {
                    self.bank.edit_transaction_with_raw_parts(self.edit_transaction_id, value_string, currency_string, date, description, tags);
                    self.page = Pages::Transactions;
                }
            }
            
            Signal::StartRemovingTransaction(id) => {
                self.edit_transaction_id = id;
                self.page = Pages::RemovingTransaction;
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
                self.edit_transaction_selected_date = new_date;
                self.edit_date_picker_mode = DatePickerModes::Hidden;
            }
            
            Signal::UpdateEditTransactionDescriptionContent(action) => {
                self.edit_transaction_description_content.perform(action);
            }
            
            Signal::UpdateEditTags(new_tags) => {
                self.edit_transaction_tags = new_tags;
            }
        }
        Task::none()
    }

    /// Renders the app.
    /// Used by Iced.
    pub fn view(&self) -> Element<Signal> {
        match self.page {
            Pages::Transactions => { transactions_page(self).into() }
            Pages::AddingTransaction => { transactions_page(self).into() }
            Pages::EditingTransaction => { edit_transaction_page(self, self.edit_transaction_id).into() }
            Pages::RemovingTransaction => { transactions_page(self).into() }
            Pages::Quitting => { transactions_page(self).into() }
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
}