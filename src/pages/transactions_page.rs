use iced::Length::FillPortion;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, image, mouse_area, responsive, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::pages::filter_ui::{advance_filter_month_panel, advance_filter_year_panel, filter_mode_toggle_button, filter_tags, recede_filter_month_panel, recede_filter_year_panel, search_bar, search_terms, toggle_filter_month_panel, toggle_filter_year_panel};
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, navigation_panel, pad, panel, panel_button, spacer, ui_string};
use crate::ui::material::{MaterialColors, MaterialStyle, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::{CashFlow, RingParse};
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack::{self, Fail, Pass};

/// The page used to display `Transaction`s.
#[must_use]
pub fn transactions_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    let bank = &app.bank;
    let filtered_ids = bank.get_filtered_ids(Filters::Primary);
    let transactions: Vec<&Transaction> = filtered_ids.iter()
        .filter(|id| { bank.get(**id).is_pass() })
        .map(|id| { bank.get(*id).wont_fail("These ids are guaranteed to have transactions attached.")})
        .collect();
    
    stack![
        row![
            navigation_panel(app),
            stack![
                container(transaction_list(app, &transactions)).center_x(Fill),
                management_panel_overlay(app),
            ].width(FillPortion(3)),
            parse_panel(app)
        ],
        header(app, Vec::new(), Vec::new()),
        if app.hovered_segment.is_some() { segment_popup(app) } else { spacer(Orientations::Horizontal, Spacing::Small) },
    ]
}



// components
/// A displayed list of `Transaction`s.
#[must_use]
fn transaction_list<'a>(
    app: &'a App,
    transactions: &[&Transaction],
    //value_display_format: ValueDisplayFormats,
)  -> Element<'a, Signal> {
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();
    for (i, existing_transaction) in transactions.iter().enumerate() {
        if i % 2 == 0 { first_half.push(existing_transaction); }
        else { second_half.push(existing_transaction); }
    }
    scrollable(
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),

            row![
                column(first_half.into_iter().map(|transaction| { transaction_panel(app, transaction) }))
                .spacing(Spacing::Micro.size()),

                column(second_half.into_iter().map(|transaction| { transaction_panel(app, transaction) }))
                .spacing(Spacing::Micro.size()),
            ]
            .spacing(Spacing::Small.size()),
            
            spacer(Orientations::Vertical, Spacing::ManagementPanelSpace),
        ]
        .spacing(Spacing::None.size())
    )
    .direction(Direction::Vertical(Scrollbar::hidden()))
    .width(Widths::SmallCard.size() * 2.0 + Spacing::Small.size() * 3.0)
    .height(Fill)
    .into()
}

/// A panel that displays an individual `Transaction`.
#[must_use]
fn transaction_panel<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Background,
            strength: 2,
            cast_shadow: true,
        },
        PanelSize { width: Widths::SmallCard, height: Heights::Shrink },
        PaddingSizes::None, {
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
                        tag_panel(app, tag)
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

/// A button that allows the user to edit a `Transaction`.
#[must_use]
fn edit_transaction_button<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
        ButtonShapes::Bloated,
        icon("pencil"),
        Signal::StartEditingTransaction(ResultStack::from_option(transaction.get_id(), "Tried to get the id from a transaction without an id!")),
        true,
    )
}

/// A panel that displays a `Tag`.
#[must_use]
pub fn tag_panel<'a>(
    app: &'a App,
    tag: &Tag,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Acrylic,
            color: app.bank.tag_registry.get(tag),
            strength: 1,
            cast_shadow: true,
        },
        PanelSize { width: Widths::Shrink, height: Heights::Shrink },
        PaddingSizes::Small, {
            ui_string(app, 1, tag.display(TagStyles::Lowercase), TextSizes::Interactable)
        }
    )
}

/// Allows a user to start ading a `Transaction`.
#[must_use]
fn add_transaction_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Success,
            strength: 1,
            cast_shadow: true,
        },
        ButtonShapes::Wide,
        icon("plus"),
        Signal::StartAddingTransaction,
        true,
    )
}

