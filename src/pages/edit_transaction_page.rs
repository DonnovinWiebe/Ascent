use iced::overlay::Element;
use iced::widget::*;
use iced::widget::column;
use iced::widget::row;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{panel, standard_text, text_input_style, transaction_list, PaddingSizes, TextSizes};
use crate::ui::palette::AppColors;
use crate::vault::bank::Filters;
use crate::vault::transaction::{Id, Value, ValueDisplayFormats};

pub fn edit_transaction_page(app: &App, transaction_id: Id) -> Column<Signal> {
    let bank = &app.bank;
    let transaction = bank.get(transaction_id);
    let mut new_value = transaction.value.clone();
    let mut new_date = transaction.date.clone();
    let mut new_description = transaction.description.clone();
    let mut new_tags = transaction.tags.clone();

    column![
        panel(
            app,
            AppColors::Midground,
            true,
            PaddingSizes::Medium, {
                column![
                    standard_text(
                        app,
                        TextSizes::Interactable,
                        "Implement".to_string()
                    ),
                    text_input("Enter value...", &app.edit_transaction_value_string)
                    .style(text_input_style(app, AppColors::Foreground))
                    .on_input(Signal::UpdateEditValueString),
                ].into()
            }
        )
    ]
}