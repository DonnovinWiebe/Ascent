use iced::{Center, Fill, Length};
use iced::{Color, Element, Size};
use iced::advanced::Widget;
use iced::border::color;
use iced::futures::{FutureExt, TryFutureExt};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button::Status;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::material::{MaterialColors, AppThemes, Materials};
use crate::vault::parse::*;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::container::signal::Signal::*;

// modes
/// The different modes that a date picker can be in.
#[derive(Debug, Clone)]
pub enum DatePickerModes {
    Hidden,
    ShowingMonthsInYear,
    ShowingDaysInMonth,
}

/// The difference ways individual transactions are managed.
#[derive(Debug, Clone)]
pub enum TransactionManagementTypes {
    Adding,
    Editing,
}



// standard parameters
/// Allows custom widgets to use standardized widths.
pub enum Widths {
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
            Widths::SmallCard => { 300.0 }
            Widths::MediumCard => { 500.0 }
            Widths::LargeCard => { 700.0 }
            Widths::SmallField => { 100.0 }
            Widths::MediumField => { 250.0 }
            Widths::LargeField => { 400.0 }
            Widths::Other(size) => { *size }
        }
    }
}

/// Allows custom widgets to use standardized widths.
pub enum Heights {
    SmallCard,
    MediumCard,
    LargeCard,
    Other(f32),
}
impl Heights {
    pub fn size(&self) -> f32 {
        match self {
            Heights::SmallCard => { 150.0 }
            Heights::MediumCard => { 325.0 }
            Heights::LargeCard => { 500.0 }
            Heights::Other(size) => { *size }
        }
    }
}

/// Allows custom widgets to use standardized padding.
pub enum PaddingSizes {
    None,
    Nano,
    Micro,
    Small,
    Medium,
    Large,
    Other(f32)
}
impl PaddingSizes {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            PaddingSizes::None => { 0.0 }
            PaddingSizes::Nano => { 2.0 }
            PaddingSizes::Micro => { 4.0 }
            PaddingSizes::Small => { 6.0 }
            PaddingSizes::Medium => { 10.0 }
            PaddingSizes::Large => { 16.0 }
            PaddingSizes::Other(size) => { *size }
        }
    }
}

/// Allows custom widgets to use standardized corner radius sizes.
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
            TextSizes::LargeHeading => { 22.0 }
            TextSizes::Interactable => { 16.0 }
            TextSizes::Custom(size) => { *size }
        }
    }
}



// standard styles
/// Returns a standard rounded background style.
pub fn rounded_container_style(
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
                    Materials::Plastic => { MaterialColors::Shadow.themed(&app.theme_selection, strength) }
                    Materials::RimmedPlastic => { MaterialColors::Shadow.themed(&app.theme_selection, strength) }
                    Materials::Acrylic => { color.themed(&app.theme_selection, strength) }
                }
            }
            else {
                Color::TRANSPARENT
            },
            offset: iced::Vector::new(1.0, 1.0),
            blur_radius: if cast_shadow { 3.0 } else { 0.0 },
        },
        text_color: Some(MaterialColors::Text.themed(&app.theme_selection, 1)),
        snap: false,
    }
}

/// Returns standard button style.
pub fn button_style(
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
                    Materials::Plastic => { MaterialColors::Shadow.themed(&app.theme_selection, strength) }
                    Materials::RimmedPlastic => { MaterialColors::Shadow.themed(&app.theme_selection, strength) }
                    Materials::Acrylic => { color.themed(&app.theme_selection, strength) }
                }
            }
            else {
                Color::TRANSPARENT
            },
            offset: iced::Vector::new(1.0, 1.0),
            blur_radius: if cast_shadow { 3.0 } else { 0.0 },
        },
        text_color: MaterialColors::Text.themed(&app.theme_selection, 1),
        snap: false,
    }
}

/// Returns a standard text input style.
pub fn text_input_style(
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
/// A standard text widget.
pub fn standard_text(
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
    width: Option<Widths>,
    height: Option<Heights>,
    internal_padding: PaddingSizes,
    content: Element<'a, Signal>,
) -> Element<'a, Signal> {
    container(
        container(content)
            .padding(internal_padding.size())
            .style(rounded_container_style(app, material, color, strength, cast_shadow))
            .width(if let Some(width) = width { Length::Fixed(width.size()) } else { Length::Shrink })
            .height(if let Some(height) = height { Length::Fixed(height.size()) } else { Length::Shrink })
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
        .style(button_style(app, material, color, strength, cast_shadow))
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
        Some(width),
        None,
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
    let mut additional_content = additional_content;
    additional_content.insert(0, space().width(PaddingSizes::Large.size()).into());
    additional_content.push(space().width(PaddingSizes::Large.size()).into());

    column![
        space().height(PaddingSizes::Large.size()),

        row![
            space().width(PaddingSizes::Large.size()),
            space::horizontal(),

            home_button(app, can_go_home),

            space().width(PaddingSizes::Large.size()),

            panel(
                app,
                Materials::Acrylic,
                MaterialColors::Background,
                4,
                true,
                None,
                None,
                PaddingSizes::Medium, {
                    standard_text(app, 1, title.to_string(), TextSizes::LargeHeading)
                }

            ),

            space().width(PaddingSizes::Large.size()),

            cycle_theme_button(app),

            space::horizontal(),
            space().width(PaddingSizes::Large.size()),

        ]
        .align_y(Center)
        .width(Fill),

        row(additional_content)
        .align_y(Center)
        .spacing(PaddingSizes::Large.size()),

        space::vertical(),
    ]
        .spacing(PaddingSizes::Small.size())
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
        fa_icon_solid("palette"),
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