/// Holds and aligns the management panel.
#[must_use]
fn management_panel_overlay<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    container(
        column![
            spacer(Orientations::Vertical, Spacing::Fill),
            management_panel(app),
        ]
        .spacing(Spacing::None.size())
    )
    .center_x(Fill)
    .into()
}

/// A panel that manages the `Transaction` `Filter`s and search terms for the main transactions page.
#[must_use]
fn management_panel<'a>(
    app: &'a App
) -> Element<'a, Signal> {
    pad(PaddingSizes::Small,
        panel(
            app,
            MaterialStyle {
                material: Materials::Acrylic,
                color: MaterialColors::Background,
                strength: 3, cast_shadow: true
            },
            PanelSize { width: Widths::GinormousCard, height: Heights::ManagementPanel },
            PaddingSizes::Small, {
                row![
                    spacer(Orientations::Vertical, Spacing::Fill),
                    
                    // general controls
                    column![
                        add_transaction_button(app),
                        filter_mode_toggle_button(app, Filters::Primary),
                    ]
                    .align_x(Center),
                    
                    // date
                    column![
                        ui_string(app, 2, "Date".to_string(), TextSizes::Body),
                        // month
                        row![
                            recede_filter_month_panel(app, Filters::Primary),
                            toggle_filter_month_panel(app, Filters::Primary),
                            advance_filter_month_panel(app, Filters::Primary),
                        ],
                        // year
                        row![
                            recede_filter_year_panel(app, Filters::Primary),
                            toggle_filter_year_panel(app, Filters::Primary),
                            advance_filter_year_panel(app, Filters::Primary),
                        ],
                    ]
                    .align_x(Center),
                    
                    // tags
                    column![
                        ui_string(app, 2, "Tags".to_string(), TextSizes::Body),
                        filter_tags(app, Filters::Primary),
                    ]
                    .align_x(Center),
                    
                    // search terms
                    column![
                        ui_string(app, 2, "Search Terms".to_string(), TextSizes::Body),
                        search_terms(app, Filters::Primary),
                        search_bar(app, Filters::Primary),
                    ]
                    .align_x(Center),
                    
                    spacer(Orientations::Vertical, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::Small.size())
                .into()
            }
        )
    )
}

/// A panel that visualizes information about the `Transaction`s on the screen.
#[must_use]
fn parse_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    pad(PaddingSizes::Small,
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),
            
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 2,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::SmallCard, height: Heights::Fill },
                PaddingSizes::None, {
                    scrollable(
                        column![
                            spacer(Orientations::Vertical, Spacing::Small),
                            
                            // cash flow
                            spacer(Orientations::Vertical, Spacing::Small),
                            row![
                                spacer(Orientations::Horizontal, Spacing::Small),
                                cash_flow_panel(app, ValueDisplayFormats::Dollars),
                                spacer(Orientations::Horizontal, Spacing::Small),
                            ]
                            .spacing(Spacing::None.size()),
                            
                            // ring charts
                            ring_charts(app),
                            
                            spacer(Orientations::Vertical, Spacing::Small),
                        ]
                        .align_x(Center)
                        .spacing(Spacing::None.size())
                    )
                    .direction(Direction::Vertical(Scrollbar::hidden()))
                    .into()
                }
            )
        ]
        .spacing(0)
        .into()
    )
}

/// A panel that displays the cash flow for the primary `Filter` in the `Bank`.
#[must_use]
fn cash_flow_panel<'a>(
    app: &'a App,
    value_display_format: ValueDisplayFormats,
) -> Element<'a, Signal> {
    let cash_flow_result = CashFlow::new(&app.bank.primary_filter.get_filtered_ids(), &app.bank, 1.0);
    
    match cash_flow_result {
        Pass(cash_flow) => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 3,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::Fill, height: Heights::Shrink },
                PaddingSizes::Medium, {
                    match value_display_format {
                        ValueDisplayFormats::Dollars => {
                            column(cash_flow.value_flows.iter().map(|value| {
                                ui_string(app, 1, format!("{} {}", value, value.currency()), TextSizes::SmallHeading)
                            }))
                            .align_x(Center)
                            .spacing(Spacing::Small.size())
                            .into()
                        }
                        
                        ValueDisplayFormats::Time(_) => {
                            column(cash_flow.value_flows.iter().map(|value| {
                                let time_price = Transaction::get_time_price(value);
                                let time_price_string = format!("{time_price:.2} hrs");
                                ui_string(app, 1, time_price_string, TextSizes::Interactable)
                            }))
                            .align_x(Center)
                            .spacing(Spacing::Small.size())
                            .into()
                        }
                    }
                }
            )
        }
        
        Fail(_) => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 3,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::Fill, height: Heights::Shrink },
                PaddingSizes::Medium, {
                    ui_string(app, 1, "Failed to create Cash Flow.".to_string(), TextSizes::SmallHeading)
                }
            )
        }
    }
}

