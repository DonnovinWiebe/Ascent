use iced::{Color, Element, Size};
use iced::border::color;
use iced::futures::{FutureExt, TryFutureExt};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button::Status;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::palette::{Appearance::*, AppColors};
use crate::vault::parse::*;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::container::signal::Signal::*;

// standards
/// Allows custom widgets to use standardized padding.
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
    TextFor(TextStylingColors),
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
                    TextStylingColors::AboveBackground =>       { theme.extended_palette().background.base.text }
                    TextStylingColors::AbovePrimary =>          { theme.extended_palette().primary.base.text }
                    TextStylingColors::AboveSecondary =>        { theme.extended_palette().secondary.base.text }
                    TextStylingColors::AboveSuccess =>          { theme.extended_palette().success.base.text }
                    TextStylingColors::AboveWarning =>          { theme.extended_palette().warning.base.text }
                    TextStylingColors::AboveDanger =>           { theme.extended_palette().danger.base.text }
                    TextStylingColors::AboveStrongBackground => { theme.extended_palette().background.strong.text }
                    TextStylingColors::AboveStrongPrimary =>    { theme.extended_palette().primary.strong.text }
                    TextStylingColors::AboveStrongSecondary =>  { theme.extended_palette().secondary.strong.text }
                    TextStylingColors::AboveStrongSuccess =>    { theme.extended_palette().success.strong.text }
                    TextStylingColors::AboveStrongWarning =>    { theme.extended_palette().warning.strong.text }
                    TextStylingColors::AboveStrongDanger =>     { theme.extended_palette().danger.strong.text }
                    TextStylingColors::AboveWeakBackground =>   { theme.extended_palette().background.weak.text }
                    TextStylingColors::AboveWeakPrimary =>      { theme.extended_palette().primary.weak.text }
                    TextStylingColors::AboveWeakSecondary =>    { theme.extended_palette().secondary.weak.text }
                    TextStylingColors::AboveWeakSuccess =>      { theme.extended_palette().success.weak.text }
                    TextStylingColors::AboveWeakWarning =>      { theme.extended_palette().warning.weak.text }
                    TextStylingColors::AboveWeakDanger =>       { theme.extended_palette().danger.weak.text }
                    TextStylingColors::Standard => { theme.palette().text }
                }
            }
            StylingColors::Other(color) => { color.clone() }
        }
    }
}

/// Used in StylingColors::TextFor to reflect which color the text will be placed on top of.
pub enum TextStylingColors {
    AboveBackground,
    AbovePrimary,
    AboveSecondary,
    AboveSuccess,
    AboveWarning,
    AboveDanger,
    AboveStrongBackground,
    AboveStrongPrimary,
    AboveStrongSecondary,
    AboveStrongSuccess,
    AboveStrongWarning,
    AboveStrongDanger,
    AboveWeakBackground,
    AboveWeakPrimary,
    AboveWeakSecondary,
    AboveWeakSuccess,
    AboveWeakWarning,
    AboveWeakDanger,
    Standard,
}
impl TextStylingColors {
    pub fn from(coloring: &StylingColors) -> TextStylingColors {
        match coloring {
            StylingColors::Background => { TextStylingColors::AboveBackground }
            StylingColors::Primary => { TextStylingColors::AbovePrimary }
            StylingColors::Secondary => { TextStylingColors::AboveSecondary }
            StylingColors::Success => { TextStylingColors::AboveSuccess }
            StylingColors::Warning => { TextStylingColors::AboveWarning }
            StylingColors::Danger => { TextStylingColors::AboveDanger }
            StylingColors::StrongBackground => { TextStylingColors::AboveStrongBackground }
            StylingColors::StrongPrimary => { TextStylingColors::AboveStrongPrimary }
            StylingColors::StrongSecondary => { TextStylingColors::AboveStrongSecondary }
            StylingColors::StrongSuccess => { TextStylingColors::AboveStrongSuccess }
            StylingColors::StrongWarning => { TextStylingColors::AboveStrongWarning }
            StylingColors::StrongDanger => { TextStylingColors::AboveStrongDanger }
            StylingColors::WeakBackground => { TextStylingColors::AboveWeakBackground }
            StylingColors::WeakPrimary => { TextStylingColors::AboveWeakPrimary }
            StylingColors::WeakSecondary => { TextStylingColors::AboveWeakSecondary }
            StylingColors::WeakSuccess => { TextStylingColors::AboveWeakSuccess }
            StylingColors::WeakWarning => { TextStylingColors::AboveWeakWarning }
            StylingColors::WeakDanger => { TextStylingColors::AboveWeakDanger }
            _ => TextStylingColors::Standard
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
        text_color: Some(StylingColors::TextFor(TextStylingColors::from(&coloring)).get_for(theme)),
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
            Status::Hovered => { theme.extended_palette().secondary.strong.color.into() }
            Status::Pressed => { coloring.get_for(theme).into() }
            Status::Disabled => { theme.extended_palette().secondary.weak.color.into() }
        }),
        border: iced::Border::default().rounded(CornerRadii::Medium.size()),
        text_color: StylingColors::TextFor(TextStylingColors::from(&coloring)).get_for(theme),
        ..Default::default()
    }
}


