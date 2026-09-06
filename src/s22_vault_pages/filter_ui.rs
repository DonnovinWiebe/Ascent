use iced::Center;
use iced::Element;
use iced::widget::scrollable;
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::s11_container::app::App;
use crate::s11_container::signal::FilterSignal;
use crate::s11_container::signal::Signal;
use materialui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, panel, panel_button, panel_text_input, spacer, ui_string};
use materialui::materials::Depths;
use materialui::materials::MaterialStyle;
use materialui::materials::{MaterialColors, Materials};
use crate::s21_vault::bank::Filters;
use crate::s21_vault::filter::FilterModes;
use crate::s21_vault::transaction::{Date, Tag};

/// Toggles the filter year panel by setting or clearing the filter year.
#[must_use]
pub fn toggle_filter_year_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_year = app.get_bank().get_filter(filter).get_filter_year();
    let new_filter_year = match &current_filter_year {
        Some(_) => None,
        None => Some(app.get_bank().get_latest_date_for_filter(filter).get_year()),
    };
    let label = match current_filter_year {
        Some(year) => format!("{year}"),
        None => "Year".to_string(),
    };
    let signal = match new_filter_year {
        Some(year) => Signal::FilterSignal(FilterSignal::SetFilterYear(year, filter)),
        None => Signal::FilterSignal(FilterSignal::ClearFilterYear(filter)),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Standard,
        ui_string(app, label, TextSizes::Interactable, MaterialColors::StrongText),
        signal,
        true,
    )
}

/// Advances the filter year.
#[must_use]
pub fn advance_filter_year_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_year = app.get_bank().get_filter(filter).get_filter_year();
    let new_filter_year = match &current_filter_year {
        Some(year) => Date::get_advanced_year(*year),
        None => app.get_bank().get_latest_date_for_filter(filter).get_year(),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Minimal,
        icon("chevron-right"),
        Signal::FilterSignal(FilterSignal::SetFilterYear(new_filter_year, filter)),
        true,
    )
}

/// Recedes the filter year.
#[must_use]
pub fn recede_filter_year_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_year = app.get_bank().get_filter(filter).get_filter_year();
    let new_filter_year = match &current_filter_year {
        Some(year) => Date::get_receded_year(*year),
        None => app.get_bank().get_latest_date_for_filter(filter).get_year(),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud
        },
        ButtonShapes::Minimal,
        icon("chevron-left"),
        Signal::FilterSignal(FilterSignal::SetFilterYear(new_filter_year, filter)),
        true,
    )
}


/// Toggles the filter month panel by setting or clearing the filter month.
#[must_use]
pub fn toggle_filter_month_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_month = app.get_bank().get_filter(filter).get_filter_month();
    let new_filter_month = match &current_filter_month {
        Some(_) => None,
        None => Some(app.get_bank().get_latest_date_for_filter(filter).get_month()),
    };
    let label = match current_filter_month {
        Some(month) => month.display(),
        None => "Month".to_string(),
    };
    let signal = match new_filter_month {
        Some(month) => Signal::FilterSignal(FilterSignal::SetFilterMonth(month, filter)),
        None => Signal::FilterSignal(FilterSignal::ClearFilterMonth(filter)),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Standard,
        ui_string(app, label, TextSizes::Interactable, MaterialColors::StrongText),
        signal,
        true,
    )
}

/// Advances the filter month.
#[must_use]
pub fn advance_filter_month_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_month = app.get_bank().get_filter(filter).get_filter_month();
    let new_filter_month = match &current_filter_month {
        Some(month) => month.get_next(),
        None => app.get_bank().get_latest_date_for_filter(filter).get_month(),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Minimal,
        icon("chevron-right"),
        Signal::FilterSignal(FilterSignal::SetFilterMonth(new_filter_month, filter)),
        true,
    )
}

/// Recedes the filter month.
#[must_use]
pub fn recede_filter_month_panel<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_filter_month = app.get_bank().get_filter(filter).get_filter_month();
    let new_filter_month = match &current_filter_month {
        Some(month) => month.get_previous(),
        None => app.get_bank().get_latest_date_for_filter(filter).get_month(),
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Minimal,
        icon("chevron-left"),
        Signal::FilterSignal(FilterSignal::SetFilterMonth(new_filter_month, filter)),
        true,
    )
}

