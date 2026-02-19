use iced::{Color, Element, Size};
use iced::border::color;
use iced::futures::{FutureExt, TryFutureExt};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button::Status;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::palette::{ColorModes::*, ThemeColors};
use crate::vault::parse::*;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::container::signal::Signal::*;

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
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    StrongBackground,
    StrongPrimary,
    StrongSecondary,
    StrongSuccess,
    StrongWarning,
    StrongDanger,
    WeakBackground,
    WeakPrimary,
    WeakSecondary,
    WeakSuccess,
    WeakWarning,
    WeakDanger,
    Text,
    TextFor(TextStylingColorOptions),
    Other(Color),
}
impl StylingColors {
    /// Gets the theme color from the selection.
    /// Other is for colors that are not in the theme's palette.
    pub fn get_for(&self, theme: &Theme) -> Color {
        match self {
            StylingColors::Background =>       { theme.extended_palette().background.base.color }
            StylingColors::Primary =>          { theme.extended_palette().primary.base.color }
            StylingColors::Secondary =>        { theme.extended_palette().secondary.base.color }
            StylingColors::Success =>          { theme.extended_palette().success.base.color }
            StylingColors::Warning =>          { theme.extended_palette().warning.base.color }
            StylingColors::Danger =>           { theme.extended_palette().danger.base.color }
            StylingColors::StrongBackground => { theme.extended_palette().background.strong.color }
            StylingColors::StrongPrimary =>    { theme.extended_palette().primary.strong.color }
            StylingColors::StrongSecondary =>  { theme.extended_palette().secondary.strong.color }
            StylingColors::StrongSuccess =>    { theme.extended_palette().success.strong.color }
            StylingColors::StrongWarning =>    { theme.extended_palette().warning.strong.color }
            StylingColors::StrongDanger =>     { theme.extended_palette().danger.strong.color }
            StylingColors::WeakBackground =>   { theme.extended_palette().background.weak.color }
            StylingColors::WeakPrimary =>      { theme.extended_palette().primary.weak.color }
            StylingColors::WeakSecondary =>    { theme.extended_palette().secondary.weak.color }
            StylingColors::WeakSuccess =>      { theme.extended_palette().success.weak.color }
            StylingColors::WeakWarning =>      { theme.extended_palette().warning.weak.color }
            StylingColors::WeakDanger =>       { theme.extended_palette().danger.weak.color }
            StylingColors::Text =>             { theme.palette().text }
            StylingColors::TextFor(coloring) => {
                match coloring {
                    TextStylingColorOptions::Background =>       { theme.extended_palette().background.base.text }
                    TextStylingColorOptions::Primary =>          { theme.extended_palette().primary.base.text }
                    TextStylingColorOptions::Secondary =>        { theme.extended_palette().secondary.base.text }
                    TextStylingColorOptions::Success =>          { theme.extended_palette().success.base.text }
                    TextStylingColorOptions::Warning =>          { theme.extended_palette().warning.base.text }
                    TextStylingColorOptions::Danger =>           { theme.extended_palette().danger.base.text }
                    TextStylingColorOptions::StrongBackground => { theme.extended_palette().background.strong.text }
                    TextStylingColorOptions::StrongPrimary =>    { theme.extended_palette().primary.strong.text }
                    TextStylingColorOptions::StrongSecondary =>  { theme.extended_palette().secondary.strong.text }
                    TextStylingColorOptions::StrongSuccess =>    { theme.extended_palette().success.strong.text }
                    TextStylingColorOptions::StrongWarning =>    { theme.extended_palette().warning.strong.text }
                    TextStylingColorOptions::StrongDanger =>     { theme.extended_palette().danger.strong.text }
                    TextStylingColorOptions::WeakBackground =>   { theme.extended_palette().background.weak.text }
                    TextStylingColorOptions::WeakPrimary =>      { theme.extended_palette().primary.weak.text }
                    TextStylingColorOptions::WeakSecondary =>    { theme.extended_palette().secondary.weak.text }
                    TextStylingColorOptions::WeakSuccess =>      { theme.extended_palette().success.weak.text }
                    TextStylingColorOptions::WeakWarning =>      { theme.extended_palette().warning.weak.text }
                    TextStylingColorOptions::WeakDanger =>       { theme.extended_palette().danger.weak.text }
                    TextStylingColorOptions::Other => { theme.palette().text }
                }
            }
            StylingColors::Other(color) => { color.clone() }
        }
    }
}

