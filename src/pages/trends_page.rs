use iced::Length::Shrink;
use iced::Fill;
use iced::Element;
use iced::widget::{Stack, container, image, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, navigation_panel, panel, panel_button, spacer, ui_string};
use crate::ui::material::{Depths, MaterialColors, MaterialStyle, Materials};
use crate::vault::transaction::Tag;
use crate::vault::schrod::Schrod::{Fail, Pass};

/// The page used for managing the persistent coloring of `Tag`s.
#[must_use]
pub fn trends_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app),
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
                match &app.trend_parse_result {
                    Pass(trend_parse) => {
                        match &trend_parse.chart_handle {
                            Pass(handle) => { image(handle.clone()).into() }
                            Fail(_) => { ui_string(app, "No chart generated!", TextSizes::SmallHeading, MaterialColors::StrongText) }
                        }
                    }
                    
                    Fail(_) => ui_string(app, "Invalid TrendParse!", TextSizes::SmallHeading, MaterialColors::StrongText),
                },

                spacer(Orientations::Horizontal, Spacing::Large),
                row![
                    reduce_trend_panel(app),
                    extend_trend_panel(app),
                    toggle_show_balance(app),
                    trending_tags(app),
                ]
                .spacing(0),
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
    let color = if app.show_overall_cash_flow_line { MaterialColors::accent(app.theme_selection) }
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
        Signal::ToggleShowOverallCashFlowLine,
        true,
    )
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
        PanelSize { width: Widths::Shrink, height: Heights::Shrink },
        PaddingSizes::None, {
            let mut tag_panels: Vec<_> = app.bank.get_tags().into_iter().map(|tag| trending_tag_panel(app, tag)).collect();
            tag_panels.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            tag_panels.push(spacer(Orientations::Horizontal, Spacing::Small));
            
            column![
                spacer(Orientations::Vertical, Spacing::Nano),
                
                scrollable(row(tag_panels))
                    .direction(Direction::Horizontal(Scrollbar::hidden()))
                    .width(Widths::SmallCard.size())
                    .height(Shrink),
                
                spacer(Orientations::Vertical, Spacing::Nano),
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
    tag: Tag,
) -> Element<'a, Signal> {
    let mut color = MaterialColors::CardHollowContent;
    let signal = match &app.trend_parse_result {
        Pass(trend_parse) => {
            if trend_parse.is_tag_trending(&tag) {
                color = app.bank.tag_registry.get(&tag);
                Signal::RemoveTrendingTag(tag.clone())
            }
            else { Signal::AddTrendingTag(tag.clone()) }
        }
        Fail(_) => Signal::AddTrendingTag(tag.clone()),
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
        Signal::ExtendTrendingLength,
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
        Signal::ReduceTrendingLength,
        true,
    )
}