use iced::overlay::Element;
use iced::widget::Column;
use iced::widget::column;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::transaction_list;
use crate::vault::bank::Filters;
use crate::vault::transaction::ValueDisplayFormats;

pub fn transactions_page(app: &App) -> Column<Signal> {
    let bank = &app.bank;
    let filtered_ids = bank.get_filtered_ids(Filters::Primary);
    let transactions = filtered_ids.into_iter().map(|id| {
        bank.get(id)
    }).collect();
    
    column![
        transaction_list(app, transactions, ValueDisplayFormats::Dollars)
    ]
}