/// A visual representation of how much earning and spending there is associated with each `Tag`.
#[must_use]
fn ring_charts<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    if app.are_ring_charts_ready {
        column![
            ui_string(app, 1, "Earning".to_string(), TextSizes::SmallHeading),
            spacer(Orientations::Vertical, Spacing::Micro),
            match &app.earning_ring_parse_result {
                Pass(earning_ring_parse) => {
                    responsive(|layout_size| {
                        mouse_area(image(earning_ring_parse.get_current_handle()))
                            .on_move(move |point| Signal::MouseMovedInEarningRingChart(point, layout_size))
                            .on_exit(Signal::MouseExitedEarningRingChart)
                            .into()
                    })
                    .width(RingParse::max_size())
                    .height(RingParse::max_size())
                    .into()
                },
                Fail(_) => ui_string(app, 1, "Could not create earning ring chart.".to_string(), TextSizes::SmallHeading),
            },
            
            spacer(Orientations::Vertical, Spacing::Medium),
            ui_string(app, 1, "Spending".to_string(), TextSizes::SmallHeading),
            spacer(Orientations::Vertical, Spacing::Micro),
            match &app.spending_ring_parse_result {
                Pass(spending_ring_parse) => {
                    responsive(|layout_size| {
                        mouse_area(image(spending_ring_parse.get_current_handle()))
                            .on_move(move |point| Signal::MouseMovedInSpendingRingChart(point, layout_size))
                            .on_exit(Signal::MouseExitedSpendingRingChart)
                            .into()
                    })
                    .width(RingParse::max_size())
                    .height(RingParse::max_size())
                    .into()
                },
                Fail(_) => ui_string(app, 1, "Could not create spending ring chart.".to_string(), TextSizes::SmallHeading),
            },
        ]
        //.height(Length::Fill)
        .spacing(Spacing::None.size())
        .padding(PaddingSizes::Small.size())
        .into()
    }
    else {
        ui_string(app, 1, "Loading ring charts...".to_string(), TextSizes::SmallHeading)
    }
}

/// A popup for displaying the `Tag` and percentage when a `RingChart` `Segment` is hovered over.
#[must_use]
fn segment_popup<'a>(
    app: &'a App
) -> Element<'a, Signal> {
    container(
        panel(
            app,
            MaterialStyle {
                material: Materials::Acrylic,
                color: {
                    match &app.hovered_segment {
                        Some(segment) => segment.get_color(),
                        None => MaterialColors::Background,
                    }
                },
                strength: 3,
                cast_shadow: true,
            },
            PanelSize { width: Widths::Shrink, height: Heights::Shrink },
            PaddingSizes::Ginormous, {
                match &app.hovered_segment {
                    Some(segment) => {
                        column![
                            ui_string(app, 1, segment.get_tag().get_label().clone(), TextSizes::LargeHeading),
                            spacer(Orientations::Vertical, Spacing::Small),
                            ui_string(app, 2, format!("{:.1}%", segment.get_percentage() * 100.0), TextSizes::SmallHeading),
                        ]
                        .width(Fill)
                        .spacing(Spacing::None.size())
                        .into()
                    }
                    None => {
                        ui_string(app, 1, "No Segment hovered...".to_string(), TextSizes::SmallHeading)
                    }
                }
            }
        )
    )
    .center_x(Fill)
    .center_y(Fill)
    .into()
}