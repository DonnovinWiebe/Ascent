use iced_font_awesome::fa_icon_solid as icon;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Orientations, Spacing, TextSizes, Widths, header, navigation_panel, panel_button, spacer, ui_string};
use crate::ui::material::{AppThemes, Depths, MaterialColors, MaterialStyle, Materials};

/// The page used to display settings for the `App`.
#[must_use]
pub fn settings_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app),
            container(settings_list(app)).center_x(Fill),
        ],
        header(app, Vec::new()),
    ]
}

/// The list of settings
#[must_use]
fn settings_list<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    scrollable(
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),
            
            // appearance
            setting_heading(app, "Appearance".to_string()),
            theme_setting(app),
            
            // save data
            spacer(Orientations::Vertical, Spacing::Large),
            setting_heading(app, "Save Data".to_string()),
            backup_button(app),
            save_data_import_button(app),
            legacy_save_data_import_button(app),
        ]
        .spacing(Spacing::Medium.size())
    )
    .direction(Direction::Vertical(Scrollbar::hidden()))
    .width(Widths::LargeCard.size())
    .height(Fill)
    .into()
}

/// Provides a label to group related settings.
#[must_use]
fn setting_heading<'a>(
    app: &'a App,
    label: String,
) -> Element<'a, Signal> {
    row![
        ui_string(app, label, TextSizes::LargeHeading, MaterialColors::StrongText),
        spacer(Orientations::Horizontal, Spacing::Fill),
    ]
    .into()
}

/// The theme selection setting.
#[must_use]
fn theme_setting<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Theme", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.theme_selection == AppThemes::Peach {
                    MaterialColors::accent(app.theme_selection)
                }
                else { MaterialColors::Card },
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            ui_string(app, AppThemes::Peach.name(), TextSizes::Interactable, MaterialColors::StrongText),
            Signal::ChangeTheme(AppThemes::Peach),
            true,
        ),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.theme_selection == AppThemes::Midnight {
                    MaterialColors::accent(app.theme_selection)
                }
                else { MaterialColors::Card },
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            ui_string(app, AppThemes::Midnight.name(), TextSizes::Interactable, MaterialColors::StrongText),
            Signal::ChangeTheme(AppThemes::Midnight),
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// The save data backup button.
#[must_use]
fn backup_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Create Backup", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon("floppy-disk"),
            Signal::Backup,
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// The save data import button.
#[must_use]
fn save_data_import_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Import Save Data", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon("file-import"),
            Signal::OpenImportFilePicker,
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// The legacy save data import button.
#[must_use]
fn legacy_save_data_import_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Import Legacy Save Data", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon("file-import"),
            Signal::OpenLegacyImportFilePicker,
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}