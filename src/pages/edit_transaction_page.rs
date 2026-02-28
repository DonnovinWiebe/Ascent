use iced::{Center, Fill, Length};
use iced::{Color, Element, Size};
use iced::widget::*;
use iced::widget::column;
use iced::widget::row;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::{UpdateEditDatePickerMode, UpdateNewDatePickerMode};
use crate::ui::components::{panel, panel_button, standard_text, panel_text_input, text_input_style, DatePickerModes, PaddingSizes, TextSizes, TransactionManagementTypes, Widths, header};
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::transaction::{Id, Transaction, Value, ValueDisplayFormats};

// edit transaction page
pub fn edit_transaction_page(
    app: &App,
    transaction_id: Id
) -> Stack<Signal> {
    let bank = &app.bank;
    let transaction = bank.get(transaction_id);
    let mut new_value = transaction.value.clone();
    let mut new_date = transaction.date.clone();
    let mut new_description = transaction.description.clone();
    let mut new_tags = transaction.tags.clone();

    stack![
        edit_transaction_panel(app),

        header(
            app,
            true,
            Vec::new(),
        )
    ]
        .width(Fill)
        .height(Fill)
}



// components
/// A panel used to edit a transaction.
pub fn edit_transaction_panel(
    app: &App,
) -> Element<Signal> {
    container(
        panel(
            app,
            Materials::Plastic,
            MaterialColors::Background,
            2,
            true,
            Some(Widths::LargeCard),
            None,
            PaddingSizes::Medium, {
            column![
                // title
                row![
                    standard_text(app, 1, "Edit Transaction".to_string(), TextSizes::SmallHeading),
                    space::horizontal(),
                ],

                space().height(PaddingSizes::Small.size()),

                // value and date
                 row![
                    panel_text_input(
                        app,
                        Materials::RimmedPlastic,
                        MaterialColors::Background,
                        3,
                        true,
                        Widths::SmallField,
                        "Enter value...",
                        &app.edit_transaction_value_string,
                        Signal::UpdateEditValueString
                    ),

                    space().width(PaddingSizes::Nano.size()),

                    currency_picker(app, TransactionManagementTypes::Editing),

                    space::horizontal(),

                    date_picker(app, TransactionManagementTypes::Editing),
                ]
                .spacing(PaddingSizes::None.size()),
            ].into()
        })
    )
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

/// A variable date picker widget used to update the date.
pub fn date_picker(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    match transaction_management {
        TransactionManagementTypes::Adding => {
            match app.new_date_picker_mode {
                DatePickerModes::Hidden => {
                    row![
                        standard_text(app, 1, app.new_transaction_date.display(), TextSizes::Interactable),
                        space().width(PaddingSizes::Medium.size()),
                        panel_button(
                            app,
                            Materials::RimmedPlastic,
                            MaterialColors::Background,
                            3,
                            true,
                            "Edit",
                            UpdateNewDatePickerMode(DatePickerModes::ShowingDaysInMonth),
                            true,
                        ),
                    ]
                        .align_y(Center)
                        .into()
                }

                DatePickerModes::ShowingMonthsInYear => {todo!()}

                DatePickerModes::ShowingDaysInMonth => {todo!()}
            }
        }



        TransactionManagementTypes::Editing => {
            match app.edit_date_picker_mode {
                DatePickerModes::Hidden => {
                    row![
                        standard_text(app, 1, app.edit_transaction_date.display(), TextSizes::Interactable),
                        space().width(PaddingSizes::Medium.size()),
                        panel_button(
                            app,
                            Materials::RimmedPlastic,
                            MaterialColors::Background,
                            3,
                            true,
                            "Edit",
                            UpdateEditDatePickerMode(DatePickerModes::ShowingDaysInMonth),
                            true,
                        ),
                    ]
                        .align_y(Center)
                        .into()
                }

                DatePickerModes::ShowingMonthsInYear => {todo!()}

                DatePickerModes::ShowingDaysInMonth => {todo!()}
            }
        }
    }
}

/// A widget used to select a currency.
pub fn currency_picker(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let currency_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_currency_string }
        TransactionManagementTypes::Editing => { &app.edit_transaction_currency_string }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { Signal::UpdateNewCurrencyString }
        TransactionManagementTypes::Editing => { Signal::UpdateEditCurrencyString }
    };
    let is_valid = Transaction::is_currency_string_valid(currency_string);

    panel_text_input(
        app,
        Materials::RimmedPlastic,
        if is_valid { MaterialColors::Background } else { MaterialColors::Danger },
        3,
        true,
        Widths::SmallField,
        "Currency",
        currency_string,
        signal,
    )
}