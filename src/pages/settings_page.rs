use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::ui::components::*;
use crate::ui::material::{AppThemes, MaterialColors, Materials};

// transactions page
pub fn settings_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    stack![
        container(
            row![
                spacer(Orientations::Horizontal, Spacing::Small),
                navigation_panel(app),
                spacer(Orientations::Horizontal, Spacing::Fill),
                settings_list(app),
                spacer(Orientations::Horizontal, Spacing::Fill),
                spacer(Orientations::Horizontal, Spacing::Small),
            ]
            .spacing(Spacing::None.size())
        )
        .center_x(Fill),
        
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
    .width(Fill)
    .height(Fill)
    .into()
}

/// Provides a label to group related settings.
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
pub fn theme_setting<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, 1, "Theme".to_string(), TextSizes::SmallHeading),
        panel_button(
            app,
            Materials::RimmedPlastic,
            if app.theme_selection == AppThemes::Peach {
                MaterialColors::Accent
            } else {
                MaterialColors::Background
            },
            2,
            true,
            ButtonShapes::Minimal,
            ui_string(app, 1, AppThemes::Peach.name(), TextSizes::Interactable),
            ChangeTheme(AppThemes::Peach),
            true,
        ),
        panel_button(
            app,
            Materials::RimmedPlastic,
            if app.theme_selection == AppThemes::Midnight {
                MaterialColors::Accent
            } else {
                MaterialColors::Background
            },
            2,
            true,
            ButtonShapes::Minimal,
            ui_string(app, 1, AppThemes::Midnight.name(), TextSizes::Interactable),
            ChangeTheme(AppThemes::Midnight),
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}