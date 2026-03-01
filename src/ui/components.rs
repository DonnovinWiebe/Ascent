use iced::{Center, Length};
use iced::{Color, Element};
use iced::advanced::Widget;
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button::Status;
use iced_font_awesome::fa_icon_solid;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::material::{MaterialColors, Materials};
use crate::container::signal::Signal::*;

// modes
/// The different modes that a date picker can be in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatePickerModes {
    Hidden,
    ShowingMonthsInYear,
    ShowingDaysInMonth,
}

/// The difference ways individual transactions are managed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionManagementTypes {
    Adding,
    Editing,
}



// standard parameters
/// Allows custom widgets to use standardized widths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Widths {
    Shrink,
    Fill,
    MicroCard,
    SmallCard,
    MediumCard,
    LargeCard,
    SmallField,
    MediumField,
    LargeField,
    Other(f32),
}
impl Widths {
    pub fn size(&self) -> f32 {
        match self {
            Widths::Shrink => { 1.0 }
            Widths::Fill => { 1.0 }
            Widths::MicroCard => { 175.0 }
            Widths::SmallCard => { 350.0 }
            Widths::MediumCard => { 550.0 }
            Widths::LargeCard => { 750.0 }
            Widths::SmallField => { 100.0 }
            Widths::MediumField => { 250.0 }
            Widths::LargeField => { 400.0 }
            Widths::Other(size) => { *size }
        }
    }
}

/// Allows custom widgets to use standardized widths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heights {
    Shrink,
    Fill,
    MicroCard,
    SmallCard,
    MediumCard,
    LargeCard,
    Other(f32),
}
impl Heights {
    pub fn size(&self) -> f32 {
        match self {
            Heights::Shrink => { 1.0 }
            Heights::Fill => { 1.0 }
            Heights::MicroCard => { 100.0 }
            Heights::SmallCard => { 200.0 }
            Heights::MediumCard => { 350.0 }
            Heights::LargeCard => { 500.0 }
            Heights::Other(size) => { *size }
        }
    }
}

/// Allows custom widgets to use standardized padding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaddingSizes {
    None,
    Nano,
    Micro,
    Small,
    Medium,
    Large,
    Ginormous,
    Other(f32)
}
impl PaddingSizes {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            PaddingSizes::None => { 0.0 }
            PaddingSizes::Nano => { 2.0 }
            PaddingSizes::Micro => { 4.0 }
            PaddingSizes::Small => { 8.0 }
            PaddingSizes::Medium => { 16.0 }
            PaddingSizes::Large => { 24.0 }
            PaddingSizes::Ginormous => { 36.0 }
            PaddingSizes::Other(size) => { *size }
        }
    }
}

/// Allows custom spacing between widgets.
/// This mirrors Padding Sizes, but in a more fitting name.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing {
    Fill,
    None,
    Nano,
    Micro,
    Small,
    Medium,
    Large,
    Ginormous,
    Other(f32),
    HeaderSpace,
}
impl Spacing {
    pub fn size(&self) -> f32 {
        match self {
            Spacing::Fill => { PaddingSizes::None.size() }
            Spacing::None => { PaddingSizes::None.size() }
            Spacing::Nano => { PaddingSizes::Nano.size() }
            Spacing::Micro => { PaddingSizes::Micro.size() }
            Spacing::Small => { PaddingSizes::Small.size() }
            Spacing::Medium => { PaddingSizes::Medium.size() }
            Spacing::Large => { PaddingSizes::Large.size() }
            Spacing::Ginormous => { PaddingSizes::None.size() }
            Spacing::Other(size) => { PaddingSizes::Other(*size).size() }
            Spacing::HeaderSpace => { 90.0 }
        }
    }
}

/// Allows orientation in various custom widget fields.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientations {
    Horizontal,
    Vertical,
}

/// Allows custom widgets to use standardized corner radius sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CornerRadii {
    Small,
    Medium,
    Large,
}
impl CornerRadii {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            CornerRadii::Small => { 8.0 }
            CornerRadii::Medium => { 12.0 }
            CornerRadii::Large => { 16.0 }
        }
    }
}

/// Allows custom widgets to use standardized corner radius sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderThickness {
    Disabled,
    Thin,
    Standard,
    Thick,
}
impl BorderThickness {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            BorderThickness::Disabled => { 0.0 }
            BorderThickness::Thin => { 2.0 }
            BorderThickness::Standard => { 4.0 }
            BorderThickness::Thick => { 6.0 }
        }
    }
}

/// Allows custom text widgets to use standardized text sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextSizes {
    Footnote,
    Body,
    SmallHeading,
    LargeHeading,
    Interactable,
    Custom(f32),
}
impl TextSizes {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            TextSizes::Footnote => { 12.0 }
            TextSizes::Body => { 14.0 }
            TextSizes::SmallHeading => { 18.0 }
            TextSizes::LargeHeading => { 24.0 }
            TextSizes::Interactable => { 16.0 }
            TextSizes::Custom(size) => { *size }
        }
    }
}



