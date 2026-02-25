use iced::{Center, Fill};
use iced::{Color, Element, Size};
use iced::widget::{container, scrollable, space, Column};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::StartEditingTransaction;
use crate::ui::components::{cycle_theme_button, panel, panel_button, standard_text, PaddingSizes, TextSizes, Widths};
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};

// transactions page
pub fn transactions_page(
    app: &App
) -> Column<Signal> {
    let bank = &app.bank;
    let filtered_ids = bank.get_filtered_ids(Filters::Primary);
    let transactions = filtered_ids.clone().into_iter().map(|id| {
        bank.get(id)
    }).collect();

    column![
        row![
            space::horizontal(),
            cash_flow_panel(app, &CashFlow::new(filtered_ids.clone(), &app.bank), ValueDisplayFormats::Dollars),
            space().width(PaddingSizes::Medium.size()),
            cycle_theme_button(app),
        ],

        transaction_list(app, transactions, ValueDisplayFormats::Dollars),
    ]
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
            row![
                column(first_half.into_iter().map(|transaction| {
                    transaction_panel(app, transaction)
                }))
                .spacing(PaddingSizes::Small.size()),

                //space().width(PaddingSizes::Small.size()),

                column(second_half.into_iter().map(|transaction| {
                    transaction_panel(app, transaction)
                }))
                .spacing(PaddingSizes::Small.size()),
            ]
        )
            .direction(Direction::Vertical(Scrollbar::hidden()))
            .width(Widths::SmallCard.size() * 2.0 + PaddingSizes::Medium.size() * 3.0)
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
    panel(app, Materials::Plastic, MaterialColors::Background, 2, true, PaddingSizes::Other(0.0), Some(Widths::SmallCard), None, {
        column![
            space().height(PaddingSizes::Medium.size()),

            row![
                space().width(PaddingSizes::Medium.size()),
                standard_text(app, TextSizes::SmallHeading, 1, transaction.value.to_string()),
                space().width(PaddingSizes::Large.size()),
                standard_text(app, TextSizes::Body, 2, transaction.date.display()),
                space().width(PaddingSizes::Large.size()),
                space::horizontal(),
                edit_transaction_button(app, transaction),
                space().width(PaddingSizes::Medium.size()),
            ],

            row![
                space().width(PaddingSizes::Medium.size()),
                standard_text(app, TextSizes::Body, 1, transaction.description.clone()),
                space::horizontal(),
                space().width(PaddingSizes::Medium.size()),
            ],

            scrollable(
                row({
                    let mut tags: Vec<_> = transaction.tags.iter().map(|tag| {
                        tag_panel(app, tag, app.bank.tag_registry.get(&tag).unwrap_or(MaterialColors::Unavailable))
                    }).collect();
                    tags.insert(0, space().width(PaddingSizes::Nano.size()).into());
                    tags.push(space().width(PaddingSizes::Nano.size()).into());
                    tags
                    })
                .spacing(PaddingSizes::Nano.size()),
            )
            .direction(Direction::Horizontal(Scrollbar::hidden())),

            space().height(PaddingSizes::Medium.size()),
        ]
            .spacing(PaddingSizes::Small.size())
            .into()
    })
}

/// A button that allows the user to edit a transaction.
pub fn edit_transaction_button<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    panel_button(app, Materials::RimmedPlastic, MaterialColors::Accent, 1, true, "Edit", StartEditingTransaction(transaction.get_id().expect("Tried to edit a transaction without an id!"))).into()
}

/// A panel that displays a tag.
pub fn tag_panel<'a>(
    app: &'a App,
    tag: &Tag,
    color: MaterialColors,
) -> Element<'a, Signal> {
    panel(app, Materials::Acrylic, color, 1, false, PaddingSizes::Small, None, None, {
        standard_text(app, TextSizes::Interactable, 1, tag.display(TagStyles::Lowercase))
    })
}

/// A panel that displays the cash flow for the primary filter in the bank.
pub fn cash_flow_panel<'a>(
    app: &'a App,
    cash_flow: &CashFlow,
    value_display_format: ValueDisplayFormats
) -> Element<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            panel(app, Materials::Acrylic, MaterialColors::Accent, 1, true, PaddingSizes::Small, None, None, {
                column(cash_flow.value_flows.iter().map(|value| {
                    standard_text(app, TextSizes::Interactable, 1, value.to_string())
                })).into()
            }).into()
        }

        ValueDisplayFormats::Time(price) => {
            panel(app, Materials::Acrylic, MaterialColors::Accent, 1, true, PaddingSizes::Medium, None, None, {
                column(cash_flow.value_flows.iter().map(|value| {
                    standard_text(app, TextSizes::Interactable, 1, Transaction::get_time_price(&value, price).to_string())
                })).into()
            }).into()
        }
    }
}