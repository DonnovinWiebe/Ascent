use iced::{Center, Fill};
use iced::Element;
use iced::widget::{scrollable, stack, Stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{header, panel, panel_button, spacer, ui_string, ButtonShapes, Heights, Orientations, PaddingSizes, Spacing, TextSizes, Widths};
use crate::ui::material::{MaterialColors, Materials};

// application errors page
pub fn application_errors_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        header(app, false, Vec::new(), vec![dismiss_errors_button(app)]),
        row![
            spacer(Orientations::Horizontal, Spacing::Fill),
            column![
                spacer(Orientations::Vertical, Spacing::HeaderSpace),
                
                panel(
                    app,
                    Materials::Plastic,
                    MaterialColors::Background,
                    2,
                    true,
                    Widths::MediumCard,
                    Heights::Shrink,
                    PaddingSizes::Medium, {
                        column![
                            ui_string(app, 1, "Ascent has encountered an error!".to_string(), TextSizes::LargeHeading),
                            spacer(Orientations::Vertical, Spacing::Micro),
                            ui_string(app, 2, "Here is the call stack...".to_string(), TextSizes::SmallHeading),
                            spacer(Orientations::Vertical, Spacing::Large),
                            
                            panel(
                                app,
                                Materials::Plastic,
                                MaterialColors::Background,
                                1,
                                false,
                                Widths::Fill,
                                Heights::MediumCard,
                                PaddingSizes::None, {
                                    let mut errors = app.application_failures.iter().map(|f| ui_string(app, 1, f.clone(), TextSizes::SmallHeading)).collect::<Vec<_>>();
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
                            )
                        ]
                        .align_x(Center)
                        .spacing(Spacing::None.size())
                        .into()
                    }
                )
            ],
            spacer(Orientations::Horizontal, Spacing::Fill),
        ],
    ]
    .width(Fill)
    .height(Fill)
}

/// A button that dismisses every application error.
pub fn dismiss_errors_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Success,
        1,
        true,
        ButtonShapes::Wide,
        ui_string(app, 1, "Dismiss".to_string(), TextSizes::Interactable),
        Signal::DismissErrors,
        true,
    )
    .into()
}