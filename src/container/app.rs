use iced::{Application, Element, Task, Theme};
use iced::widget::{button, column, container, text};
use crate::container::signal::Signal;
use crate::pages::edit_transaction_page::edit_transaction_page;
use crate::pages::transactions_page::transactions_page;
use crate::ui::components::{cash_flow_panel, transaction_list, transaction_panel, DatePickerModes};
use crate::ui::palette::{AppThemes};
use crate::vault::bank::*;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Date, Id, Tag, ValueDisplayFormats};

/// The available pages in the app.
#[derive(Debug, Clone)]
pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    RemovingTransaction,
    Quitting,
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
    pub new_transaction_date: Date,
    pub new_transaction_description_string: String,
    pub new_transaction_tags: Vec<Tag>,

    // edit transaction state information
    pub edit_transaction_id: Id,
    pub edit_transaction_value_string: String,
    pub edit_transaction_currency_string: String,
    pub edit_date_picker_mode: DatePickerModes,
    pub edit_transaction_date: Date,
    pub edit_transaction_description_string: String,
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
        let launch_theme = AppThemes::Peach;
        App {
            bank,
            theme_selection: launch_theme.clone(),
            theme: launch_theme.generate(&launch_theme),
            page: Pages::Transactions,
            value_display_format: ValueDisplayFormats::Dollars,

            new_transaction_value_string: "".to_string(),
            new_transaction_currency_string: "".to_string(),
            new_date_picker_mode: DatePickerModes::Hidden,
            new_transaction_date: Date::default(),
            new_transaction_description_string: "".to_string(),
            new_transaction_tags: Vec::new(),

            edit_transaction_id: 0,
            edit_transaction_value_string: "".to_string(),
            edit_transaction_currency_string: "".to_string(),
            edit_date_picker_mode: DatePickerModes::Hidden,
            edit_transaction_date: Date::default(),
            edit_transaction_description_string: "".to_string(),
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

            Signal::Cancel(_) => {
                eprintln!("Cancelling...");
            }



            // transactions page signals
            Signal::StartAddingTransaction => {
                eprintln!("Starting adding transaction...");
                self.new_transaction_value_string = "".to_string();
                self.new_transaction_currency_string = "".to_string();
                self.new_date_picker_mode = DatePickerModes::Hidden;
                self.new_transaction_date = Date::default();
                self.new_transaction_description_string = "".to_string();
                self.new_transaction_tags = Vec::new();
            }

            Signal::StartEditingTransaction(id) => {
                let transaction = self.bank.get(id);
                self.edit_transaction_id = id;
                self.edit_transaction_value_string = transaction.value.amount().to_string();
                self.edit_transaction_currency_string = "".to_string();
                self.edit_date_picker_mode = DatePickerModes::Hidden;
                self.edit_transaction_date = transaction.date.clone();
                self.edit_transaction_description_string = transaction.description.clone();
                self.edit_transaction_tags = transaction.tags.clone();
                self.page = Pages::EditingTransaction;
            }



            // adding transaction page signals
            Signal::AddTransaction(value, date, description, tags) => {}
            
            Signal::UpdateNewValueString(new_value_string) => {}

            Signal::UpdateNewCurrencyString(new_currency_string) => {}

            Signal::UpdateNewDatePickerMode(new_mode) => {}

            Signal::GoToPreviousNewDatePickerSelectedYear => {}

            Signal::GoToNextNewDatePickerSelectedYear => {}

            Signal::UpdateNewDatePickerSelectedMonth(new_month) => {}
            
            Signal::UpdateNewDate(new_date) => {}
            
            Signal::UpdateNewDescriptionString(new_description) => {}
            
            Signal::UpdateNewTags(new_tags) => {}



            // editing transaction page signals
            Signal::EditTransaction(new_value, new_date, new_description, new_tags) => {}
            
            Signal::StartRemovingTransaction(id) => {}
            
            Signal::UpdateEditValueString(new_value_string) => {
                self.edit_transaction_value_string = new_value_string;
            }

            Signal::UpdateEditCurrencyString(new_currency_string) => {}

            Signal::UpdateEditDatePickerMode(new_mode) => {}

            Signal::GoToPreviousEditDatePickerSelectedYear => {}

            Signal::GoToNextEditDatePickerSelectedYear => {}

            Signal::UpdateEditDatePickerSelectedMonth(new_month) => {}
            
            Signal::UpdateEditDate(new_date) => {}
            
            Signal::UpdateEditDescriptionString(new_description) => {}
            
            Signal::UpdateEditTags(new_tags) => {}
        }
        Task::none()
    }

    /// Renders the app.
    /// Used by Iced.
    pub fn view(&self) -> Element<Signal> {
        match self.page {
            Pages::Transactions => { transactions_page::<Signal>(self).into() }
            Pages::AddingTransaction => { transactions_page::<Signal>(self).into() }
            Pages::EditingTransaction => { edit_transaction_page::<Signal>(self, self.edit_transaction_id).into() }
            Pages::RemovingTransaction => { transactions_page::<Signal>(self).into() }
            Pages::Quitting => { transactions_page::<Signal>(self).into() }
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
        self.theme = self.theme_selection.generate(&self.theme_selection);
    }
}