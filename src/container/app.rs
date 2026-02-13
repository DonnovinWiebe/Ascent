use iced::{Element, Task};
use iced::widget::{button, column, container, text};
use crate::container::signal::Signal;
use crate::ui::components::cash_flow_pane;
use crate::ui::palette::ColorThemes;
use crate::vault::bank::*;
use crate::vault::parse::CashFlowGrouping;
use crate::vault::transaction::ValueDisplayFormats;

#[derive(Debug, Clone)]
pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    RemovingTransaction,
    Quitting,
}



pub struct App {
    // basics
    pub bank: Bank,
    // app state
    theme: ColorThemes,
    // bank state
    value_display_format: ValueDisplayFormats,
}
impl Default for App {
    fn default() -> Self {
        Self::new_app()
    }
}
impl App {
    // initializing
    pub fn new(_flags: ()) -> (App, Task<Signal>) {
        (App::new_app(), Task::none())
    }

    fn new_app() -> App {
        let mut bank = Bank::new();
        bank.init();

        App {
            bank,
            theme: ColorThemes::Dark,
            value_display_format: ValueDisplayFormats::Dollars,
        }
    }

    pub fn title(&self) -> String {
        "Ascent".to_string()
    }

    // running
    pub(crate) fn update(&mut self, signal: Signal) -> Task<Signal> {
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

    pub(crate) fn view(&self) -> Element<Signal> {
        container(
            column![
                cash_flow_pane(CashFlowGrouping::new(self.bank.primary_filter.get_filtered_ids(), &self.bank), ValueDisplayFormats::Dollars),
            ]
                .spacing(20)
                .padding(20)
        )
            .center(iced::Fill)
            .into()
    }
}