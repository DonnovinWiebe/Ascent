use iced::Alignment::Center;
use iced::Fill;
use iced::Element;
use iced::widget::{Stack, container, stack, row};
use iced::widget::column;
use crate::container::app::{App, Pages};
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, panel, panel_button, spacer, ui_string};
use crate::ui::material::{MaterialColors, MaterialStyle, Materials};
use iced_font_awesome::fa_icon_solid as icon;

/// The page used to inform users of what each page is for and how to use it.
#[must_use]
pub fn help_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(help_panel(app)).center(Fill),
        header(app, Vec::new()),
    ]
}

/// Displays help information for the current page.
#[must_use]
fn help_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
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
                ui_string(app, 1, "Help!".to_string(), TextSizes::LargeHeading),
                
                spacer(Orientations::Vertical, Spacing::Large),
                ui_string(app, 1, get_page_info(app), TextSizes::SmallHeading),
                
                spacer(Orientations::Vertical, Spacing::Medium),
                column(get_page_keybinds(app)).spacing(0),
                
                spacer(Orientations::Vertical, Spacing::Large),
                dismiss_help_button(app)
            ]
            .width(Fill)
            .align_x(Center)
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// Returns information about the current page.
#[must_use]
fn get_page_info(app: &App) -> String {
    match app.page {
        Pages::Transactions => "This page lists all your transactions.\n\nThese transactions can be filtered by date, tag, and search term, either requiring a full filter match or a partial match.\nThe ring chart and cash flow display then show how money is spent and earned.".to_string(),
        
        Pages::AddingTransaction => "This page allows you to add a new transaction.".to_string(),
        
        Pages::EditingTransaction => "This page allows you to edit an existing transaction.".to_string(),
        
        Pages::TagRegistry => "This page allows you to select the color for each tag.".to_string(),
        
        Pages::Settings => "This page allows you to configure your application settings.".to_string(),
        
        Pages::ConfirmImport => "This page is used to import data from a backup file.\n\nPlease note that importing from a backup file will overwrite any existing data\nincluding transactions, tag coloring, and any other data.".to_string(),
        
        Pages::ConfirmLegacyImport => "This page is used to import legacy data from a legacy backup file.\n\nPlease note that importing from a legacy backup file will overwrite all existing transactions.".to_string(),
    }
}

/// Returns the keybinds for the current page.
#[must_use]
fn get_page_keybinds<'a>(app: &'a App) -> Vec<Element<'a, Signal>> {
    match app.page {
        Pages::Transactions => vec![
            Keybind::new("Add Transaction", KeybindKeys::StandardKey('a'), vec![KeybindModifiers::Command]).widget(app),
            Keybind::new("Advance Filter Year", KeybindKeys::ArrowKey(ArrowKeys::Right), vec![KeybindModifiers::Shift]).widget(app),
            Keybind::new("Recede Filter Year", KeybindKeys::ArrowKey(ArrowKeys::Left), vec![KeybindModifiers::Shift]).widget(app),
            Keybind::new("Advance Filter Month", KeybindKeys::ArrowKey(ArrowKeys::Right), vec![]).widget(app),
            Keybind::new("Recede Filter Month", KeybindKeys::ArrowKey(ArrowKeys::Left), vec![]).widget(app),
        ],
        
        Pages::AddingTransaction => vec![
            Keybind::new("Add Transaction", KeybindKeys::StandardKey('a'), vec![KeybindModifiers::Command]).widget(app),
            Keybind::new("Advance Year", KeybindKeys::ArrowKey(ArrowKeys::Right), vec![KeybindModifiers::Shift]).widget(app),
            Keybind::new("Recede Year", KeybindKeys::ArrowKey(ArrowKeys::Left), vec![KeybindModifiers::Shift]).widget(app),
            Keybind::new("Advance Month", KeybindKeys::ArrowKey(ArrowKeys::Right), vec![]).widget(app),
            Keybind::new("Recede Month", KeybindKeys::ArrowKey(ArrowKeys::Left), vec![]).widget(app),
            Keybind::new("Advance Day", KeybindKeys::ArrowKey(ArrowKeys::Up), vec![]).widget(app),
            Keybind::new("Recede Day", KeybindKeys::ArrowKey(ArrowKeys::Down), vec![]).widget(app),
        ],
        
        Pages::EditingTransaction => vec![
            Keybind::new("Edit Transaction", KeybindKeys::StandardKey('a'), vec![KeybindModifiers::Command]).widget(app),
            Keybind::new("Advance Year", KeybindKeys::ArrowKey(ArrowKeys::Right), vec![KeybindModifiers::Shift]).widget(app),
            Keybind::new("Recede Year", KeybindKeys::ArrowKey(ArrowKeys::Left), vec![KeybindModifiers::Shift]).widget(app),
            Keybind::new("Advance Month", KeybindKeys::ArrowKey(ArrowKeys::Right), vec![]).widget(app),
            Keybind::new("Recede Month", KeybindKeys::ArrowKey(ArrowKeys::Left), vec![]).widget(app),
            Keybind::new("Advance Day", KeybindKeys::ArrowKey(ArrowKeys::Up), vec![]).widget(app),
            Keybind::new("Recede Day", KeybindKeys::ArrowKey(ArrowKeys::Down), vec![]).widget(app),
        ],
        
        Pages::TagRegistry => vec![],
        
        Pages::Settings => vec![],
        
        Pages::ConfirmImport => vec![],
        
        Pages::ConfirmLegacyImport => vec![],
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Keybind {
    action: String,
    key: KeybindKeys,
    modifiers: Vec<KeybindModifiers>,
}
impl Keybind {
    fn new(action: &str, key: KeybindKeys, modifiers: Vec<KeybindModifiers>) -> Keybind {
        Keybind { action: action.to_string(), key, modifiers }
    }
    
    fn widget<'a>(self, app: &'a App) -> Element<'a, Signal> {
        if self.modifiers.is_empty() {
            row![
                self.key.paneled(app),
                
                spacer(Orientations::Horizontal, Spacing::Medium),
                ui_string(app, 1, self.action.to_string(), TextSizes::SmallHeading),
            ]
            .align_y(Center)
            .spacing(0)
            .into()
        }
        
        else {
            row![
                row(self.modifiers.into_iter().map(|m| m.paneled(app)))
                    .align_y(Center)
                    .spacing(Spacing::Micro.size()),
                self.key.paneled(app),
                
                spacer(Orientations::Horizontal, Spacing::Medium),
                ui_string(app, 1, self.action.to_string(), TextSizes::SmallHeading),
            ]
            .align_y(Center)
            .spacing(0)
            .into()
        }
    }
}