// standard styles
/// Returns a standard rounded background style.
fn panel_container_style(
    app: &App,
    material: Materials,
    color: MaterialColors,
    strength: u32,
    cast_shadow: bool,
) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(color.materialized(material, &app.theme_selection, strength).into()),
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(
                match material {
                    Materials::Plastic => { BorderThickness::Disabled.size() }
                    Materials::RimmedPlastic => { BorderThickness::Standard.size() }
                    Materials::Acrylic => { BorderThickness::Thin.size() }
                }
            )
            .color(color.materialized(material, &app.theme_selection, strength + 1)),
        shadow: iced::Shadow {
            color: if cast_shadow {
                match material {
                    Materials::Plastic => { MaterialColors::Shadow.as_shadow(&app.theme_selection, strength) }
                    Materials::RimmedPlastic => { MaterialColors::Shadow.as_shadow(&app.theme_selection, strength) }
                    Materials::Acrylic => { color.as_shadow(&app.theme_selection, strength) }
                }
            }
            else {
                Color::TRANSPARENT
            },
            offset: iced::Vector::new(1.0, 1.0),
            blur_radius: if cast_shadow { 4.0 } else { 0.0 },
        },
        text_color: Some(MaterialColors::Text.themed(&app.theme_selection, 1)),
        snap: false,
    }
}

/// Returns standard button style.
fn panel_button_style(
    app: &App,
    material: Materials,
    color: MaterialColors,
    strength: u32,
    cast_shadow: bool,
) -> impl Fn(&Theme, Status) -> button::Style {
    move |_, status| button::Style {
        background: Some(match status {
            Status::Active => { color.materialized(material, &app.theme_selection, strength).into() }
            Status::Hovered => { color.materialized(material, &app.theme_selection, strength + 1).into() }
            Status::Pressed => { MaterialColors::Unavailable.materialized(material, &app.theme_selection, strength).into() }
            Status::Disabled => { MaterialColors::Unavailable.materialized(material, &app.theme_selection, strength).into() }
        }),
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(
                match material {
                    Materials::Plastic => { BorderThickness::Disabled.size() }
                    Materials::RimmedPlastic => { BorderThickness::Thin.size() }
                    Materials::Acrylic => { BorderThickness::Thin.size() }
                }
            )
            .color(match status {
                Status::Active => { color.materialized(material, &app.theme_selection, strength + 1) }
                Status::Hovered => { color.materialized(material, &app.theme_selection, strength + 1) }
                Status::Pressed => { MaterialColors::Unavailable.materialized(material, &app.theme_selection, strength + 1) }
                Status::Disabled => { MaterialColors::Unavailable.materialized(material, &app.theme_selection, strength + 1) }
            }),
        shadow: iced::Shadow {
            color: if cast_shadow {
                match material {
                    Materials::Plastic => { MaterialColors::Shadow.as_shadow(&app.theme_selection, strength) }
                    Materials::RimmedPlastic => { MaterialColors::Shadow.as_shadow(&app.theme_selection, strength) }
                    Materials::Acrylic => { color.as_shadow(&app.theme_selection, strength) }
                }
            }
            else {
                Color::TRANSPARENT
            },
            offset: iced::Vector::new(1.0, 1.0),
            blur_radius: if cast_shadow { 4.0 } else { 0.0 },
        },
        text_color: MaterialColors::Text.themed(&app.theme_selection, 1),
        snap: false,
    }
}

/// Returns a standard text input style.
fn text_input_style(
    app: &App,
    material: Materials,
    color: MaterialColors,
    strength: u32,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_, status| text_input::Style {
        background: match status {
            text_input::Status::Active => { color.materialized(material, &app.theme_selection, strength).into() }
            text_input::Status::Hovered => { color.materialized(material, &app.theme_selection, strength + 1).into() }
            text_input::Status::Focused { is_hovered: false } => { color.materialized(material, &app.theme_selection, strength + 1).into() }
            text_input::Status::Focused { is_hovered: true } => { color.materialized(material, &app.theme_selection, strength + 1).into() }
            text_input::Status::Disabled => { MaterialColors::Unavailable.materialized(material, &app.theme_selection, strength).into() }
        },
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Thin.size())
            .color(match status {
                text_input::Status::Active => { color.themed(&app.theme_selection, strength + 1) }
                text_input::Status::Hovered => { color.themed(&app.theme_selection, strength + 1) }
                text_input::Status::Focused { is_hovered: false } => { color.themed(&app.theme_selection, strength + 1) }
                text_input::Status::Focused { is_hovered: true } => { color.themed(&app.theme_selection, strength + 1) }
                text_input::Status::Disabled => { MaterialColors::Unavailable.themed(&app.theme_selection, strength + 1) }
            }),
        icon: MaterialColors::Accent.themed(&app.theme_selection, 1),
        placeholder: MaterialColors::Text.themed(&app.theme_selection, 2),
        value: MaterialColors::Text.themed(&app.theme_selection, 1),
        selection: MaterialColors::Accent.themed(&app.theme_selection, 1),
    }
}