/// Lists the `Tag`s for filtering.
#[must_use]
pub fn filter_tags<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollow,
            depth: Depths::Recessed
        },
        PanelSize { width: Widths::Fill, height: Heights::MicroCard },
        PaddingSizes::None, {
            let tags = app.get_bank().get_tags();
            let mut first_half = Vec::new();
            let mut second_half = Vec::new();
            for (i, existing_tag) in tags.iter().enumerate() {
                let tag = existing_tag.clone();
                if i % 2 == 0 { first_half.push(filter_tag_panel(app, &tag, filter)); }
                else { second_half.push(filter_tag_panel(app, &tag, filter)); }
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

/// A panel for filtering `Transaction`s by `Tag`.
#[must_use]
pub fn filter_tag_panel<'a>(
    app: &'a App,
    tag: &Tag,
    filter: Filters
) -> Element<'a, Signal> {
    let signal = if app.get_bank().is_tag_filtered(tag, filter) {
        Signal::FilterSignal(FilterSignal::RemoveFilterTag(tag.clone(), filter))
    } else {
        Signal::FilterSignal(FilterSignal::AddFilterTag(tag.clone(), filter))
    };
    let color = if app.get_bank().is_tag_filtered(tag, filter) {
        app.get_bank().tag_registry.get(tag)
    } else {
        MaterialColors::CardHollowContent
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

/// Used for adding search terms to the given `Filter`.
#[must_use]
pub fn search_bar<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_search_term_string = match filter {
        Filters::Primary => &app.get_filter_state().primary_filter_current_search_term_string(),
        Filters::DeepDive1 => &app.get_filter_state().deep_dive_1_filter_current_search_term_string(),
        Filters::DeepDive2 => &app.get_filter_state().deep_dive_2_filter_current_search_term_string(),
    };
    let update_signal = move |str| match filter {
        Filters::Primary => Signal::FilterSignal(FilterSignal::UpdatePrimaryFilterCurrentSearchTermString(str)),
        Filters::DeepDive1 => Signal::FilterSignal(FilterSignal::UpdateDeepDive1FilterCurrentSearchTermString(str)),
        Filters::DeepDive2 => Signal::FilterSignal(FilterSignal::UpdateDeepDive2FilterCurrentSearchTermString(str)),
    };

    row![
        panel_text_input(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::CardContent,
                depth: Depths::Proud
            },
            Widths::Fill,
            "Search Term",
            current_search_term_string,
            update_signal,
            Some(Signal::FilterSignal(FilterSignal::AddFilterSearchTerm(filter))),
            true,
        ),
        spacer(Orientations::Horizontal, Spacing::Small),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::success(),
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            icon("plus"),
            Signal::FilterSignal(FilterSignal::AddFilterSearchTerm(filter)),
            true,
        ),
    ]
    .spacing(Spacing::None.size())
    .align_y(Center)
    .into()
}

/// Displays the search terms for the given `Filter`.
#[must_use]
pub fn search_terms<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollow,
            depth: Depths::Recessed
        },
        PanelSize { width: Widths::Fill, height: Heights::NanoCard },
        PaddingSizes::None, {
            let mut terms: Vec<Element<'a, Signal>> = app.get_bank().get_filter(filter).get_search_terms().into_iter().map(|term| search_term_panel(app, term, filter)).collect();
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
#[must_use]
pub fn search_term_panel<'a>(
    app: &'a App,
    term: String,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud
        },
        PanelSize { width: Widths::Shrink, height: Heights::Shrink },
        PaddingSizes::None, {
            row![
                ui_string(app, &term, TextSizes::Interactable, MaterialColors::StrongText),
                spacer(Orientations::Horizontal, Spacing::Micro),
                panel_button(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::CardHollowContent,
                        depth: Depths::Proud
                    },
                    ButtonShapes::LowProfile,
                    icon("trash"),
                    Signal::FilterSignal(FilterSignal::RemoveFilterSearchTerm(term, filter)),
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

/// A toggle button for switching between filter modes (OR/AND).
#[must_use]
pub fn filter_mode_toggle_button<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    let current_mode = app.get_bank().get_filter(filter).get_filter_mode();
    let label = match current_mode {
        FilterModes::Or => "Any Matches".to_string(),
        FilterModes::And => "All Matches".to_string(),
    };
    let color = match current_mode {
        FilterModes::Or => MaterialColors::Fern,
        FilterModes::And => MaterialColors::Amber,
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color,
            depth: Depths::Proud
        },
        ButtonShapes::Minimal,
        ui_string(app, label, TextSizes::Interactable, MaterialColors::StrongText),
        Signal::FilterSignal(FilterSignal::ToggleFilterMode(filter)),
        true,
    )
}