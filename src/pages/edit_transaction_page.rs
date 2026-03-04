use std::iter;
use iced::{Center, Fill, Length};
use iced::{Color, Element, Size};
use iced::widget::*;
use iced::widget::{row, column};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text::Alignment;
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::{Signal, Signal::*};
use crate::pages::transactions_page::transaction_panel;
use crate::ui::components::*;
use crate::ui::material::*;
use crate::vault::transaction::*;
use crate::vault::transaction::Id;

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
            Widths::LargeCard,
            Heights::Shrink,
            PaddingSizes::Large, {
            column![
                // title
                row![
                    ui_string(app, 1, "Edit Transaction".to_string(), TextSizes::LargeHeading),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center),



                // value, currency, and date
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, 2, "Value".to_string(), TextSizes::Body),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    ui_string(app, 2, "Date".to_string(), TextSizes::Body),
                    spacer(Orientations::Horizontal, Spacing::Small),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),

                row![
                    value_field(app, TransactionManagementTypes::Editing),
                    spacer(Orientations::Horizontal, Spacing::Micro),
                    currency_field(app, TransactionManagementTypes::Editing),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    date_picker(app, TransactionManagementTypes::Editing),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),



                // description
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, 2, "Description".to_string(), TextSizes::Body),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),

                description_editor(app, TransactionManagementTypes::Editing),



                // tags
                spacer(Orientations::Vertical, Spacing::Medium),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, 2, "Tags".to_string(), TextSizes::Body),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),

                row![
                    current_tag_field(app, TransactionManagementTypes::Editing),
                    spacer(Orientations::Horizontal, Spacing::Micro),
                    add_current_tag_button(app, TransactionManagementTypes::Editing),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),

                spacer(Orientations::Vertical, Spacing::Micro),
                editor_tag_list(app, TransactionManagementTypes::Editing),
            ]
                .spacing(Spacing::None.size())
                .into()
        })
    )
        .center_x(Fill)
        .center_y(Fill)
        .into()
}

/// A widget used to select a currency.
pub fn value_field(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let value_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_value_string }
        TransactionManagementTypes::Editing => { &app.edit_transaction_value_string }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { Signal::UpdateNewTransactionValueString }
        TransactionManagementTypes::Editing => { Signal::UpdateEditTransactionValueString }
    };
    let is_valid = Transaction::is_value_string_valid(value_string);

    panel_text_input(
        app,
        Materials::RimmedPlastic,
        if is_valid { MaterialColors::Background } else { MaterialColors::Danger },
        3,
        true,
        Widths::MicroField,
        "Value",
        value_string,
        signal,
    )
}

/// A widget used to select a currency.
pub fn currency_field(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let currency_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_currency_string }
        TransactionManagementTypes::Editing => { &app.edit_transaction_currency_string }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { Signal::UpdateNewTransactionCurrencyString }
        TransactionManagementTypes::Editing => { Signal::UpdateEditTransactionCurrencyString }
    };
    let is_valid = Transaction::is_currency_string_valid(currency_string);

    panel_text_input(
        app,
        Materials::RimmedPlastic,
        if is_valid { MaterialColors::Background } else { MaterialColors::Danger },
        3,
        true,
        Widths::MicroField,
        "Currency",
        currency_string,
        signal,
    )
}