// standard parts
/// A standard text widget.
pub fn standard_text<'a>(
    size: TextSizes,
    background: StylingColors,
    text: String,
) -> Text<'a> {
    Text::new(text)
        .size(size.size())
        .style(move |theme| {
            text::Style { color: Some(StylingColors::TextFor(TextStylingColors::from(&background)).get_for(&theme)) }
        }).into()
}

/// A standard box with rounded corners
pub fn panel<'a, Signal: 'a>(
    coloring: StylingColors,
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
        StylingColors::WeakBackground,
        PaddingSizes::Medium,
        {
            column![
                row![
                    standard_text(TextSizes::SmallHeading, StylingColors::Background, transaction.value.to_string()),
                    space().width(PaddingSizes::Large.size()),
                    standard_text(TextSizes::Body, StylingColors::Background, transaction.date.display()),
                    space().width(PaddingSizes::Large.size()),
                    space::horizontal(),
                    edit_transaction_button(transaction),
                ],

                row![
                    standard_text(TextSizes::Body, StylingColors::Background, transaction.description.display(TagStyles::Lowercase)),
                    space::horizontal(),
                ],

                scrollable(
                row(transaction.tags.iter().map(|tag| {
                        tag_panel(app, tag, app.bank.tag_registry.get(&tag).unwrap_or(AppColors::Aqua)).into()
                    }))
                    .spacing(PaddingSizes::Small.size()),
                )
                .direction(Direction::Horizontal(Scrollbar::hidden())),
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
        .style(button_style(StylingColors::Primary))
}

pub fn tag_panel<'a, Signal: 'a>(
    app: &App,
    tag: &Tag,
    color: AppColors,
) -> Container<'a, Signal> {
    panel(
        StylingColors::Other(color.at(app.theme_selection.appearance())),
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
                PaddingSizes::Medium,
                {
                    column(cash_flow.value_flows.iter().map(|value| {
                        standard_text(
                            TextSizes::Interactable,
                            StylingColors::Primary,
                            value.to_string(),  // todo create standard function to format values (with currency)
                        ).into()
                    })).into()
                }
            )
        }

        ValueDisplayFormats::Time(price) => {
            panel(
                StylingColors::Primary,
                PaddingSizes::Medium,
                {
                    column(cash_flow.value_flows.iter().map(|value| {
                        standard_text(
                            TextSizes::Interactable,
                            StylingColors::Primary,
                            Transaction::get_time_price(&value, price).to_string(), // todo create standard function to format values (with currency)
                        ).into()
                    })).into()
                }
            )
        }
    }
}