/// Used in StylingColors::TextFor to reflect which color the text will be placed on top of.
pub enum TextStylingColorOptions {
    Background,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    StrongBackground,
    StrongPrimary,
    StrongSecondary,
    StrongSuccess,
    StrongWarning,
    StrongDanger,
    WeakBackground,
    WeakPrimary,
    WeakSecondary,
    WeakSuccess,
    WeakWarning,
    WeakDanger,
    Other,
}
impl TextStylingColorOptions {
    pub fn from_styling_color(coloring: &StylingColors) -> TextStylingColorOptions {
        match coloring {
            StylingColors::Background => { TextStylingColorOptions::Background }
            StylingColors::Primary => { TextStylingColorOptions::Primary }
            StylingColors::Secondary => { TextStylingColorOptions::Secondary }
            StylingColors::Success => { TextStylingColorOptions::Success }
            StylingColors::Warning => { TextStylingColorOptions::Warning }
            StylingColors::Danger => { TextStylingColorOptions::Danger }
            StylingColors::StrongBackground => { TextStylingColorOptions::StrongBackground }
            StylingColors::StrongPrimary => { TextStylingColorOptions::StrongPrimary }
            StylingColors::StrongSecondary => { TextStylingColorOptions::StrongSecondary }
            StylingColors::StrongSuccess => { TextStylingColorOptions::StrongSuccess }
            StylingColors::StrongWarning => { TextStylingColorOptions::StrongWarning }
            StylingColors::StrongDanger => { TextStylingColorOptions::StrongDanger }
            StylingColors::WeakBackground => { TextStylingColorOptions::WeakBackground }
            StylingColors::WeakPrimary => { TextStylingColorOptions::WeakPrimary }
            StylingColors::WeakSecondary => { TextStylingColorOptions::WeakSecondary }
            StylingColors::WeakSuccess => { TextStylingColorOptions::WeakSuccess }
            StylingColors::WeakWarning => { TextStylingColorOptions::WeakWarning }
            StylingColors::WeakDanger => { TextStylingColorOptions::WeakDanger }
            _ => TextStylingColorOptions::Other
        }
    }
}



// standard styles
/// Returns a rounded background style.
fn rounded_container_style(
    coloring: StylingColors,
) -> impl Fn(&Theme) -> container::Style {
    move |theme| container::Style {
        background: Some(coloring.get_for(theme).into()),
        border: iced::Border::default().rounded(CornerRadii::Medium.size()),
        text_color: Some(StylingColors::TextFor(TextStylingColorOptions::from_styling_color(&coloring)).get_for(theme)),
        ..Default::default()
    }
}

/// Returns standard button style.
fn button_style(
    coloring: StylingColors,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| button::Style {
        background: Some(match status {
            Status::Active => { coloring.get_for(theme).into() }
            Status::Hovered => { StylingColors::Success.get_for(theme).into() }
            Status::Pressed => { coloring.get_for(theme).into() }
            Status::Disabled => { coloring.get_for(theme).into() }
        }),
        border: iced::Border::default().rounded(CornerRadii::Medium.size()),
        text_color: StylingColors::TextFor(TextStylingColorOptions::from_styling_color(&coloring)).get_for(theme),
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
        .style(rounded_container_style(coloring))
}



// bank overview parts
pub fn transaction_list<'a>(
    app: &App,
    transactions: Vec<&Transaction>,
    value_display_format: ValueDisplayFormats,
)  -> Scrollable<'a, Signal> {
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();
    for i in 0..transactions.len() {
        let transaction = &transactions[i];
        if i % 2 == 0 { first_half.push(transaction); }
        else { second_half.push(transaction); }
    }

    scrollable(
        row![
            column(first_half.into_iter().map(|transaction| {
                transaction_panel(app, transaction).into()
            }))
            .spacing(PaddingSizes::Small.size()),

            space().width(PaddingSizes::Medium.size()),

            column(second_half.into_iter().map(|transaction| {
                transaction_panel(app, transaction).into()
            }))
            .spacing(PaddingSizes::Small.size()),
        ]
    )
        .direction(Direction::Vertical(Scrollbar::hidden()))
}

pub fn transaction_panel<'a>(
    app: &App,
    transaction: &Transaction,
) -> Container<'a, Signal> {
    panel(
        StylingColors::Primary,
        CornerRadii::Medium,
        PaddingSizes::Medium,
        {
            column![
                row![
                    standard_text(TextSizes::SmallHeading, StylingColors::Text, transaction.value.to_string()),
                    space().width(PaddingSizes::Large.size()),
                    standard_text(TextSizes::Body, StylingColors::Text, transaction.date.display()),
                    space().width(PaddingSizes::Large.size()),
                    space::horizontal(),
                    edit_transaction_button(transaction),
                ],

                row![
                    standard_text(TextSizes::Body, StylingColors::Text, transaction.description.display(TagStyles::Lowercase)),
                    space::horizontal(),
                ],

                row(transaction.tags.iter().map(|tag| {
                    tag_panel(app, tag, app.bank.tag_registry.get(&tag).unwrap_or(ThemeColors::Aqua)).into()
                }))
                .spacing(PaddingSizes::Small.size()),
            ]
                .spacing(PaddingSizes::Small.size())
        }.into()
    )
}

pub fn edit_transaction_button<'a>(
    transaction: &Transaction,
) -> Button<'a, Signal> {
    button("Edit")
        .on_press(StartEditingTransaction(transaction.get_id().expect("Tried to edit a transaction without an id!")))
        .style(button_style(StylingColors::StrongPrimary))
}

pub fn tag_panel<'a, Signal: 'a>(
    app: &App,
    tag: &Tag,
    color: ThemeColors,
) -> Container<'a, Signal> {
    panel(
        StylingColors::Other(color.at(app.theme_selection.color_mode())),
        CornerRadii::Medium,
        PaddingSizes::Small,
        {
            standard_text(TextSizes::Interactable, StylingColors::Text, tag.display(TagStyles::Lowercase))
        }.into()
    )
}

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
                        standard_text(
                            TextSizes::Interactable,
                            StylingColors::Text,
                            value.to_string(),  // todo create standard function to format values (with currency)
                        ).into()
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
                        standard_text(
                            TextSizes::Interactable,
                            StylingColors::Text,
                            Transaction::get_time_price(&value, price).to_string(), // todo create standard function to format values (with currency)
                        ).into()
                    })).into()
                }
            )
        }
    }
}