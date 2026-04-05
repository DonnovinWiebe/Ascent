use iced::Center;
use iced::Element;
use iced::widget::scrollable;
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::ui::components::*;
use crate::ui::material::MaterialStyle;
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::filter::FilterModes;
use crate::vault::transaction::{Date, Tag};

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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Background,
            strength: 3,
            cast_shadow: true,
        },
        ButtonShapes::Minimal,
        icon("chevron-left"),
        SetFilterMonth(new_filter_month, filter),
        true,
    )
}

/// Lists the tags for filtering.
pub fn filter_tags<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Background,
            strength: 1,
            cast_shadow: false,
        },
        PanelSize { width: Widths::Fill, height: Heights::MicroCard },
        PaddingSizes::None, {
            let tags = app.bank.get_tags();
            let mut first_half = Vec::new();
            let mut second_half = Vec::new();
            for (i, existing_tag) in tags.iter().enumerate() {
                let tag = existing_tag.clone();
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
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color,
            strength: 2,
            cast_shadow: true,
        },
        ButtonShapes::Minimal,
        ui_string(app, 1, tag.get_label().to_string(), TextSizes::Interactable),
        signal,
        true,
    )
}

/// Used for adding search terms to the given filter.
pub fn search_bar<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_search_term_string = match filter {
        Filters::Primary => &app.primary_filter_current_search_term_string,
        Filters::DeepDive1 => &app.deep_dive_1_filter_current_search_term_string,
        Filters::DeepDive2 => &app.deep_dive_2_filter_current_search_term_string,
    };
    let update_signal = match filter {
        Filters::Primary => UpdatePrimaryFilterCurrentSearchTermString,
        Filters::DeepDive1 => UpdateDeepDive1FilterCurrentSearchTermString,
        Filters::DeepDive2 => UpdateDeepDive2FilterCurrentSearchTermString,
    };

    row![
        panel_text_input(
            app,
            MaterialStyle {
                material: Materials::RimmedPlastic,
                color: MaterialColors::Background,
                strength: 3,
                cast_shadow: true,
            },
            Widths::Fill,
            "Search Term",
            current_search_term_string,
            update_signal,
        ),
        spacer(Orientations::Horizontal, Spacing::Small),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::RimmedPlastic,
                color: MaterialColors::Success,
                strength: 3,
                cast_shadow: true,
            },
            ButtonShapes::Minimal,
            icon("plus"),
            AddFilterSearchTerm(filter),
            true,
        ),
    ]
    .spacing(Spacing::None.size())
    .align_y(Center)
    .into()
}

/// Displays the search terms for the given filter.
pub fn search_terms<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Background,
            strength: 1,
            cast_shadow: false,
        },
        PanelSize { width: Widths::Fill, height: Heights::NanoCard },
        PaddingSizes::None, {
            let mut terms: Vec<Element<'a, Signal>> = app.bank.get_filter(filter).get_search_terms().into_iter().map(|term| search_term_panel(app, term, filter)).collect();
            terms.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            terms.push(spacer(Orientations::Horizontal, Spacing::Small));
            
            scrollable(
                row![
                    column![
                        spacer(Orientations::Vertical, Spacing::Fill),
                        row(terms)
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

/// Displays a filter search term.
pub fn search_term_panel<'a>(
    app: &'a App,
    term: String,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Background,
            strength: 2,
            cast_shadow: false,
        },
        PanelSize { width: Widths::Shrink, height: Heights::Shrink },
        PaddingSizes::None, {
            row![
                ui_string(app, 1, term.clone(), TextSizes::Interactable),
                spacer(Orientations::Horizontal, Spacing::Micro),
                panel_button(
                    app,
                    MaterialStyle {
                        material: Materials::RimmedPlastic,
                        color: MaterialColors::Danger,
                        strength: 3,
                        cast_shadow: true,
                    },
                    ButtonShapes::LowProfile,
                    icon("trash"),
                    RemoveFilterSearchTerm(term, filter),
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

pub fn filter_mode_toggle_button<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_mode = app.bank.get_filter(filter).get_filter_mode();
    let label = match current_mode {
        FilterModes::Or => "Any Matches".to_string(),
        FilterModes::And => "All Matches".to_string(),
    };
    let color = match current_mode {
        FilterModes::Or => MaterialColors::Fern,
        FilterModes::And => MaterialColors::Amber,
    };
    
    row![
        ui_string(app, 1, "Filter Mode".to_string(), TextSizes::Interactable),
        spacer(Orientations::Horizontal, Spacing::Micro),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::RimmedPlastic,
                color,
                strength: 3,
                cast_shadow: true,
            },
            ButtonShapes::Minimal,
            ui_string(app, 1, label, TextSizes::Interactable),
            ToggleFilterMode(filter),
            true,
        ),
    ]
    .spacing(Spacing::None.size())
    .align_y(Center)
    .into()
}