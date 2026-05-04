use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, panel, panel_button, spacer, ui_string};
use crate::ui::material::{Depths, MaterialColors, MaterialStyle, Materials};

/// The page used to display application errors as they happen.
#[must_use]
pub fn application_errors_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(application_errors_panel(app)).center(Fill),
        header(app, Vec::new()),
    ]
}

/// Displays the errors collected by the `App`.
fn application_errors_panel<'a>(
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
        PaddingSizes::Medium, {
            column![
                ui_string(app, "Ascent has encountered an error!", TextSizes::LargeHeading, MaterialColors::StrongText),
                spacer(Orientations::Vertical, Spacing::Micro),
                ui_string(app, "Here is the call stack...", TextSizes::SmallHeading, MaterialColors::MediumText),
                
                spacer(Orientations::Vertical, Spacing::Large),
                panel(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::CardHollow,
                        depth: Depths::Proud,
                    },
                    PanelSize { width: Widths::Fill, height: Heights::MediumCard },
                    PaddingSizes::None, {
                        let mut errors = app.application_failures.iter().map(|f| ui_string(app, f, TextSizes::SmallHeading, MaterialColors::StrongText)).collect::<Vec<_>>();
                        errors.insert(0, spacer(Orientations::Vertical, Spacing::Nano));
                        errors.push(spacer(Orientations::Vertical, Spacing::Nano));
                        
                        row![
                            spacer(Orientations::Horizontal, Spacing::Small),
                            
                            scrollable(column(errors).spacing(Spacing::Small.size()))
                                .direction(Direction::Vertical(Scrollbar::hidden()))
                                .spacing(Spacing::Small.size())
                                .width(Fill),
                            
                            spacer(Orientations::Horizontal, Spacing::Small),
                        ]
                        .into()
                    }
                ),
                
                spacer(Orientations::Vertical, Spacing::Large),
                dismiss_errors_button(app)
            ]
            .align_x(Center)
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// A button that dismisses every application error.
#[must_use]
fn dismiss_errors_button<'a>(
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
        Signal::DismissErrors,
        true,
    )
}