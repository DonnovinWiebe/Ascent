use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::{App, Pages};
use crate::container::signal::Signal::{self, ChangePageTo};
use crate::container::warnings::Warnings;
use materialui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, panel, panel_button, spacer, ui_string};
use materialui::materials::{Depths, MaterialColors, MaterialStyle, Materials};
use iced_font_awesome::fa_icon_solid as icon;

/// The page used to display warnings
#[must_use]
pub fn warnings_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(warnings_panel(app)).center(Fill),
        header(app, Vec::new()),
    ]
}

/// Displays the warnings collected by the `App`.
#[must_use]
fn warnings_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud
        },
        PanelSize { width: Widths::MediumCard, height: Heights::Shrink },
        PaddingSizes::Small, {
            column![
                // heading
                row![
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    ui_string(app, "Ascent has detected some potential problems.", TextSizes::LargeHeading, MaterialColors::StrongText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                    close_warnings_button(app),
                ]
                .spacing(0)
                .align_y(Center),

                // warning lisst
                spacer(Orientations::Vertical, Spacing::Large),
                panel(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::CardHollow,
                        depth: Depths::Recessed,
                    },
                    PanelSize { width: Widths::Fill, height: Heights::MediumCard },
                    PaddingSizes::None, {
                        let mut warnings = app.get_app_state().get_warnings().iter().map(|w| warning_panel(app, *w)).collect::<Vec<_>>();
                        warnings.insert(0, spacer(Orientations::Vertical, Spacing::Nano));
                        warnings.push(spacer(Orientations::Vertical, Spacing::Nano));
                        
                        row![
                            spacer(Orientations::Horizontal, Spacing::Small),
                            
                            scrollable(column(warnings).spacing(Spacing::Small.size()))
                                .direction(Direction::Vertical(Scrollbar::hidden()))
                                .spacing(Spacing::Small.size())
                                .width(Fill),
                            
                            spacer(Orientations::Horizontal, Spacing::Small),
                        ]
                        .into()
                    }
                ),
                
                // dissmiss and advanced buttons
                spacer(Orientations::Vertical, Spacing::Small),
                if app.get_app_state().get_minor_errors().len() > 0 && app.get_bank().get_ledger().len() > 0 {
                    // dismiss and advanced log buttons
                    stack![
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            dismiss_warnings_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                        .spacing(0)
                        .height(Fill)
                        .align_y(Center),
                        
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            view_minor_errors_button(app),
                        ]
                        .spacing(0)
                        .height(Fill)
                        .align_y(Center),
                    ]
                    .into()
                }

                // dismiss button
                else {
                    dismiss_warnings_button(app)
                },
            ]
            .align_x(Center)
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// Displays a warning in a nice panel.
#[must_use]
fn warning_panel<'a>(
    app: &'a App,
    warning: Warnings,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud
        },
        PanelSize { width: Widths::Fill, height: Heights::Shrink },
        PaddingSizes::Small, {
            row![
                icon(&warning.icon_name()),
                spacer(Orientations::Horizontal, Spacing::Medium),
                ui_string(app, warning.description(), TextSizes::Interactable, MaterialColors::StrongText),
            ]
            .spacing(0)
            .align_y(Center)
            .into()
        }
    )
}

/// A button that closes the warnings page.
#[must_use]
fn close_warnings_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Minimal,
        icon("xmark"),
        Signal::ChangePageTo(Pages::Transactions),
        true,
    )
}

/// A button that dismisses every warning.
#[must_use]
fn dismiss_warnings_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        ui_string(app, "Dismiss", TextSizes::Interactable, MaterialColors::StrongText),
        Signal::DismissWarnings,
        true,
    )
}

/// A button that goes to the minor errors page.
#[must_use]
fn view_minor_errors_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::LowProfile,
        ui_string(app, "View Advanced Log", TextSizes::Interactable, MaterialColors::StrongText),
        Signal::ChangePageTo(Pages::MinorErrorsPage),
        app.get_app_state().get_minor_errors().len() > 0,
    )
}

/// A button that goes to this warnings page.
#[must_use]
pub fn warning_flag_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    if app.get_app_state().is_warning(app.get_bank()) {
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Acrylic,
                color: MaterialColors::warning(),
                depth: Depths::Proud
            },
            ButtonShapes::Minimal,
            icon(&Pages::WarningsPage.icon_name()),
            ChangePageTo(Pages::WarningsPage),
            app.get_app_state().is_warning(app.get_bank())
        )
    }
    
    else { spacer(Orientations::Horizontal, Spacing::None) }
}