// standard ui components
/// A standard spacing widget.
pub fn spacer<'a>(
    orientation: Orientations,
    size: Spacing,
) -> Element<'a, Signal> {
    match orientation {
        Orientations::Horizontal => {
            match size{
                Spacing::Fill => { space::horizontal().into() }
                _ => { space().width(size.size()).into() }
            }
        }
        Orientations::Vertical => {
            match size{
                Spacing::Fill => { space::vertical().into() }
                _ => { space().height(size.size()).into() }
            }
        }
    }
}

/// A standard text widget.
pub fn ui_string(
    app: &App,
    strength: u32,
    text: String,
    size: TextSizes,
) -> Element<Signal> {
    Text::new(text)
        .size(size.size())
        .style(move |_theme| {
            text::Style { color: Some(MaterialColors::Text.themed(&app.theme_selection, strength)) }
        }).into()
}

/// A standard box with rounded corners.
pub fn panel<'a>(
    app: &'a App,
    material: Materials,
    color: MaterialColors,
    strength: u32,
    cast_shadow: bool,
    width: Widths,
    height: Heights,
    internal_padding: PaddingSizes,
    content: Element<'a, Signal>,
) -> Element<'a, Signal> {
    container(
        container(content)
            .padding(internal_padding.size())
            .style(panel_container_style(app, material, color, strength, cast_shadow))
            .width(match width {
                Widths::Shrink => { Length::Shrink }
                Widths::Fill => { Length::Fill }
                _ => { Length::Fixed(width.size()) }
            })
            .height(match height {
                Heights::Shrink => { Length::Shrink }
                Heights::Fill => { Length::Fill }
                _ => { Length::Fixed(height.size()) }
            })
    )
        .padding(PaddingSizes::Micro.size())
        .into()
}

/// A standard button with rounded corners.
pub fn panel_button<'a>(
    app: &'a App,
    material: Materials,
    color: MaterialColors,
    strength: u32,
    cast_shadow: bool,
    label: impl Into<Element<'a, Signal, Theme, Renderer>>,
    signal: Signal,
    active: bool,
) -> Element<'a, Signal> {
    let button = button(label)
        .style(panel_button_style(app, material, color, strength, cast_shadow))
        .padding([PaddingSizes::Small.size(), PaddingSizes::Large.size()]);

    container(
        if active { button.on_press(signal) } else { button }
    )
        .padding(PaddingSizes::Micro.size())
        .into()
}

/// A standard text input panel with rounded corners.
pub fn panel_text_input<'a>(
    app: &'a App,
    material: Materials,
    color: MaterialColors,
    strength: u32,
    cast_shadow: bool,
    width: Widths,
    placeholder: &str,
    value: &str,
    on_change: fn(String) -> Signal,
) -> Element<'a, Signal> {
    panel(
        app,
        material,
        color,
        strength,
        cast_shadow,
        width,
        Heights::Shrink,
        PaddingSizes::None, {
        text_input(placeholder, value)
            .style(text_input_style(app, material, color, strength))
            .on_input(on_change)
            .into()
    })
}



// standard app widgets
/// A header for every page.
pub fn header<'a>(
    app: &'a App,
    can_go_home: bool,
    additional_content: Vec<Element<'a, Signal>>,
) -> Element<'a, Signal> {
    let title = app.page.name();

    column![
        panel(
            app,
            Materials::Acrylic,
            MaterialColors::Background,
            3,
            true,
            Widths::Fill,
            Heights::Shrink,
            PaddingSizes::Small, {
                row![
                    spacer(Orientations::Horizontal, Spacing::Fill),

                    home_button(app, can_go_home),

                    panel(
                        app,
                        Materials::Acrylic,
                        MaterialColors::Background,
                        4,
                        true,
                        Widths::Shrink,
                        Heights::Shrink,
                        PaddingSizes::Small, {
                            row![
                                spacer(Orientations::Horizontal, Spacing::Medium),
                                ui_string(app, 1, title.to_string(), TextSizes::LargeHeading),
                                spacer(Orientations::Horizontal, Spacing::Medium),
                            ]
                            .spacing(Spacing::None.size())
                            .into()
                        }
                    ),

                    cycle_theme_button(app),

                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .spacing(Spacing::Large.size())
                .align_y(Center)
                .into()
            }
        ),

        row(additional_content)
        .align_y(Center)
        .spacing(Spacing::Large.size()),

        space::vertical(),
    ]
        .spacing(Spacing::Micro.size())
        .padding(PaddingSizes::Micro.size())
        .into()
}

/// Brings the user back to the home page (transactions).
pub fn home_button(
    app: &App,
    active: bool,
) -> Element<Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Accent,
        1,
        true,
        fa_icon_solid("house"),
        GoHome,
        active,
    )
}

/// Cycles the app theme.
pub fn cycle_theme_button(
    app: &App,
) -> Element<Signal> {
    panel_button(
        app,
        Materials::RimmedPlastic,
        MaterialColors::Accent,
        1,
        true,
        fa_icon_solid("palette"),
        CycleTheme,
        true,
    )
}