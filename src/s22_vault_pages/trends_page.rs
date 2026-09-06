use iced::Alignment::Center;
use iced::Fill;
use iced::Element;
use iced::widget::{Stack, container, image, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use materialui::components::ThemeProvider;
use crate::s11_container::app::App;
use crate::s11_container::app::Pages;
use crate::s11_container::signal::TrendsSignal;
use crate::s21_vault::trend_parse::Intervals;
use iced_font_awesome::fa_icon_solid as icon;
use crate::s11_container::signal::Signal;
use materialui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, navigation_panel, panel, panel_button, spacer, ui_string};
use materialui::materials::{Depths, MaterialColors, MaterialStyle, Materials};
use crate::s21_vault::transaction::Tag;
use schrod::Schrod::{Fail, Pass};

/// The page used for managing the persistent coloring of `Tag`s.
#[must_use]
pub fn trends_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app, Pages::page_pointers(app)),
            container(trends_panel(app)).center(Fill),
        ],
        header(app, Vec::new()),
    ]
}

/// A panel used to edit the `TagRegistry`.
#[must_use]
fn trends_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::GinormousCard, height: Heights::GinormousCard },
        PaddingSizes::Small, {
            column![
                // title
                row![
                    ui_string(app, "Trends", TextSizes::LargeHeading, MaterialColors::StrongText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center),

                // controls
                row![
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    toggle_show_balance(app),
                    spacer(Orientations::Horizontal, Spacing::Large),
                    reduce_trend_panel(app),
                    extend_trend_panel(app),
                    spacer(Orientations::Horizontal, Spacing::Large),
                    interval_selector(app),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .spacing(0),
                
                spacer(Orientations::Vertical, Spacing::Small),
                trending_tags(app),

                // chart
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    
                    match &app.get_trends_state().trend_parse_result() {
                        Pass(trend_parse) => {
                            match &trend_parse.chart_handle {
                                Pass(handle) => { image(handle.clone()).into() }
                                Fail(_) => { ui_string(app, "No chart generated!", TextSizes::SmallHeading, MaterialColors::StrongText) }
                            }
                        }
                        
                        Fail(_) => ui_string(app, "Invalid TrendParse!", TextSizes::SmallHeading, MaterialColors::StrongText),
                    },
                    
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .spacing(0)
            ]
            .spacing(0)
            .into()
        }
    )
}

/// Toggles if the overall cash flow line is shown or not.
#[must_use]
fn toggle_show_balance<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    let color = if app.get_trends_state().show_balance_line() { MaterialColors::accent(app.material_theme()) }
    else { MaterialColors::CardContent };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color,
            depth: Depths::Proud
        },
        ButtonShapes::Minimal,
        ui_string(app, "Overall", TextSizes::Interactable, MaterialColors::StrongText),
        Signal::TrendsSignal(TrendsSignal::ToggleShowBalance),
        true,
    )
}

fn interval_selector<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.get_trends_state().interval() == Intervals::Weekly { MaterialColors::accent(app.material_theme()) } else { MaterialColors::CardContent },
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            ui_string(app, "Weekly", TextSizes::Interactable, MaterialColors::StrongText),
            Signal::TrendsSignal(TrendsSignal::SetTrendingInterval(Intervals::Weekly)),
            true,
        ),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.get_trends_state().interval() == Intervals::BiWeekly { MaterialColors::accent(app.material_theme()) } else { MaterialColors::CardContent },
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            ui_string(app, "BiWeekly", TextSizes::Interactable, MaterialColors::StrongText),
            Signal::TrendsSignal(TrendsSignal::SetTrendingInterval(Intervals::BiWeekly)),
            true,
        ),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.get_trends_state().interval() == Intervals::Monthly { MaterialColors::accent(app.material_theme()) } else { MaterialColors::CardContent },
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            ui_string(app, "Monthly", TextSizes::Interactable, MaterialColors::StrongText),
            Signal::TrendsSignal(TrendsSignal::SetTrendingInterval(Intervals::Monthly)),
            true,
        ),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.get_trends_state().interval() == Intervals::Quarterly { MaterialColors::accent(app.material_theme()) } else { MaterialColors::CardContent },
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            ui_string(app, "Quarterly", TextSizes::Interactable, MaterialColors::StrongText),
            Signal::TrendsSignal(TrendsSignal::SetTrendingInterval(Intervals::Quarterly)),
            true,
        ),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.get_trends_state().interval() == Intervals::Yearly { MaterialColors::accent(app.material_theme()) } else { MaterialColors::CardContent },
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            ui_string(app, "Yearly", TextSizes::Interactable, MaterialColors::StrongText),
            Signal::TrendsSignal(TrendsSignal::SetTrendingInterval(Intervals::Yearly)),
            true,
        ),
    ]
    .spacing(Spacing::Micro.size())
    .align_y(Center)
    .into()
}

fn trending_tags<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollow,
            depth: Depths::Recessed
        },
        PanelSize { width: Widths::Fill, height: Heights::Shrink },
        PaddingSizes::None, {
            let mut tag_panels: Vec<_> = app.get_bank().get_tags().into_iter().map(|tag| trending_tag_panel(app, &tag)).collect();
            tag_panels.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            tag_panels.push(spacer(Orientations::Horizontal, Spacing::Small));
            
            column![
                spacer(Orientations::Vertical, Spacing::Small),
                
                scrollable(row(tag_panels))
                    .direction(Direction::Horizontal(Scrollbar::hidden())),
                    //.width(Widths::MediumCard.size())
                    //.height(Shrink),
                
                spacer(Orientations::Vertical, Spacing::Small),
            ]
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// Toggles if a `Tag` is trending or not.
#[must_use]
fn trending_tag_panel<'a>(
    app: &'a App,
    tag: &Tag,
) -> Element<'a, Signal> {
    let mut color = MaterialColors::CardHollowContent;
    let signal = match &app.get_trends_state().trend_parse_result() {
        Pass(trend_parse) => {
            if trend_parse.is_tag_trending(tag) {
                color = app.get_bank().tag_registry.get(tag);
                Signal::TrendsSignal(TrendsSignal::RemoveTrendingTag(tag.clone()))
            }
            else { Signal::TrendsSignal(TrendsSignal::AddTrendingTag(tag.clone())) }
        }
        Fail(_) => Signal::TrendsSignal(TrendsSignal::AddTrendingTag(tag.clone())),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color,
            depth: Depths::Proud
        },
        ButtonShapes::Minimal,
        ui_string(app, tag.get_label(), TextSizes::Interactable, MaterialColors::StrongText),
        signal,
        true,
    )
}

/// Extends the length of the `TrendParse`.
#[must_use]
fn extend_trend_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Standard,
        icon("calendar-plus"),
        Signal::TrendsSignal(TrendsSignal::ExtendTrendingLength),
        true,
    )
}

/// Reduces the length of the `TrendParse`.
#[must_use]
fn reduce_trend_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Standard,
        icon("calendar-minus"),
        Signal::TrendsSignal(TrendsSignal::ReduceTrendingLength),
        true,
    )
}