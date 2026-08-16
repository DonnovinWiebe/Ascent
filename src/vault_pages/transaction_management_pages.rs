use std::iter;
use iced::widget::text_editor::Action;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::{row, column};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text::Alignment;
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::{AddTransactionSignal, EditTransactionSignal, GeneralSignal, KeybindSignal, Signal};
use materialui::components::{ButtonShapes, DatePickerModes, Directions, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, TransactionManagementTypes, Widths, header, panel, panel_button, panel_text_editor, panel_text_input, spacer, ui_string};
use materialui::materials::{Depths, MaterialColors, MaterialStyle, Materials};
use crate::vault::transaction::{Date, Months, Tag, TagStyles, Transaction};

/// The page used for adding `Transaction`s.
#[must_use]
pub fn add_transaction_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(transaction_management_panel(app, TransactionManagementTypes::Adding)).center(Fill),
        header(app, Vec::new()),
    ]
}

/// The page used for editing `Transaction`s.
#[must_use]
pub fn edit_transaction_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(transaction_management_panel(app, TransactionManagementTypes::Editing)).center(Fill),
        header(app, Vec::new()),
    ]
}



// components
/// A panel used to edit a `Transaction`.
#[must_use]
fn transaction_management_panel<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::LargeCard, height: Heights::Shrink },
        PaddingSizes::Small, {
            column![
                // title
                row![
                    ui_string(app, match transaction_management { TransactionManagementTypes::Adding => { "Adding Transaction" } TransactionManagementTypes::Editing => { "Editing Transaction" } }, TextSizes::LargeHeading, MaterialColors::StrongText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center),
    
    
    
                // value, currency, and date
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, "Value", TextSizes::Body, MaterialColors::WeakText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    ui_string(app, "Date", TextSizes::Body, MaterialColors::WeakText),
                    spacer(Orientations::Horizontal, Spacing::Small),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
    
                row![
                    value_field(app, transaction_management),
                    spacer(Orientations::Horizontal, Spacing::Micro),
                    currency_field(app, transaction_management),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    date_picker(app, transaction_management),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
    
    
    
                // description
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, "Description", TextSizes::Body, MaterialColors::WeakText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
    
                description_editor(app, transaction_management),
    
    
    
                // tags
                spacer(Orientations::Vertical, Spacing::Medium),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, "Tags", TextSizes::Body, MaterialColors::WeakText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
    
                row![
                    current_tag_field(app, transaction_management),
                    spacer(Orientations::Horizontal, Spacing::Micro),
                    add_current_tag_button(app, transaction_management),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
    
                spacer(Orientations::Vertical, Spacing::Micro),
                editor_tag_list(app, transaction_management),
    
    
    
                // buttons
                spacer(Orientations::Vertical, Spacing::Large),
                match transaction_management {
                    TransactionManagementTypes::Adding => {
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            save_button(app, transaction_management),
                            spacer(Orientations::Horizontal, Spacing::Large),
                            cancel_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                        .align_y(Center)
                        .spacing(Spacing::None.size())
                    }
    
                    TransactionManagementTypes::Editing => {
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            save_button(app, transaction_management),
                            spacer(Orientations::Horizontal, Spacing::Large),
                            delete_button(app),
                            spacer(Orientations::Horizontal, Spacing::Large),
                            cancel_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                        .align_y(Center)
                        .spacing(Spacing::None.size())
                    }
                },
            ]
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// A widget used to enter a `Value`.
#[must_use]
fn value_field<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let value_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.get_new_transaction_state().value_string() }
        TransactionManagementTypes::Editing => { &app.get_edit_transaction_state().value_string() }
    };
    let signal = move |str| match transaction_management {
        TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionValueString(str)) }
        TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionValueString(str)) }
    };
    let is_valid = Transaction::can_parse_to_decimal(value_string);

    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if is_valid { MaterialColors::CardContent } else { MaterialColors::danger() },
            depth: Depths::Proud,
        },
        Widths::MicroField,
        "Value",
        value_string,
        signal,
        None,
        false,
    )
}

/// A widget used to enter a `Currency`.
#[must_use]
fn currency_field<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let currency_string = match transaction_management {
        TransactionManagementTypes::Adding => { &app.get_new_transaction_state().currency_string() }
        TransactionManagementTypes::Editing => { &app.get_edit_transaction_state().currency_string() }
    };
    let signal = move |str| match transaction_management {
        TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionCurrencyString(str)) }
        TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionCurrencyString(str)) }
    };
    let is_valid = Transaction::can_parse_to_currency(currency_string);

    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if is_valid { MaterialColors::CardContent } else { MaterialColors::danger() },
            depth: Depths::Proud,
        },
        Widths::MicroField,
        "Currency",
        currency_string,
        signal,
        None,
        false,
    )
}

