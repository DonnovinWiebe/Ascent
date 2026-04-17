use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Orientations, Spacing, TextSizes, Widths, center_x, header, navigation_panel, panel_button, spacer, ui_string};
use crate::ui::material::{AppThemes, MaterialColors, MaterialStyle, Materials};

/// The page used to display settings for the `App`.
#[must_use]
pub fn settings_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app),
            spacer(Orientations::Horizontal, Spacing::Fill),
        ],
        
        center_x(settings_list(app)),
        
        header(
            app,
            Vec::new(),
            Vec::new(),
        ),
    ]
    .width(Fill)
    .height(Fill)
}

/// The list of settings
#[must_use]
pub fn settings_list<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    scrollable(
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),
            setting_heading(app, "Appearance".to_string()),
            theme_setting(app),
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
pub fn setting_heading<'a>(
    app: &'a App,
    label: String,
) -> Element<'a, Signal> {
    row![
        ui_string(app, 1, label, TextSizes::LargeHeading),
        spacer(Orientations::Horizontal, Spacing::Fill),
    ]
    .into()
}

/// The theme selection setting.
#[must_use]
pub fn theme_setting<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, 1, "Theme".to_string(), TextSizes::SmallHeading),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::RimmedPlastic,
                color: if app.theme_selection == AppThemes::Peach {
                    MaterialColors::Accent
                } else {
                    MaterialColors::Background
                },
                strength: 2,
                cast_shadow: true,
            },
            ButtonShapes::Minimal,
            ui_string(app, 1, AppThemes::Peach.name(), TextSizes::Interactable),
            Signal::ChangeTheme(AppThemes::Peach),
            true,
        ),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::RimmedPlastic,
                color: if app.theme_selection == AppThemes::Midnight {
                    MaterialColors::Accent
                } else {
                    MaterialColors::Background
                },
                strength: 2,
                cast_shadow: true,
            },
            ButtonShapes::Minimal,
            ui_string(app, 1, AppThemes::Midnight.name(), TextSizes::Interactable),
            Signal::ChangeTheme(AppThemes::Midnight),
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}