/// A variable date picker widget used to update the date.
pub fn date_picker(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    // general information
    let mode = match transaction_management {
        TransactionManagementTypes::Adding => { app.new_date_picker_mode }
        TransactionManagementTypes::Editing => { app.edit_date_picker_mode }
    };
    let current_year = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_current_year }
        TransactionManagementTypes::Editing => { &app.edit_transaction_current_year }
    };
    let current_month = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_current_month }
        TransactionManagementTypes::Editing => { &app.edit_transaction_current_month }
    };
    let selected_date = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_selected_date }
        TransactionManagementTypes::Editing => { &app.edit_transaction_selected_date }
    };

    // days in month information
    let days_in_current_month = current_month.days_in_month(*current_year);
    let days_per_row: u32 = 6;
    let rows: u32 = days_in_current_month / days_per_row + 1;
    let days_in_last_row: u32 = days_in_current_month % days_per_row;

    match mode {
        DatePickerModes::Hidden => {
            panel_button(
                app,
                Materials::RimmedPlastic,
                MaterialColors::Background,
                3,
                true,
                ButtonShapes::Bloated,
                ui_string(app, 1, selected_date.display(), TextSizes::Interactable),
                match transaction_management {
                    TransactionManagementTypes::Adding => { UpdateNewTransactionDatePickerMode(DatePickerModes::ShowingDaysInMonth) }
                    TransactionManagementTypes::Editing => { UpdateEditTransactionDatePickerMode(DatePickerModes::ShowingDaysInMonth) }
                },
                true,
            )
        }

        DatePickerModes::ShowingDaysInMonth => {
            panel(
                app,
                Materials::Plastic,
                MaterialColors::Background,
                3,
                true,
                Widths::SmallCard,
                Heights::Shrink,
                PaddingSizes::Medium, {
                    let parts = (0..rows).into_iter().map(|row_index| {
                        if row_index < rows - 1 {
                            let buttons: Vec<_> = (1..=days_per_row).into_iter().map(|day| {
                                date_picker_day_button(app, transaction_management, *current_year, *current_month, (row_index * days_per_row) + day)
                            }).collect();
                            row(buttons).into()
                        }
                        else {
                            let buttons: Vec<_> = (1..=days_in_last_row).into_iter().map(|day| {
                                date_picker_day_button(app, transaction_management, *current_year, *current_month, (row_index * days_per_row) + day)
                            }).collect();
                            row(buttons).into()
                        }
                    });

                    column(iter::once(date_picker_change_month_and_year_button(app, transaction_management, *current_year, *current_month)).chain(iter::once(spacer(Orientations::Vertical, Spacing::Medium))).chain(parts))
                        .spacing(Spacing::None.size())
                        .align_x(Center)
                        .into()
                }
            )
        }

        DatePickerModes::ShowingMonthsInYear => {
            panel(
                app,
                Materials::Plastic,
                MaterialColors::Background,
                3,
                true,
                Widths::SmallCard,
                Heights::Shrink,
                PaddingSizes::Medium, {
                    column![
                        // changing the year
                        row![
                            date_picker_change_year_button(app, transaction_management, Directions::Recede),
                            ui_string(app, 1, current_year.to_string(), TextSizes::Interactable),
                            date_picker_change_year_button(app, transaction_management, Directions::Advance),
                        ]
                        .spacing(Spacing::Medium.size())
                        .align_y(Center),

                        // changing the month
                        spacer(Orientations::Vertical, Spacing::Medium),
                        row![
                            column![
                                date_picker_month_button(app, transaction_management, Months::January),
                                date_picker_month_button(app, transaction_management, Months::April),
                                date_picker_month_button(app, transaction_management, Months::July),
                                date_picker_month_button(app, transaction_management, Months::October),
                            ]
                            .spacing(Spacing::Micro.size())
                            .align_x(Alignment::Left),

                            spacer(Orientations::Horizontal, Spacing::Fill),
                            column![
                                date_picker_month_button(app, transaction_management, Months::February),
                                date_picker_month_button(app, transaction_management, Months::May),
                                date_picker_month_button(app, transaction_management, Months::August),
                                date_picker_month_button(app, transaction_management, Months::November),
                            ]
                            .spacing(Spacing::Micro.size())
                            .align_x(Alignment::Center),

                            spacer(Orientations::Horizontal, Spacing::Fill),
                            column![
                                date_picker_month_button(app, transaction_management, Months::March),
                                date_picker_month_button(app, transaction_management, Months::June),
                                date_picker_month_button(app, transaction_management, Months::September),
                                date_picker_month_button(app, transaction_management, Months::December),
                            ]
                            .spacing(Spacing::Micro.size())
                            .align_x(Alignment::Right),
                        ]
                    ]
                        .spacing(Spacing::None.size())
                        .align_x(Center)
                        .into()
                }
            )
        }
    }
}

