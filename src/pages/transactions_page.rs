use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, image, mouse_area, responsive, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::pages::filter_ui::*;
use crate::ui::components::*;
use crate::ui::material::{MaterialColors, MaterialStyle, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::parse::*;

// transactions page
pub fn transactions_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    let bank = &app.bank;
    let filtered_ids = bank.get_filtered_ids(Filters::Primary);
    let transactions = filtered_ids.clone().into_iter().map(|id| {
        bank.get(id).unwrap() // todo: this is temporary - fix later
    }).collect();
    
    let mut elements: Vec<Element<Signal>> = Vec::new();
    
    elements.push(
        container(
            row![
                spacer(Orientations::Horizontal, Spacing::Small),
                navigation_panel(app),
                spacer(Orientations::Horizontal, Spacing::Fill),
                transaction_list(app, transactions/*, ValueDisplayFormats::Dollars*/),
                spacer(Orientations::Horizontal, Spacing::Fill),
                management_panel(app),
                spacer(Orientations::Horizontal, Spacing::Small),
            ]
            .spacing(Spacing::None.size())
        )
        .center_x(Fill)
        .into()
    );
    
    if app.hovered_segment.is_some() {
        elements.push(
            segment_popup(app)
        );
    }
    
    elements.push(
        header(
            app,
            Vec::new(),
            Vec::new(),
        ),
    );

    stack(elements)
    .width(Fill)
    .height(Fill)
}



// components
/// A displayed list of transactions.
pub fn transaction_list<'a>(
    app: &'a App,
    transactions: Vec<&Transaction>,
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
    .height(Fill)
    .into()
}

/// A panel that displays an individual transaction.
pub fn transaction_panel<'a>(
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

/// A button that allows the user to edit a transaction.
pub fn edit_transaction_button<'a>(
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
        StartEditingTransaction(transaction.get_id().expect("Tried to edit a transaction without an id!")),
        true,
    )
}

/// A panel that displays a tag.
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

/// Allows a user to start ading a transaction.
pub fn add_transaction_button<'a>(
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
        StartAddingTransaction,
        true,
    )
}

/// Allows a user to start ading a transaction.
pub fn open_tag_registry_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Accent,
            strength: 1,
            cast_shadow: true,
        },
        ButtonShapes::Wide,
        icon("tag"),
        OpenTagRegistry,
        true,
    )
}

pub fn management_panel<'a>(
    app: &'a App
) -> Element<'a, Signal> {
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
            PanelSize { width: Widths::MediumCard, height: Heights::Fill },
            PaddingSizes::None, {
                scrollable(
                    column![
                        row![
                            // filtering
                            column![
                                spacer(Orientations::Vertical, Spacing::Small),
                                
                                // general controls
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    add_transaction_button(app),
                                    spacer(Orientations::Horizontal, Spacing::Large),
                                    open_tag_registry_button(app),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ]
                                .spacing(Spacing::None.size()),
                                
                                // year
                                spacer(Orientations::Vertical, Spacing::Small),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    recede_filter_year_panel(app, Filters::Primary),
                                    toggle_filter_year_panel(app, Filters::Primary),
                                    advance_filter_year_panel(app, Filters::Primary),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ],
                                
                                // month
                                spacer(Orientations::Vertical, Spacing::Small),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    recede_filter_month_panel(app, Filters::Primary),
                                    toggle_filter_month_panel(app, Filters::Primary),
                                    advance_filter_month_panel(app, Filters::Primary),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ],
                                
                                // tags
                                spacer(Orientations::Vertical, Spacing::Small),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    filter_tags(app, Filters::Primary),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ],
                                
                                // search terms
                                spacer(Orientations::Vertical, Spacing::Small),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    search_terms(app, Filters::Primary),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ],
                                spacer(Orientations::Vertical, Spacing::Micro),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    search_bar(app, Filters::Primary),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ],
                                
                                // filter mode
                                spacer(Orientations::Vertical, Spacing::Small),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    filter_mode_toggle_button(app, Filters::Primary),
                                    spacer(Orientations::Horizontal, Spacing::Fill),
                                ],
                            ]
                            .align_x(Center)
                            .spacing(Spacing::None.size()),
                            
                            // parsing
                            column![
                                spacer(Orientations::Vertical, Spacing::Small),
                                
                                // cash flow
                                spacer(Orientations::Vertical, Spacing::Small),
                                row![
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                    cash_flow_panel(app, &CashFlow::new(app.bank.primary_filter.get_filtered_ids(), &app.bank), ValueDisplayFormats::Dollars),
                                    spacer(Orientations::Horizontal, Spacing::Small),
                                ]
                                .spacing(Spacing::None.size()),
                                
                                // ring charts
                                ring_charts(app),
                                
                                spacer(Orientations::Vertical, Spacing::Small),
                            ]
                            .align_x(Center)
                            .spacing(Spacing::None.size())
                        ]
                        .spacing(Spacing::None.size()),
                    ]
                    .spacing(Spacing::None.size()),
                )
                .direction(Direction::Vertical(Scrollbar::hidden()))
                .into()
            }
        ),
        
        spacer(Orientations::Vertical, Spacing::Small),
    ]
    .spacing(Spacing::None.size())
    .into()
}

/// A panel that displays the cash flow for the primary filter in the bank.
pub fn cash_flow_panel<'a>(
    app: &'a App,
    cash_flow: &CashFlow,
    value_display_format: ValueDisplayFormats
) -> Element<'a, Signal> {
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
                        let time_price_result = Transaction::get_time_price(value);
                        if let Pass(time_price) = time_price_result {
                            ui_string(app, 1, time_price, TextSizes::Interactable)
                        }
                        else {
                            ui_string(app, 1, time_price_result.most_recent_result(), TextSizes::Interactable)
                        }
                    }))
                    .align_x(Center)
                    .spacing(Spacing::Small.size())
                    .into()
                }
            }
        }
    )
}

/// A visual representation of how much earning and spending there is associated with each tag.
pub fn ring_charts<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    column![
        ui_string(app, 1, "Earning".to_string(), TextSizes::SmallHeading),
        spacer(Orientations::Vertical, Spacing::Micro),
        match &app.earning_ring_parse_result {
            Pass(earning_ring_parse) => {
                responsive(|layout_size| {
                    mouse_area(image(earning_ring_parse.get_current_handle()))
                        .on_move(move |point| MouseMovedInEarningRingChart(point, layout_size))
                        .on_exit(MouseExitedEarningRingChart)
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
                        .on_move(move |point| MouseMovedInSpendingRingChart(point, layout_size))
                        .on_exit(MouseExitedSpendingRingChart)
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

/// A popup for displaying the tag name and percentage when a Ring Chart Segment is hovered over.
pub fn segment_popup<'a>(
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
            PaddingSizes::Large, {
                match &app.hovered_segment {
                    Some(segment) => {
                        column![
                            ui_string(app, 1, segment.get_tag().get_label().to_string(), TextSizes::LargeHeading),
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