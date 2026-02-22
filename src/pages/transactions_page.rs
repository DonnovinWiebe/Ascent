use iced::Center;
use iced::overlay::Element;
use iced::widget::{space, Column};
use iced::widget::column;
use iced::widget::row;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::transaction_list;
use crate::vault::bank::Filters;
use crate::vault::transaction::ValueDisplayFormats;

pub fn transactions_page<'a, Sig: 'a + Clone>(
    app: &'a App
) -> Column<'a, Signal> {
    let bank = &app.bank;
    let filtered_ids = bank.get_filtered_ids(Filters::Primary);
    let transactions = filtered_ids.into_iter().map(|id| {
        bank.get(id)
    }).collect();

    column![
        // add title

        transaction_list::<Sig>(app, transactions, ValueDisplayFormats::Dollars),
    ]
}