/// Enumerates the modifiers that can be used in keybinds.
/// This is only used to help display keybinds.
#[allow(dead_code)] // I want to keep the unused variants for future use
#[derive(Debug, Clone, PartialEq)]
enum KeybindModifiers {
    Command,
    Control,
    Shift,
}
impl KeybindModifiers {
    #[must_use]
    fn named(&self) -> String {
        match self {
            KeybindModifiers::Command => {
                #[cfg(target_os = "macos")]
                { "Cmd".to_string() }
                #[cfg(not(target_os = "macos"))]
                { "Ctrl".to_string() }
            },
            KeybindModifiers::Control => "Ctrl".to_string(),
            KeybindModifiers::Shift => "Shift".to_string(),
        }
    }
    
    fn paneled<'a>(self, app: &'a App) -> Element<'a, Signal> {
        panel(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Background,
                strength: 3,
                cast_shadow: true,
            },
            PanelSize { width: Widths::Shrink, height: Heights::Shrink },
            PaddingSizes::Micro,
            ui_string(app, 1, self.named(), TextSizes::SmallHeading)
        )
    }
}

#[allow(dead_code)] // I want to keep the unused variants for future use
#[derive(Debug, Clone, PartialEq)]
enum KeybindKeys {
    StandardKey(char),
    ArrowKey(ArrowKeys),
    Tab,
    Return,
    Backspace,
    Space,
    Escape,
}
impl KeybindKeys {
    fn named(&self) -> String {
        match self {
            KeybindKeys::StandardKey(c) => c.to_string().to_uppercase(),
            KeybindKeys::ArrowKey(_) => "icon".to_string(),
            KeybindKeys::Tab => "Tab".to_string(),
            KeybindKeys::Return => "Return".to_string(),
            KeybindKeys::Backspace => "Backspace".to_string(),
            KeybindKeys::Space => "Space".to_string(),
            KeybindKeys::Escape => "Escape".to_string(),
        }
    }
    
    fn paneled<'a>(self, app: &'a App) -> Element<'a, Signal> {
        if let KeybindKeys::ArrowKey(arrow_key) = self {
            let icon = match arrow_key {
                ArrowKeys::Up => icon("caret-up"),
                ArrowKeys::Down => icon("caret-down"),
                ArrowKeys::Left => icon("caret-left"),
                ArrowKeys::Right => icon("caret-right"),
            };
            
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 3,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                PaddingSizes::Micro,
                icon.into(),
            )
        }
        
        else {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::Background,
                    strength: 3,
                    cast_shadow: true,
                },
                PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                PaddingSizes::Micro,
                ui_string(app, 1, self.named(), TextSizes::SmallHeading)
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ArrowKeys {
    Up,
    Down,
    Left,
    Right,
}

/// A button that dismisses the help page.
#[must_use]
fn dismiss_help_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Success,
            strength: 1,
            cast_shadow: true,
        },
        ButtonShapes::Wide,
        ui_string(app, 1, "Dismiss".to_string(), TextSizes::Interactable),
        Signal::DontHelpMe,
        true,
    )
}

/// A button that displays the help page.
#[must_use]
pub fn help_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::RimmedPlastic,
            color: MaterialColors::Success,
            strength: 1,
            cast_shadow: true,
        },
        ButtonShapes::Wide,
        icon("question"),
        Signal::HelpMe,
        true,
    )
}