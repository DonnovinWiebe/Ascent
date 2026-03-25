use iced::{Center, Fill};
use iced::{Length};
use iced::{Color, Element, Size};
use iced::advanced::Widget;
use iced::widget::{Column, Stack, container, image, mouse_area, responsive, scrollable, space, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::ui::components::*;
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Date, Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::parse::*;

/// Toggles the filter year panel by setting or clearing the filter year.
pub fn toggle_filter_year_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_year = app.bank.get_filter(filter).get_filter_year();
    let new_filter_year = match &current_filter_year {
        Some(_) => None,
        None => Some(app.bank.get_latest_date_for_filter(filter).get_year()),
    };
    let label = match current_filter_year {
        Some(year) => format!("{}", year),
        None => "Year".to_string(),
    };
    let signal = match new_filter_year {
        Some(year) => SetFilterYear(year, filter),
        None => ClearFilterYear(filter),
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        1,
        true,
        ButtonShapes::Standard,
        ui_string(app, 1, label, TextSizes::Interactable),
        signal,
        true,
    )
}

/// Advances the filter year.
pub fn advance_filter_year_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_year = app.bank.get_filter(filter).get_filter_year();
    let new_filter_year = match &current_filter_year {
        Some(year) => Date::get_advanced_year(*year),
        None => app.bank.get_latest_date_for_filter(filter).get_year(),
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        1,
        true,
        ButtonShapes::Minimal,
        icon("chevron-right"),
        SetFilterYear(new_filter_year, filter),
        true,
    )
}

/// Recedes the filter year.
pub fn recede_filter_year_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_year = app.bank.get_filter(filter).get_filter_year();
    let new_filter_year = match &current_filter_year {
        Some(year) => Date::get_receded_year(*year),
        None => app.bank.get_latest_date_for_filter(filter).get_year(),
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        1,
        true,
        ButtonShapes::Minimal,
        icon("chevron-left"),
        SetFilterYear(new_filter_year, filter),
        true,
    )
}


/// Toggles the filter month panel by setting or clearing the filter month.
pub fn toggle_filter_month_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_month = app.bank.get_filter(filter).get_filter_month();
    let new_filter_month = match &current_filter_month {
        Some(_) => None,
        None => Some(app.bank.get_latest_date_for_filter(filter).get_month()),
    };
    let label = match current_filter_month {
        Some(month) => month.display(),
        None => "Month".to_string(),
    };
    let signal = match new_filter_month {
        Some(month) => SetFilterMonth(month, filter),
        None => ClearFilterMonth(filter),
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        1,
        true,
        ButtonShapes::Standard,
        ui_string(app, 1, label, TextSizes::Interactable),
        signal,
        true,
    )
}

/// Advances the filter month.
pub fn advance_filter_month_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_month = app.bank.get_filter(filter).get_filter_month();
    let new_filter_month = match &current_filter_month {
        Some(month) => month.get_next(),
        None => app.bank.get_latest_date_for_filter(filter).get_month(),
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        1,
        true,
        ButtonShapes::Minimal,
        icon("chevron-right"),
        SetFilterMonth(new_filter_month, filter),
        true,
    )
}

/// Recedes the filter month.
pub fn recede_filter_month_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_month = app.bank.get_filter(filter).get_filter_month();
    let new_filter_month = match &current_filter_month {
        Some(month) => month.get_previous(),
        None => app.bank.get_latest_date_for_filter(filter).get_month(),
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Background,
        1,
        true,
        ButtonShapes::Minimal,
        icon("chevron-left"),
        SetFilterMonth(new_filter_month, filter),
        true,
    )
}

/// Lists the tags for filtering.
pub fn filter_tags_area<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        Materials::Plastic,
        MaterialColors::Background,
        1,
        false,
        Widths::Fill,
        Heights::MicroCard,
        PaddingSizes::None, {
            let tags = app.bank.get_tags();
            let mut first_half = Vec::new();
            let mut second_half = Vec::new();
            for i in 0..tags.len() {
                let tag = tags[i].clone();
                if i % 2 == 0 { first_half.push(filter_tag_panel(app, tag, filter)); }
                else { second_half.push(filter_tag_panel(app, tag, filter)); }
            }
            first_half.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            first_half.push(spacer(Orientations::Horizontal, Spacing::Small));
            second_half.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            second_half.push(spacer(Orientations::Horizontal, Spacing::Small));
            
            scrollable(
                row![
                    column![
                        spacer(Orientations::Vertical, Spacing::Fill),
                        
                        row(first_half)
                        .spacing(Spacing::Small.size()),
                        
                        spacer(Orientations::Vertical, Spacing::Nano),
                        
                        row(second_half)
                        .spacing(Spacing::Small.size()),
                        
                        spacer(Orientations::Vertical, Spacing::Fill),
                    ]
                    .spacing(Spacing::None.size())
                ]
                .spacing(Spacing::None.size())
            )
            .direction(Direction::Horizontal(Scrollbar::hidden()))
            .into()
        },
    )
}

/// A panel for filtering transactions by tag.
pub fn filter_tag_panel<'a>(
    app: &'a App,
    tag: Tag,
    filter: Filters
) -> Element<'a, Signal> {
    let signal = if app.bank.is_tag_filtered(tag.clone(), filter) {
        RemoveFilterTag(tag.clone(), filter)
    } else {
        AddFilterTag(tag.clone(), filter)
    };
    let color = if app.bank.is_tag_filtered(tag.clone(), filter) {
        app.bank.tag_registry.get(&tag)
    } else {
        MaterialColors::Background
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        color,
        3,
        true,
        ButtonShapes::Minimal,
        ui_string(app, 1, tag.get_label().to_string(), TextSizes::Interactable),
        signal,
        true,
    )
}