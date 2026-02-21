use iced::{Color, Element, Size};
use iced::border::color;
use iced::futures::{FutureExt, TryFutureExt};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button::Status;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::palette::{AppColorStrengths, AppColors, AppThemes};
use crate::vault::parse::*;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::container::signal::Signal::*;

// standards
/// Allows custom widgets to use standardized padding.
pub enum PaddingSizes {
    Small,
    Medium,
    Large,
    Other(f32)
}
impl PaddingSizes {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            PaddingSizes::Small => { 6.0 }
            PaddingSizes::Medium => { 10.0 }
            PaddingSizes::Large => { 14.0 }
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
            CornerRadii::Small => { 12.0 }
            CornerRadii::Medium => { 16.0 }
            CornerRadii::Large => { 20.0 }
        }
    }
}

/// Allows custom widgets to use standardized corner radius sizes.
pub enum BorderThickness {
    Thin,
    Standard,
    Thick,
}
impl BorderThickness {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            BorderThickness::Thin => { 1.0 }
            BorderThickness::Standard => { 2.0 }
            BorderThickness::Thick => { 3.0 }
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
/// Returns a rounded background style.
fn rounded_container_style(
    app: &App,
    color: AppColors,
    cast_shadow: bool,
) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(color.themed(&app.theme_selection, AppColorStrengths::Dark).into()),
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Standard.size())
            .color(color.themed(&app.theme_selection, AppColorStrengths::Light)),
        shadow: iced::Shadow {
            color: if cast_shadow { Color::BLACK } else { Color::TRANSPARENT },
            offset: iced::Vector::new(1.5, 1.5),
            blur_radius: if cast_shadow { 2.5 } else { 0.0 },
        },
        text_color: Some(AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Dark).into()),
        ..Default::default()
    }
}

/// Returns standard button style.
fn button_style(
    app: &App,
    color: AppColors,
    cast_shadow: bool,
) -> impl Fn(&Theme, Status) -> button::Style {
    move |_theme, status| button::Style {
        background: Some(match status {
            Status::Active => { color.themed(&app.theme_selection, AppColorStrengths::Dark).into() }
            Status::Hovered => { color.themed(&app.theme_selection, AppColorStrengths::Light).into() }
            Status::Pressed => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Dark).into() }
            Status::Disabled => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Dark).into() }
        }),
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Standard.size())
            .color(match status {
                Status::Active => { color.themed(&app.theme_selection, AppColorStrengths::Light) }
                Status::Hovered => { color.themed(&app.theme_selection, AppColorStrengths::Light) }
                Status::Pressed => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Light) }
                Status::Disabled => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Light) }
            }),
        shadow: iced::Shadow {
            color: if cast_shadow { Color::BLACK } else { Color::TRANSPARENT },
            offset: iced::Vector::new(1.5, 1.5),
            blur_radius: if cast_shadow { 2.5 } else { 0.0 },
        },
        text_color: AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Dark).into(),
        ..Default::default()
    }
}


// standard parts
/// A standard text widget.
pub fn standard_text(
    app: &App,
    size: TextSizes,
    text: String,
) -> Text {
    Text::new(text)
        .size(size.size())
        .style(move |_theme| {
            text::Style { color: Some(AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Dark).into()) }
        }).into()
}

/// A standard box with rounded corners
pub fn panel<'a, Signal: 'a>(
    app: &'a App,
    color: AppColors,
    cast_shadow: bool,
    internal_padding: PaddingSizes,
    content: Element<'a, Signal>,
) -> Container<'a, Signal> {
    Container::new(content)
        .padding(internal_padding.size())
        .style(rounded_container_style(app, color, cast_shadow))
}



// bank overview parts
pub fn transaction_list<'a>(
    app: &'a App,
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
    app: &'a App,
    transaction: &Transaction,
) -> Container<'a, Signal> {
    panel(
        app,
        AppColors::Midground,
        true,
        PaddingSizes::Other(0.0), {
            column![
                space().height(PaddingSizes::Medium.size()),

                row![
                    space().width(PaddingSizes::Medium.size()),
                    standard_text(app, TextSizes::SmallHeading, transaction.value.to_string()),
                    space().width(PaddingSizes::Large.size()),
                    standard_text(app, TextSizes::Body, transaction.date.display()),
                    space().width(PaddingSizes::Large.size()),
                    space::horizontal(),
                    edit_transaction_button(app, transaction),
                    space().width(PaddingSizes::Medium.size()),
                ],

                row![
                    space().width(PaddingSizes::Medium.size()),
                    standard_text(app, TextSizes::Body, transaction.description.clone()),
                    space::horizontal(),
                    space().width(PaddingSizes::Medium.size()),
                ],

                scrollable(
                    row({
                        let mut list: Vec<Element<Signal>> = transaction.tags.iter().map(|tag| {
                            tag_panel(app, tag, app.bank.tag_registry.get(&tag).unwrap_or(AppColors::Aqua)).into()
                        }).collect::<Vec<Element<Signal>>>();
                        list.insert(0, space().width(PaddingSizes::Small.size()).into());
                        list.push(space().width(PaddingSizes::Small.size()).into());
                        list
                        })
                    .spacing(PaddingSizes::Small.size()),
                )
                .direction(Direction::Horizontal(Scrollbar::hidden())),

                space().height(PaddingSizes::Medium.size()),
            ]
                .spacing(PaddingSizes::Small.size())
        }.into()
    )
}

pub fn edit_transaction_button<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Button<'a, Signal> {
    button("Edit")
        .on_press(StartEditingTransaction(transaction.get_id().expect("Tried to edit a transaction without an id!")))
        .style(button_style(app, AppColors::Accent, false))
}

pub fn tag_panel<'a, Signal: 'a>(
    app: &'a App,
    tag: &Tag,
    color: AppColors,
) -> Container<'a, Signal> {
    panel(
        app,
        color,
        false,
        PaddingSizes::Small, {
            standard_text(app, TextSizes::Interactable, tag.display(TagStyles::Lowercase))
        }.into()
    )
}

/// Returns a cash flow panel.
pub fn cash_flow_panel<'a, Signal: 'a>(
    app: &'a App,
    cash_flow: &CashFlow,
    value_display_format: ValueDisplayFormats
) -> Container<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            panel(
                app,
                AppColors::Accent,
                true,
                PaddingSizes::Medium, {
                    column(cash_flow.value_flows.iter().map(|value| {
                        standard_text(
                            app,
                            TextSizes::Interactable,
                            value.to_string(),  // todo create standard function to format values (with currency)
                        ).into()
                    })).into()
                }
            )
        }

        ValueDisplayFormats::Time(price) => {
            panel(
                app,
                AppColors::Accent,
                true,
                PaddingSizes::Medium, {
                    column(cash_flow.value_flows.iter().map(|value| {
                        standard_text(
                            app,
                            TextSizes::Interactable,
                            Transaction::get_time_price(&value, price).to_string(), // todo create standard function to format values (with currency)
                        ).into()
                    })).into()
                }
            )
        }
    }
}