/// A variable date picker widget used to update the `Date`.
#[must_use]
fn date_picker<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    // general information
    let mode = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().date_picker_state().mode() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().date_picker_state().mode() }
    };
    let current_year = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().date_picker_state().current_year() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().date_picker_state().current_year() }
    };
    let current_month = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().date_picker_state().current_month() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().date_picker_state().current_month() }
    };
    let selected_date = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().date_picker_state().selected_date() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().date_picker_state().selected_date() }
    };

    match mode {
        DatePickerModes::Hidden => { open_date_picker_panel(app, transaction_management, selected_date) }

        DatePickerModes::ShowingDaysInMonth => { days_in_month_panel(app, transaction_management, current_year, current_month) }

        DatePickerModes::ShowingMonthsInYear => { months_in_year_panel(app, transaction_management, current_year) }
    }
}

/// The portion of the date picker that allows the user to open it.
#[must_use]
fn open_date_picker_panel<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    selected_date: Date,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Bloated,
        ui_string(app, selected_date.display(), TextSizes::Interactable, MaterialColors::StrongText),
        match transaction_management {
            TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionDatePickerMode(DatePickerModes::ShowingDaysInMonth)) }
            TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionDatePickerMode(DatePickerModes::ShowingDaysInMonth)) }
        },
        true,
    )
}

/// The portion of the date picker that shows the days in a given month for a given year.
#[must_use]
fn days_in_month_panel<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    current_year: u32,
    current_month: Months,
) -> Element<'a, Signal> {
    // days in month information
    let days_in_current_month = current_month.days_in_month(current_year);
    let days_per_row: u32 = 8;
    let rows: u32 = days_in_current_month / days_per_row + 1;
    let days_in_last_row: u32 = days_in_current_month % days_per_row;
    
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollow,
            depth: Depths::Recessed,
        },
        PanelSize { width: Widths::SmallCard, height: Heights::Shrink },
        PaddingSizes::Medium, {
            let parts = (0..rows).map(|row_index| {
                if row_index < rows - 1 {
                    let mut buttons: Vec<_> = (1..=days_per_row).map(|day| {
                        date_picker_day_button(app, transaction_management, current_year, current_month, (row_index * days_per_row) + day)
                    }).collect();
                    buttons.insert(0, spacer(Orientations::Horizontal, Spacing::Fill));
                    buttons.push(spacer(Orientations::Horizontal, Spacing::Fill));

                    row(buttons)
                        .spacing(Spacing::None.size())
                        .into()
                }
                else {
                    let mut buttons: Vec<_> = (1..=days_in_last_row).map(|day| {
                        date_picker_day_button(app, transaction_management, current_year, current_month, (row_index * days_per_row) + day)
                    }).collect();
                    buttons.insert(0, spacer(Orientations::Horizontal, Spacing::Fill));
                    buttons.push(spacer(Orientations::Horizontal, Spacing::Fill));

                    row(buttons)
                        .spacing(Spacing::None.size())
                        .into()
                }
            });

            column(iter::once(date_picker_change_month_and_year_button(app, transaction_management, current_year, current_month)).chain(iter::once(spacer(Orientations::Vertical, Spacing::Medium))).chain(parts))
                .spacing(Spacing::None.size())
                .align_x(Center)
                .into()
        }
    )
}

/// The portion of the date picker that shows the months in a year.
#[must_use]
fn months_in_year_panel<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    current_year: u32,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Background,
            depth: Depths::Recessed,
        },
        PanelSize { width: Widths::SmallCard, height: Heights::Shrink },
        PaddingSizes::Medium, {
            column![
                // changing the year
                row![
                    date_picker_change_year_button(app, transaction_management, Directions::Recede),
                    ui_string(app, current_year.to_string(), TextSizes::Interactable, MaterialColors::StrongText),
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
                    .spacing(Spacing::None.size())
                    .align_x(Alignment::Left),

                    spacer(Orientations::Horizontal, Spacing::Fill),
                    column![
                        date_picker_month_button(app, transaction_management, Months::February),
                        date_picker_month_button(app, transaction_management, Months::May),
                        date_picker_month_button(app, transaction_management, Months::August),
                        date_picker_month_button(app, transaction_management, Months::November),
                    ]
                    .spacing(Spacing::None.size())
                    .align_x(Alignment::Center),

                    spacer(Orientations::Horizontal, Spacing::Fill),
                    column![
                        date_picker_month_button(app, transaction_management, Months::March),
                        date_picker_month_button(app, transaction_management, Months::June),
                        date_picker_month_button(app, transaction_management, Months::September),
                        date_picker_month_button(app, transaction_management, Months::December),
                    ]
                    .spacing(Spacing::None.size())
                    .align_x(Alignment::Right),
                ]
            ]
            .spacing(Spacing::None.size())
            .align_x(Center)
            .into()
        }
    )
}

