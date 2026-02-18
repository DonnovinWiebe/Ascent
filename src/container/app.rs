use iced::{Application, Element, Task, Theme};
use iced::widget::{button, column, container, text};
use crate::container::signal::Signal;
use crate::ui::components::{cash_flow_panel, transaction_list, transaction_panel};
use crate::ui::palette::{ColorModes, Themes};
use crate::vault::bank::*;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::ValueDisplayFormats;

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
    pub theme_selection: Themes,
    theme: Theme,
    // bank state
    value_display_format: ValueDisplayFormats,
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
        App {
            bank,
            theme_selection: Themes::Peach,
            theme: Themes::Peach.generate(),
            value_display_format: ValueDisplayFormats::Dollars,
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
            Signal::InvalidAction(_) => {}
            Signal::StartAddingTransaction => {}
            Signal::StartEditingTransaction(_) => {}
            Signal::StartRemovingTransaction(_) => {}
            Signal::Cancel(_) => {}
            Signal::AddTransaction(_, _, _, _) => {}
            Signal::EditTransaction(_, _, _, _) => {}
        }
        Task::none()
    }

    /// Renders the app.
    /// Used by Iced.
    pub fn view(&self) -> Element<Signal> {
        container(
            column![
                cash_flow_panel(&CashFlow::new(self.bank.primary_filter.get_filtered_ids(), &self.bank), ValueDisplayFormats::Time(25.0)),
                transaction_list(self, self.bank.ledger(), ValueDisplayFormats::Dollars),
            ]
                .spacing(20)
                .padding(20)
        )
            .center(iced::Fill)
            .into()
    }

    /// Gets the current theme.
    /// Used by Iced
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// Updates the theme of the app.
    pub fn update_theme(&mut self, new_theme_selection: Themes) {
        self.theme_selection = new_theme_selection;
        self.theme = self.theme_selection.generate();
    }
}