/// The button used to set a specific date with the date picker.
pub fn date_picker_day_button(
    app: &App,
    transaction_management: TransactionManagementTypes,
    year: u32,
    month: Months,
    day: u32,
) -> Element<Signal> {
    container(
        panel_button(
            app,
            Materials::RimmedPlastic,
            MaterialColors::Background,
            4,
            true,
            ButtonShapes::Bloated,
            ui_string(app, 1, day.to_string(), TextSizes::Body),
            match transaction_management {
                TransactionManagementTypes::Adding => { UpdateNewTransactionSelectedDate(Date::new(year, month, day)) }
                TransactionManagementTypes::Editing => { UpdateEditTransactionSelectedDate(Date::new(year, month, day)) }
            },
            true,
        )
    )
        .width(Fill)
        .center_x(Fill)
        .into()
}

/// The button used to start changing the month and year of the date picker.
pub fn date_picker_change_month_and_year_button(
    app: &App,
    transaction_management: TransactionManagementTypes,
    year: u32,
    month: Months,
) -> Element<Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        4,
        true,
        ButtonShapes::Standard,
        ui_string(app, 1, format!("{}, {}", month.display(), year.to_string()), TextSizes::Interactable),
        match transaction_management {
            TransactionManagementTypes::Adding => { UpdateNewTransactionDatePickerMode(DatePickerModes::ShowingMonthsInYear) }
            TransactionManagementTypes::Editing => { UpdateEditTransactionDatePickerMode(DatePickerModes::ShowingMonthsInYear) }
        },
        true,
    )
}

/// The button used to set the month of the date picker.
pub fn date_picker_month_button(
    app: &App,
    transaction_management: TransactionManagementTypes,
    month: Months,
) -> Element<Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        4,
        true,
        ButtonShapes::Bloated,
        ui_string(app, 1, month.display(), TextSizes::Body),
        match transaction_management {
            TransactionManagementTypes::Adding => { UpdateNewTransactionCurrentMonth(month) }
            TransactionManagementTypes::Editing => { UpdateEditTransactionCurrentMonth(month) }
        },
        true,
    )
}

/// The button used to advance or recede the year of the date picker.
pub fn date_picker_change_year_button(
    app: &App,
    transaction_management: TransactionManagementTypes,
    direction: Directions,
) -> Element<Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        4,
        true,
        ButtonShapes::Bloated,
        ui_string(app, 1, match direction { Directions::Advance => { ">".to_string() } Directions::Recede => { "<".to_string() } }, TextSizes::Interactable),
        match transaction_management {
            TransactionManagementTypes::Adding => { match direction {
                Directions::Advance => { AdvanceNewTransactionCurrentYear }
                Directions::Recede => { RecedeNewTransactionCurrentYear }
            } }
            TransactionManagementTypes::Editing => { match direction {
                Directions::Advance => { AdvanceEditTransactionCurrentYear }
                Directions::Recede => { RecedeEditTransactionCurrentYear }
            } }
        },
        true,
    )
}

/// The field used to edit the transaction description.
pub fn description_editor(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let description_content = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_description_content }
        TransactionManagementTypes::Editing => { &app.edit_transaction_description_content }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { UpdateNewTransactionDescriptionContent }
        TransactionManagementTypes::Editing => { UpdateEditTransactionDescriptionContent }
    };
    let is_valid = Transaction::is_description_valid(&description_content.text());

    panel_text_editor(
        app,
        Materials::RimmedPlastic,
        if is_valid { MaterialColors::Background } else { MaterialColors::Danger },
        3,
        true,
        Widths::LargeField,
        Heights::Shrink,
        description_content,
        signal,
    )
}