/// The button used to set a specific `Date` with the date picker.
#[must_use]
fn date_picker_day_button<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    year: u32,
    month: Months,
    day: u32,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud,
        },
        ButtonShapes::LowProfile,
        ui_string(app, day.to_string(), TextSizes::Body, MaterialColors::StrongText),
        match transaction_management {
            TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionSelectedDate(Date::new(year, month, day))) }
            TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionSelectedDate(Date::new(year, month, day))) }
        },
        true,
    )
}

/// The button used to start changing the month and year of the date picker.
#[must_use]
fn date_picker_change_month_and_year_button<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    year: u32,
    month: Months,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Standard,
        ui_string(app, format!("{}, {}", month.display(), year), TextSizes::Interactable, MaterialColors::StrongText),
        match transaction_management {
            TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionDatePickerMode(DatePickerModes::ShowingMonthsInYear)) }
            TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionDatePickerMode(DatePickerModes::ShowingMonthsInYear)) }
        },
        true,
    )
}

/// The button used to set the month of the date picker.
#[must_use]
pub fn date_picker_month_button<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    month: Months,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Bloated,
        ui_string(app, month.display(), TextSizes::Body, MaterialColors::StrongText),
        match transaction_management {
            TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionCurrentMonth(month)) }
            TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionCurrentMonth(month)) }
        },
        true,
    )
}

/// The button used to advance or recede the year of the date picker.
#[must_use]
fn date_picker_change_year_button<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    direction: Directions,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Bloated,
        ui_string(app, match direction { Directions::Advance => { ">" } Directions::Recede => { "<" } }, TextSizes::Interactable, MaterialColors::StrongText),
        match transaction_management {
            TransactionManagementTypes::Adding => { match direction {
                Directions::Advance => { Signal::AddTransactionSignal(AddTransactionSignal::AdvanceNewTransactionCurrentYear) }
                Directions::Recede => { Signal::AddTransactionSignal(AddTransactionSignal::RecedeNewTransactionCurrentYear) }
            } }
            TransactionManagementTypes::Editing => { match direction {
                Directions::Advance => { Signal::EditTransactionSignal(EditTransactionSignal::AdvanceEditTransactionCurrentYear) }
                Directions::Recede => { Signal::EditTransactionSignal(EditTransactionSignal::RecedeEditTransactionCurrentYear) }
            } }
        },
        true,
    )
}

/// The field used to edit the `Transaction` `description`.
#[must_use]
fn description_editor<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let description_content = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().description_content() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().description_content() }
    };
    let signal: fn(action: Action) -> Signal = match transaction_management {
        TransactionManagementTypes::Adding => signal_adding,
        TransactionManagementTypes::Editing => signal_editing,
    };
    fn signal_adding(action: Action) -> Signal {
        Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionDescriptionContent(action))
    }
    fn signal_editing(action: Action) -> Signal {
        Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionDescriptionContent(action))
    }
    let is_valid = Transaction::is_description_valid(&description_content.text());

    panel_text_editor(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if is_valid { MaterialColors::CardContent } else { MaterialColors::danger() },
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::LargeField, height: Heights::Shrink },
        description_content,
        signal,
    )
}

/// Edits the current `Tag`.
#[must_use]
fn current_tag_field<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let tag_string = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().current_tag_string() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().current_tag_string() }
    };
    let update_signal = move |str| match transaction_management {
        TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::UpdateNewTransactionCurrentTagString(str)) }
        TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::UpdateEditTransactionCurrentTagString(str)) }
    };
    let (submit_signal, is_valid) = add_current_tag_signal(app, transaction_management);

    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if is_valid { MaterialColors::success() } else { MaterialColors::CardContent },
            depth: Depths::Proud,
        },
        Widths::SmallField,
        "New Tag",
        tag_string,
        update_signal,
        Some(submit_signal),
        is_valid,
    )
}

/// Adds the current `Tag` for editing.
#[must_use]
fn add_current_tag_button<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let (signal, is_valid) = add_current_tag_signal(app, transaction_management);
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud,
        },
        ButtonShapes::Bloated,
        icon("plus"),
        signal,
        is_valid,
    )
}

