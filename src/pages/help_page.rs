use iced::Alignment::Center;
use iced::Fill;
use iced::Element;
use iced::widget::{Stack, container, stack, row};
use iced::widget::column;
use crate::container::app::{App, Pages};
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, panel, panel_button, spacer, ui_string};
use crate::ui::material::Depths;
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
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::MediumCard, height: Heights::Shrink },
        PaddingSizes::Medium, {
            column![
                ui_string(app, "Help!", TextSizes::LargeHeading, MaterialColors::StrongText),
                
                spacer(Orientations::Vertical, Spacing::Large),
                ui_string(app, get_page_info(app), TextSizes::SmallHeading, MaterialColors::MediumText),
                
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
    #[allow(clippy::match_same_arms)] // I want to keep these empty match arms for future use
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

/// Represents a keybind, including the action description, key, and key modifiers.
#[derive(Debug, Clone, PartialEq)]
struct Keybind {
    /// The description of the action performed by this keybind.
    action: String,
    /// The key that triggers this keybind.
    key: KeybindKeys,
    /// The modifiers that must be held down with the key to trigger this keybind.
    modifiers: Vec<KeybindModifiers>,
}
impl Keybind {
    /// Creates a new `Keybind` with the given action, key, and modifiers.
    #[must_use]
    fn new(action: &str, key: KeybindKeys, modifiers: Vec<KeybindModifiers>) -> Keybind {
        Keybind { action: action.to_string(), key, modifiers }
    }
    
    /// Turns the `Keybind` into a widget.
    #[must_use]
    fn widget<'a>(self, app: &'a App) -> Element<'a, Signal> {
        if self.modifiers.is_empty() {
            row![
                self.key.paneled(app),
                
                spacer(Orientations::Horizontal, Spacing::Medium),
                ui_string(app, &self.action, TextSizes::SmallHeading, MaterialColors::StrongText),
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
                ui_string(app, &self.action, TextSizes::SmallHeading, MaterialColors::StrongText),
            ]
            .align_y(Center)
            .spacing(0)
            .into()
        }
    }
}

/// Defines the modifiers that can be used in `Keybind`s.
#[allow(dead_code)] // I want to keep the unused variants for future use
#[derive(Debug, Clone, PartialEq)]
enum KeybindModifiers {
    Command,
    Control,
    Shift,
}
impl KeybindModifiers {
    /// Returns the name of the modifier as a `String`.
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
    
    /// Turns the modifier into a paneled element.
    #[must_use]
    fn paneled<'a>(self, app: &'a App) -> Element<'a, Signal> {
        panel(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            PanelSize { width: Widths::Shrink, height: Heights::Shrink },
            PaddingSizes::Micro,
            ui_string(app, self.named(), TextSizes::SmallHeading, MaterialColors::StrongText)
        )
    }
}

/// Defines the keys for `Keybind`s.
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
    /// Returns the name of the key as a `String`.
    #[must_use]
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
    
    /// Turns the key into a paneled element.
    #[must_use]
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
                    color: MaterialColors::Card,
                    depth: Depths::Proud,
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
                    depth: Depths::Proud,
                },
                PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                PaddingSizes::Micro,
                ui_string(app, self.named(), TextSizes::SmallHeading, MaterialColors::StrongText)
            )
        }
    }
}

/// Defines the arrow keys for `Keybind`s.
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
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        ui_string(app, "Dismiss", TextSizes::Interactable, MaterialColors::StrongText),
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
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        ButtonShapes::Minimal,
        icon("question"),
        Signal::HelpMe,
        true,
    )
}