/// Edits the current tag.
pub fn current_tag_field(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let tag_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_current_tag_string }
        TransactionManagementTypes::Editing => { &app.edit_transaction_current_tag_string }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { UpdateNewTransactionCurrentTagString }
        TransactionManagementTypes::Editing => { UpdateEditTransactionCurrentTagString }
    };
    let is_valid = Tag::is_allowed(tag_string);

    panel_text_input(
        app,
        Materials::RimmedPlastic,
        if is_valid { MaterialColors::Background } else { MaterialColors::Unavailable },
        3,
        true,
        Widths::SmallField,
        "New Tag",
        tag_string,
        signal,
    )
}

/// Adds the current tag for editing.
pub fn add_current_tag_button(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let tag_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.new_transaction_current_tag_string }
        TransactionManagementTypes::Editing => { &app.edit_transaction_current_tag_string }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { AddNewTransactionTag(tag_string.clone()) }
        TransactionManagementTypes::Editing => { AddEditTransactionTag(tag_string.clone()) }
    };
    let is_valid = Tag::is_allowed(tag_string);

    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Success,
        4,
        true,
        ButtonShapes::Bloated,
        icon("plus"),
        signal,
        is_valid,
    )
}

/// Displays the tags in a transaction for editing.
pub fn editor_tag_list(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> Element<Signal> {
    let tags = match transaction_management {
        TransactionManagementTypes::Adding => { app.new_transaction_tags.clone() }
        TransactionManagementTypes::Editing => { app.edit_transaction_tags.clone() }
    };
    let not_empty = !tags.is_empty();

    panel(
        app,
        Materials::Plastic,
        MaterialColors::Background,
        1,
        false,
        Widths::LargeField,
        Heights::Shrink,
        PaddingSizes::None, {
            column![
                spacer(Orientations::Vertical, Spacing::Micro),

                scrollable(
                    row({
                        let mut tag_panels: Vec<_> = tags.into_iter().map(|tag| {
                            editor_tag_panel(app, transaction_management, tag)
                        }).collect();
                        tag_panels.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
                        tag_panels.push(spacer(Orientations::Horizontal, Spacing::Small));

                        if not_empty {
                            tag_panels
                        }
                        else {
                            vec![
                                spacer(Orientations::Horizontal, Spacing::Small),
                                panel(
                                    app,
                                    Materials::Acrylic,
                                    MaterialColors::Danger,
                                    3,
                                    true,
                                    Widths::Shrink,
                                    Heights::Shrink,
                                    PaddingSizes::Small, {
                                        ui_string(app, 1, "Tags cannot be empty!".to_string(), TextSizes::Interactable)
                                    }
                                ),
                                spacer(Orientations::Horizontal, Spacing::Small),
                            ]
                        }
                    })
                    .spacing(PaddingSizes::Nano.size()),
                )
                .direction(Direction::Horizontal(Scrollbar::hidden())),

                spacer(Orientations::Vertical, Spacing::Micro),
            ]
                .align_x(Center)
                .into()
        }
    )
}

/// Displays a tag for editing.
pub fn editor_tag_panel(
    app: &App,
    transaction_management: TransactionManagementTypes,
    tag: Tag,
) -> Element<Signal> {
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { RemoveNewTransactionTag(tag.clone()) }
        TransactionManagementTypes::Editing => { RemoveEditTransactionTag(tag.clone()) }
    };

    panel(
        app,
        Materials::Plastic,
        MaterialColors::Background,
        2,
        true,
        Widths::Shrink,
        Heights::Shrink,
        PaddingSizes::None, {
            row![
                ui_string(app, 1, tag.display(TagStyles::Lowercase), TextSizes::Interactable),
                spacer(Orientations::Horizontal, Spacing::Micro),
                panel_button(
                    app,
                    Materials::RimmedPlastic,
                    MaterialColors::Danger,
                    3,
                    true,
                    ButtonShapes::Minimal,
                    icon("trash"),
                    signal,
                    true,
                )
            ]
                .spacing(Spacing::None.size())
                .align_y(Center)
                .padding([PaddingSizes::Nano.size(), PaddingSizes::Small.size()])
                .into()
        }
    )
}