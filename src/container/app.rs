use iced::{Application, Element, Task, Theme};
use iced::widget::{button, column, container, text};
use crate::container::signal::Signal;
use crate::pages::transactions_page::transactions_page;
use crate::ui::components::{cash_flow_panel, transaction_list, transaction_panel};
use crate::ui::palette::{AppThemes};
use crate::vault::bank::*;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Date, Tag, ValueDisplayFormats};

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
    // bank display state
    value_display_format: ValueDisplayFormats,

    // new transaction state information
    pub new_transaction_value_string: String,
    pub new_transaction_date: Date,
    pub new_transaction_description_string: String,
    pub new_transaction_tags: Vec<Tag>,

    // edit transaction state information
    pub edit_transaction_value_string: String,
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
        let launch_theme = AppThemes::Midnight;
        App {
            bank,
            theme_selection: launch_theme.clone(),
            theme: launch_theme.generate(&launch_theme),
            value_display_format: ValueDisplayFormats::Dollars,

            new_transaction_value_string: "".to_string(),
            new_transaction_date: Date::default(),
            new_transaction_description_string: "".to_string(),
            new_transaction_tags: Vec::new(),
            edit_transaction_value_string: "".to_string(),
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
                self.new_transaction_date = Date::default();
                self.new_transaction_description_string = "".to_string();
                self.new_transaction_tags = Vec::new();
            }

            Signal::StartEditingTransaction(_) => {
                eprintln!("Starting editing transaction...");
                self.edit_transaction_value_string = "".to_string();
                self.edit_transaction_date = Date::default();
                self.edit_transaction_description_string = "".to_string();
                self.edit_transaction_tags = Vec::new();
            }



            // adding transaction page signals
            Signal::AddTransaction(_, _, _, _) => {
                eprintln!("Adding transaction...");
            }
            
            Signal::UpdateNewValueString(_) => {
                eprintln!("Updating new value...");
            }
            
            Signal::UpdateNewDate(_) => {
                eprintln!("Updating new date...");
            }
            
            Signal::UpdateNewDescriptionString(_) => {
                eprintln!("Updating new descriptions...");
            }
            
            Signal::UpdateNewTags(_) => {
                eprintln!("Updating new tags...");
            }



            // editing transaction page signals
            Signal::EditTransaction(_, _, _, _) => {
                eprintln!("Editing transaction...");
            }
            
            Signal::StartRemovingTransaction(_) => {
                eprintln!("Starting removing transaction...");
            }
            
            Signal::UpdateEditValueString(_) => {
                eprintln!("Updating edit value...");
            }
            
            Signal::UpdateEditDate(_) => {
                eprintln!("Updating edit date...");
            }
            
            Signal::UpdateEditDescriptionString(_) => {
                eprintln!("Updating edit description...");
            }
            
            Signal::UpdateEditTags(_) => {
                eprintln!("Updating edit tags...");
            }
        }
        Task::none()
    }

    /// Renders the app.
    /// Used by Iced.
    pub fn view(&self) -> Element<Signal> {
        transactions_page(self).into()
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