/// Returns the appropriate `Signal` to add the current `Tag` and a flag indicating if the `Tag` is valid.
#[must_use]
fn add_current_tag_signal(
    app: &App,
    transaction_management: TransactionManagementTypes,
) -> (Signal, bool) {
    let tag_string = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().current_tag_string() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().current_tag_string() }
    };
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::AddNewTransactionTag(tag_string.to_string())) }
        TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::AddEditTransactionTag(tag_string.to_string())) }
    };
    let is_valid = Tag::is_allowed(tag_string);
    
    (signal, is_valid)
}

/// Displays the `Tag`s in a `Transaction` for editing.
#[must_use]
fn editor_tag_list<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let tags = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().tags() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().tags() }
    };
    let not_empty = !tags.is_empty();

    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollow,
            depth: Depths::Recessed,
        },
        PanelSize { width: Widths::LargeField, height: Heights::Shrink },
        PaddingSizes::None, {
            column![
                spacer(Orientations::Vertical, Spacing::Micro),

                scrollable(
                    row({
                        let mut tag_panels: Vec<_> = tags.into_iter().map(|tag| {
                            editor_tag_panel(app, transaction_management, &tag)
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
                                    MaterialStyle {
                                        material: Materials::Plastic,
                                        color: MaterialColors::danger(),
                                        depth: Depths::Proud,
                                    },
                                    PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                                    PaddingSizes::Small, {
                                        ui_string(app, "Tags cannot be empty!", TextSizes::Interactable, MaterialColors::StrongText)
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

/// Displays a `Tag` for editing.
#[must_use]
fn editor_tag_panel<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
    tag: &Tag,
) -> Element<'a, Signal> {
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::RemoveNewTransactionTag(tag.clone())) }
        TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::RemoveEditTransactionTag(tag.clone())) }
    };

    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::Shrink, height: Heights::Shrink },
        PaddingSizes::None, {
            row![
                ui_string(app, tag.display(TagStyles::Lowercase), TextSizes::Interactable, MaterialColors::StrongText),
                spacer(Orientations::Horizontal, Spacing::Micro),
                panel_button(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::danger(),
                        depth: Depths::Proud,
                    },
                    ButtonShapes::LowProfile,
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

/// Saves the `Transaction`.
#[must_use]
fn save_button<'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let signal = match transaction_management {
        TransactionManagementTypes::Adding => { Signal::AddTransactionSignal(AddTransactionSignal::AddTransaction) }
        TransactionManagementTypes::Editing => { Signal::EditTransactionSignal(EditTransactionSignal::EditTransaction) }
    };
    let value_string = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().value_string() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().value_string() }
    };
    let currency_string = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().currency_string() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().currency_string() }
    };
    let description = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().description_content().text() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().description_content().text() }
    };
    let tags = match transaction_management {
        TransactionManagementTypes::Adding => { app.get_new_transaction_state().tags() }
        TransactionManagementTypes::Editing => { app.get_edit_transaction_state().tags() }
    };
    let is_valid = Transaction::are_raw_parts_valid(value_string, currency_string, &description, &tags);

    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        icon("check"),
        signal,
        is_valid,
    )
}

/// Cancels.
#[must_use]
fn cancel_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        icon("xmark"),
        Signal::GeneralSignal(GeneralSignal::GoHome),
        true,
    )
}

/// Deletes a `Transaction`.
#[must_use]
fn delete_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    let is_primed = app.get_edit_transaction_state().is_delete_primed();

    if is_primed {
        row![
            panel_button(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::danger(),
                    depth: Depths::Proud,
                },
                ButtonShapes::Bloated,
                icon("trash"),
                Signal::EditTransactionSignal(EditTransactionSignal::RemoveTransaction),
                true,
            ),
            spacer(Orientations::Horizontal, Spacing::Micro),
            panel_button(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::CardContent,
                    depth: Depths::Proud,
                },
                ButtonShapes::Bloated,
                icon("xmark"),
                Signal::EditTransactionSignal(EditTransactionSignal::UnprimeRemoveTransaction),
                true,
            ),
        ]
        .spacing(Spacing::None.size())
        .align_y(Center)
        .into()
    }

    else {
        row![
            panel_button(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::danger(),
                    depth: Depths::Proud,
                },
                ButtonShapes::Wide,
                icon("trash"),
                Signal::EditTransactionSignal(EditTransactionSignal::PrimeRemoveTransaction),
                true,
            ),
        ]
        .spacing(Spacing::None.size())
        .align_y(Center)
        .into()
    }
}