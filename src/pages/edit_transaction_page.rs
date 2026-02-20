use iced::overlay::Element;
use iced::widget::Column;
use iced::widget::column;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{panel, standard_text, transaction_list, PaddingSizes, StylingColors, TextSizes, TextStylingColors};
use crate::vault::bank::Filters;
use crate::vault::transaction::{Id, Value, ValueDisplayFormats};

pub fn edit_transaction_page<'a>(app: &App, transaction_id: Id) -> Column<'a, Signal> {
    let bank = &app.bank;
    let transaction = bank.get(transaction_id);
    let mut new_value = transaction.value.clone();
    let mut new_date = transaction.date.clone();
    let mut new_description = transaction.description.clone();
    let mut new_tags = transaction.tags.clone();

    column![
        panel(
            StylingColors::WeakBackground,
            PaddingSizes::Medium, {
                column![
                    standard_text(
                        TextSizes::Interactable,
                        StylingColors::WeakBackground,
                        "Implement".to_string()
                    )
                ].into()
            }
        )
    ]
}