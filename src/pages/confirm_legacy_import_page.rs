use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, stack};
use iced::widget::column;
use iced::widget::row;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, panel, panel_button, spacer, ui_string};
use crate::ui::material::{Depths, MaterialColors, MaterialStyle, Materials};

/// The page used to confirm if the user wants to replace the current `Transaction`s with those from an external save file (legacy).
#[must_use]
pub fn confirm_legacy_import_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(confirm_legacy_import_panel(app)).center(Fill),
        header(app, Vec::new()),
    ]
}

/// The panel that allows a user to confirm or cancel an import.
#[must_use]
fn confirm_legacy_import_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    match &app.legacy_import_data {
        // assuming the app contains legacy import data
        Some(import_data) => {
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
                        ui_string(app, "Would you like to load legacy Transactions?", TextSizes::LargeHeading, MaterialColors::StrongText),
                        spacer(Orientations::Vertical, Spacing::Small),
                        ui_string(app, format!("{} Transactions found.", import_data.len()), TextSizes::SmallHeading, MaterialColors::MediumText),
                        spacer(Orientations::Vertical, Spacing::Large),
                        ui_string(app, "Please note that importing these Transactions will erase all previous save data.", TextSizes::SmallHeading, MaterialColors::StrongText),
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
                    color: MaterialColors::Card,
                    depth: Depths::Proud,
                },
                PanelSize { width: Widths::SmallCard, height: Heights::Shrink },
                PaddingSizes::Medium, {
                    column![
                        ui_string(app, "Woops! No data has been loaded.", TextSizes::LargeHeading, MaterialColors::StrongText),
                        spacer(Orientations::Vertical, Spacing::Medium),
                        cancel_legacy_import_button(app)
                    ]
                    .align_x(Center)
                    .into()
                }
            )
        }
    }
}

/// Confirms a legacy data import.
#[must_use]
fn confirm_legacy_import_button<'a>(
    app: &'a App
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud
        },
        ButtonShapes::Wide,
        ui_string(app, "Confirm", TextSizes::Interactable, MaterialColors::StrongText),
        Signal::ConfirmLegacyImport,
        true,
    )
}

/// Cancels a legacy data import.
#[must_use]
fn cancel_legacy_import_button<'a>(
    app: &'a App
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::danger(),
            depth: Depths::Proud
        },
        ButtonShapes::Wide,
        ui_string(app, "Cancel", TextSizes::Interactable, MaterialColors::StrongText),
        Signal::CancelLegacyImport,
        true,
    )
}