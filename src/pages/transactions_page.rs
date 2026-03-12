use iced::{Center, Fill};
use iced::{Color, Element, Size};
use iced::advanced::Widget;
use iced::widget::{container, scrollable, space, stack, Column, Stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::{StartAddingTransaction, StartEditingTransaction};
use crate::ui::components::{cycle_theme_button, header, panel, panel_button, spacer, ui_string, ButtonShapes, Heights, Orientations, PaddingSizes, Spacing, TextSizes, Widths};
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};

// transactions page
pub fn transactions_page(
    app: &App
) -> Stack<Signal> {
    let bank = &app.bank;
    let filtered_ids = bank.get_filtered_ids(Filters::Primary);
    let transactions = filtered_ids.clone().into_iter().map(|id| {
        bank.get(id)
    }).collect();

    stack![
        transaction_list(app, transactions, ValueDisplayFormats::Dollars),

        header(
            app,
            false,
            vec![],
            vec![
                add_transaction_button(app),
            ],
        ),
    ]
    .width(Fill)
    .height(Fill)
}



// components
/// A displayed list of transactions.
pub fn transaction_list<'a>(
    app: &'a App,
    transactions: Vec<&Transaction>,
    value_display_format: ValueDisplayFormats,
)  -> Element<'a, Signal> {
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();
    for i in 0..transactions.len() {
        let transaction = &transactions[i];
        if i % 2 == 0 { first_half.push(transaction); }
        else { second_half.push(transaction); }
    }

    container(
        scrollable(
            column![
                spacer(Orientations::Vertical, Spacing::HeaderSpace),

                row![
                    column(first_half.into_iter().map(|transaction| {
                        transaction_panel(app, transaction)
                    }))
                    .spacing(Spacing::Micro.size()),

                    column(second_half.into_iter().map(|transaction| {
                        transaction_panel(app, transaction)
                    }))
                    .spacing(Spacing::Micro.size()),
                ]
                .spacing(Spacing::Small.size())
            ]
                .spacing(Spacing::None.size())
        )
            .direction(Direction::Vertical(Scrollbar::hidden()))
            .width(Widths::SmallCard.size() * 2.0 + Spacing::Small.size() * 3.0)
            .height(Fill),
    )
        .center_x(Fill)
        .into()
}

/// A panel that displays an individual transaction.
pub fn transaction_panel<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    panel(
        app,
        Materials::Plastic,
        MaterialColors::Background,
        2,
        true,
        Widths::SmallCard,
        Heights::Shrink,
        PaddingSizes::None,{
        column![
            spacer(Orientations::Vertical, Spacing::Medium),

            // value, currency, and date
            row![
                spacer(Orientations::Horizontal, Spacing::Medium),

                ui_string(app, 1, transaction.value.to_string(), TextSizes::SmallHeading),
                spacer(Orientations::Horizontal, Spacing::Micro),
                ui_string(app, 2, transaction.value.currency().to_string(), TextSizes::Body),
                spacer(Orientations::Horizontal, Spacing::Medium),
                ui_string(app, 2, transaction.date.display(), TextSizes::Body),
                spacer(Orientations::Horizontal, Spacing::Fill),
                edit_transaction_button(app, transaction),

                spacer(Orientations::Horizontal, Spacing::Medium),
            ]
            .align_y(Center),

            // description
            spacer(Orientations::Vertical, Spacing::Small),
            row![
                spacer(Orientations::Horizontal, Spacing::Medium),

                ui_string(app, 1, transaction.description.clone(), TextSizes::Body),
                spacer(Orientations::Horizontal, Spacing::Fill),

                spacer(Orientations::Horizontal, Spacing::Medium),
            ]
            .align_y(Center),

            // tags
            spacer(Orientations::Vertical, Spacing::Micro),
            scrollable(
                row({
                    let mut tags: Vec<_> = transaction.tags.iter().map(|tag| {
                        tag_panel(app, tag, app.bank.tag_registry.get(&tag).unwrap_or(MaterialColors::Unavailable))
                    }).collect();
                    tags.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
                    tags.push(spacer(Orientations::Horizontal, Spacing::Small));
                    tags
                })
                .spacing(PaddingSizes::Nano.size()),
            )
            .direction(Direction::Horizontal(Scrollbar::hidden())),

            spacer(Orientations::Vertical, Spacing::Medium),
        ]
            .spacing(Spacing::None.size())
            .into()
    })
}

/// A button that allows the user to edit a transaction.
pub fn edit_transaction_button<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        3,
        true,
        ButtonShapes::Bloated,
        icon("pencil"),
        StartEditingTransaction(transaction.get_id().expect("Tried to edit a transaction without an id!")),
        true,
    ).into()
}

/// A panel that displays a tag.
pub fn tag_panel<'a>(
    app: &'a App,
    tag: &Tag,
    color: MaterialColors,
) -> Element<'a, Signal> {
    panel(
        app,
        Materials::Acrylic,
        color,
        1,
        true,
        Widths::Shrink,
        Heights::Shrink,
        PaddingSizes::Small, {
            ui_string(app, 1, tag.display(TagStyles::Lowercase), TextSizes::Interactable)
        }
    )
}

/// Allows a user to start ading a transaction.
pub fn add_transaction_button(
    app: &App,
) -> Element<Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Success,
        1,
        true,
        ButtonShapes::Wide,
        icon("plus"),
        StartAddingTransaction,
        true,
    )
}

/// A panel that displays the cash flow for the primary filter in the bank.
pub fn cash_flow_panel<'a>(
    app: &'a App,
    cash_flow: &CashFlow,
    value_display_format: ValueDisplayFormats
) -> Element<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            panel(
                app,
                Materials::Acrylic,
                MaterialColors::Accent,
                1,
                true,
                Widths::Shrink,
                Heights::Shrink,
                PaddingSizes::Small, {
                    row![
                        spacer(Orientations::Horizontal, Spacing::Medium),
                        column(cash_flow.value_flows.iter().map(|value| {
                            ui_string(app, 1, value.to_string(), TextSizes::SmallHeading)
                        })),
                        spacer(Orientations::Horizontal, Spacing::Medium),
                    ]
                        .spacing(Spacing::None.size())
                        .into()
                }
            ).into()
        }

        ValueDisplayFormats::Time(price) => {
            panel(
                app,
                Materials::Acrylic,
                MaterialColors::Accent,
                1,
                true,
                Widths::Shrink,
                Heights::Shrink,
                PaddingSizes::Medium, {
                column(cash_flow.value_flows.iter().map(|value| {
                    let time_price_result = Transaction::get_time_price(&value, price);
                    if let Pass(time_price) = time_price_result {
                        ui_string(app, 1, time_price, TextSizes::Interactable)
                    }
                    else {
                        ui_string(app, 1, time_price_result.most_recent_result(), TextSizes::Interactable)
                    }
                })).into()
            }).into()
        }
    }
}