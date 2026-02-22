use iced::{Fill, Length};
use iced::overlay::Element;
use iced::widget::*;
use iced::widget::column;
use iced::widget::row;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{date_picker, panel, standard_text, text_input_style, transaction_list, PaddingSizes, TextSizes, TransactionManagementTypes, Widths};
use crate::ui::palette::AppColors;
use crate::vault::bank::Filters;
use crate::vault::transaction::{Id, Value, ValueDisplayFormats};

pub fn edit_transaction_page<'a, Sig: 'a + Clone>(
    app: &'a App,
    transaction_id: Id
) -> Column<'a, Signal> {
    let bank = &app.bank;
    let transaction = bank.get(transaction_id);
    let mut new_value = transaction.value.clone();
    let mut new_date = transaction.date.clone();
    let mut new_description = transaction.description.clone();
    let mut new_tags = transaction.tags.clone();

    column![
        container(
            panel::<Sig>(app, AppColors::Midground, true, PaddingSizes::Medium, Some(Widths::LargeCard), None, {
                column![
                    // title
                    row![
                        standard_text::<Sig>(app, TextSizes::SmallHeading, "Edit Transaction".to_string()),
                        space::horizontal(),
                    ],

                    space().height(PaddingSizes::Small.size()),

                    // value and date
                    row![
                        text_input("Enter value...", &app.edit_transaction_value_string)
                        .style(text_input_style(app, AppColors::Foreground))
                        .width(Widths::SmallField.size())
                        .on_input(Signal::UpdateEditValueString),

                        space().width(PaddingSizes::Medium.size()),

                        date_picker::<Sig>(app, TransactionManagementTypes::Editing),
                    ],
                ].into()
            })
        )
        .center_x(Fill)
        .center_y(Fill),
    ].into()
}