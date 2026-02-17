use iced::{Color, Element, Size};
use iced::border::color;
use iced::futures::{FutureExt, TryFutureExt};
use iced::widget::*;
use crate::container::signal::Signal;
use crate::ui::palette::{ColorModes::*, ThemeColors};
use crate::vault::parse::*;
use crate::vault::transaction::{Transaction, ValueDisplayFormats};

// standards
/// Allows custom widgets use standardized padding.
pub enum PaddingSizes {
    Small,
    Medium,
    Large,
}
impl PaddingSizes {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            PaddingSizes::Small => { 6.0 }
            PaddingSizes::Medium => { 10.0 }
            PaddingSizes::Large => { 14.0 }
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
            CornerRadii::Small => { 12.0 }
            CornerRadii::Medium => { 16.0 }
            CornerRadii::Large => { 20.0 }
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

/// Allows custom widgets to be easily colored based on the app's theme.
pub enum StylingColors {
    Background,
    Text,
    Primary,
    Success,
    Warning,
    Danger,
    Other(Color),
}
impl StylingColors {
    /// Gets the theme color from the selection.
    /// Other is for colors that are not in the theme's palette.
    pub fn get_for(&self, theme: &Theme) -> Color {
        match self {
            StylingColors::Background => { theme.palette().background }
            StylingColors::Text => { theme.palette().text }
            StylingColors::Primary => { theme.palette().primary }
            StylingColors::Success => { theme.palette().success }
            StylingColors::Warning => { theme.palette().warning }
            StylingColors::Danger => { theme.palette().danger }
            StylingColors::Other(color) => { color.clone() }
        }
    }
}



// standard styles
/// Returns a rounded background Style.
fn rounded_background_style(
    coloring: StylingColors,
    corner_radius: CornerRadii,
) -> impl Fn(&Theme) -> container::Style {
    move |theme| container::Style {
        background: Some(coloring.get_for(theme).into()),
        border: iced::Border::default().rounded(corner_radius.size()),
        text_color: theme.palette().text.into(),
        ..Default::default()
    }
}



// standard parts
/// A standard text widget.
pub fn standard_text<'a>(
    size: TextSizes,
    coloring: StylingColors,
    text: String,
) -> Text<'a> {
    Text::new(text)
        .size(size.size())
        .style(move |theme| {
            text::Style { color: Some(coloring.get_for(&theme)) }
        }).into()
}

/// A standard box with rounded corners
pub fn panel<'a, Signal: 'a>(
    coloring: StylingColors,
    corner_radius: CornerRadii,
    internal_padding: PaddingSizes,
    content: Element<'a, Signal>,
) -> Container<'a, Signal> {
    Container::new(content)
        .padding(internal_padding.size())
        .style(rounded_background_style(coloring, corner_radius))
}



// bank overview parts
/// Returns a cash flow panel.
pub fn cash_flow_panel<'a, Signal: 'a>(
    cash_flow: &CashFlow,
    value_display_format: ValueDisplayFormats
) -> Container<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            panel(
                StylingColors::Primary,
                CornerRadii::Medium,
                PaddingSizes::Medium,
                {
                    column(cash_flow.value_flows.iter().map(|value| {
                        text(value.to_string()).into() // todo create standard function to format values (with currency)
                    })).into()
                }
            )
        }

        ValueDisplayFormats::Time(price) => {
            panel(
                StylingColors::Primary,
                CornerRadii::Medium,
                PaddingSizes::Medium,
                {
                    column(cash_flow.value_flows.iter().map(|value| {
                        text(Transaction::get_time_price(&value, price)).into() // todo create standard function to format time prices
                    })).into()
                }
            )
        }
    }
}