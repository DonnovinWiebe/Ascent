use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, stack};
use iced::widget::column;
use iced::widget::row;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, navigation_panel, panel, panel_button, spacer, ui_string};
use crate::ui::material::{MaterialColors, MaterialStyle, Materials};

/// The page used to confirm if the user wants to replace the current `Transaction`s with those from an external save file (legacy).
#[must_use]
pub fn confirm_legacy_import_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app),
            container(confirm_legacy_import_panel(app)).center(Fill),
        ],
        header(app, Vec::new(), Vec::new()),
    ]
}

/// The panel that allows a user to confirm or cancel an import.
#[must_use]
pub fn confirm_legacy_import_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    match &app.legacy_import_data {
        // assuming the app contains legacy import data
        Some(import_data) => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 2,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::MediumCard, height: Heights::Shrink },
                PaddingSizes::Medium, {
                    column![
                        ui_string(app, 1, "Would you like to load legacy Transactions?".to_string(), TextSizes::LargeHeading),
                        spacer(Orientations::Vertical, Spacing::Small),
                        ui_string(app, 2, format!("{} Transactions found.", import_data.len()), TextSizes::SmallHeading),
                        spacer(Orientations::Vertical, Spacing::Large),
                        ui_string(app, 1, "Please note that importing these Transactions will erase all previous save data.".to_string(), TextSizes::SmallHeading),
                        spacer(Orientations::Vertical, Spacing::Ginormous),
                        
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            confirm_legacy_import_button(app),
                            spacer(Orientations::Horizontal, Spacing::Medium),
                            cancel_legacy_import_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                    ]
                    .align_x(Center)
                    .into()
                }
            )
        }
        
        // fallback if it does not
        None => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 2,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::SmallCard, height: Heights::Shrink },
                PaddingSizes::Medium, {
                    panel(
                        app,
                        MaterialStyle {
                            material: Materials::Plastic,
                            color: MaterialColors::Background,
                            strength: 1,
                            cast_shadow: false,
                        },
                        PanelSize { width: Widths::Fill, height: Heights::MediumCard },
                        PaddingSizes::None, {
                            column![
                                ui_string(app, 1, "Woops! No data has been loaded.".to_string(), TextSizes::LargeHeading),
                                spacer(Orientations::Vertical, Spacing::Medium),
                                cancel_legacy_import_button(app)
                            ]
                            .align_x(Center)
                            .into()
                        }
                    )
                }
            )
        }
    }
}

/// Confirms a legacy data import.
#[must_use]
pub fn confirm_legacy_import_button<'a>(
    app: &'a App
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Success,
            strength: 3,
            cast_shadow: true,
        },
        ButtonShapes::Wide,
        ui_string(app, 1, "Confirm".to_string(), TextSizes::Interactable),
        Signal::ConfirmLegacyImport,
        true,
    )
}

/// Cancels a legacy data import.
#[must_use]
pub fn cancel_legacy_import_button<'a>(
    app: &'a App
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Danger,
            strength: 3,
            cast_shadow: true,
        },
        ButtonShapes::Wide,
        ui_string(app, 1, "Cancel".to_string(), TextSizes::Interactable),
        Signal::CancelLegacyImport,
        true,
    )
}