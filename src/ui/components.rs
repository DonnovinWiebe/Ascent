use iced::{Fill, Length};
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

// options
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



// standards
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
            PaddingSizes::Micro => { 3.0 }
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
    Standard,
    Thick,
}
impl BorderThickness {
    /// Gets the size of the selection.
    pub fn size(&self) -> f32 {
        match self {
            BorderThickness::Standard => { 1.25 }
            BorderThickness::Thick => { 2.5 }
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
    color: AppColors,
    cast_shadow: bool,
) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(color.themed(&app.theme_selection, AppColorStrengths::Base).into()),
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Standard.size())
            .color(color.themed(&app.theme_selection, AppColorStrengths::Secondary)),
        shadow: iced::Shadow {
            color: if cast_shadow { Color::BLACK } else { Color::TRANSPARENT },
            offset: if cast_shadow { iced::Vector::new(1.0, 1.0) } else { iced::Vector::new(0.0, 0.0) },
            blur_radius: if cast_shadow { 3.0 } else { 0.0 },
        },
        text_color: Some(AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Base)),
        snap: false,
    }
}

/// Returns a standard text input style.
pub fn text_input_style(
    app: &App,
    color: AppColors,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_theme, status| text_input::Style {
        background: match status {
            text_input::Status::Active => { color.themed(&app.theme_selection, AppColorStrengths::Base).into() }
            text_input::Status::Hovered => { color.themed(&app.theme_selection, AppColorStrengths::Secondary).into() }
            text_input::Status::Focused { is_hovered: false } => { AppColors::Accent.themed(&app.theme_selection, AppColorStrengths::Base).into() }
            text_input::Status::Focused { is_hovered: true } => { AppColors::Accent.themed(&app.theme_selection, AppColorStrengths::Base).into() }
            text_input::Status::Disabled => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Base).into() }
        },
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Standard.size())
            .color(match status {
                text_input::Status::Active => { color.themed(&app.theme_selection, AppColorStrengths::Secondary) }
                text_input::Status::Hovered => { color.themed(&app.theme_selection, AppColorStrengths::Secondary) }
                text_input::Status::Disabled => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Secondary) }
                _ => { color.themed(&app.theme_selection, AppColorStrengths::Secondary) }
            }),
        icon: AppColors::Accent.themed(&app.theme_selection, AppColorStrengths::Base),
        placeholder: AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Secondary),
        value: AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Base),
        selection: AppColors::Accent.themed(&app.theme_selection, AppColorStrengths::Base),
    }
}

/// Returns standard button style.
pub fn button_style(
    app: &App,
    color: AppColors,
    cast_shadow: bool,
) -> impl Fn(&Theme, Status) -> button::Style {
    move |_theme, status| button::Style {
        background: Some(match status {
            Status::Active => { color.themed(&app.theme_selection, AppColorStrengths::Base).into() }
            Status::Hovered => { color.themed(&app.theme_selection, AppColorStrengths::Secondary).into() }
            Status::Pressed => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Base).into() }
            Status::Disabled => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Base).into() }
        }),
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Standard.size())
            .color(match status {
                Status::Active => { color.themed(&app.theme_selection, AppColorStrengths::Secondary) }
                Status::Hovered => { color.themed(&app.theme_selection, AppColorStrengths::Secondary) }
                Status::Pressed => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Secondary) }
                Status::Disabled => { AppColors::Unavailable.themed(&app.theme_selection, AppColorStrengths::Secondary) }
            }),
        shadow: iced::Shadow {
            color: if cast_shadow { Color::BLACK } else { Color::TRANSPARENT },
            offset: if cast_shadow { iced::Vector::new(1.0, 1.0) } else { iced::Vector::new(0.0, 0.0) },
            blur_radius: if cast_shadow { 3.0 } else { 0.0 },
        },
        text_color: AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Base),
        snap: false,
    }
}


// standard parts
/// A standard text widget.
pub fn standard_text<'a, Sig: 'a + Clone>(
    app: &'a App,
    size: TextSizes,
    text: String,
) -> Element<'a, Signal> {
    Text::new(text)
        .size(size.size())
        .style(move |_theme| {
            text::Style { color: Some(AppColors::Text.themed(&app.theme_selection, AppColorStrengths::Base)) }
        }).into()
}

/// A standard box with rounded corners
pub fn panel<'a, Sig: 'a + Clone>(
    app: &'a App,
    color: AppColors,
    cast_shadow: bool,
    internal_padding: PaddingSizes,
    width: Option<Widths>,
    height: Option<Heights>,
    content: Element<'a, Signal>,
) -> Element<'a, Signal> {
    Container::new(content)
        .padding(internal_padding.size())
        .style(rounded_container_style(app, color, cast_shadow))
        .width(if let Some(width) = width { Length::Fixed(width.size()) } else { Length::Shrink })
        .height(if let Some(height) = height { Length::Fixed(height.size()) } else { Length::Shrink })
        .into()
}

/// A standard button with rounded corners
pub fn panel_button<'a, Sig: 'a + Clone>(
    app: &'a App,
    label: String,
    color: AppColors,
    cast_shadow: bool,
    signal: Signal,
) -> Element<'a, Signal> {
    button(text(label))
        .style(button_style(app, color, cast_shadow))
        .on_press(signal)
        .into()
}



// bank overview parts
pub fn transaction_list<'a, Sig: 'a + Clone>(
    app: &'a App,
    transactions: Vec<&Transaction>,
    value_display_format: ValueDisplayFormats,
)  -> Element<'a, Signal> {
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();
    for i in 0..transactions.len() {
        let transaction = &transactions[i];
        if i % 2 == 0 { first_half.push(transaction); }
        else { second_half.push(transaction); }
    }

    container(
        scrollable(
            row![
                column(first_half.into_iter().map(|transaction| {
                    transaction_panel::<Sig>(app, transaction)
                }))
                .spacing(PaddingSizes::Small.size()),

                space().width(PaddingSizes::Medium.size()),

                column(second_half.into_iter().map(|transaction| {
                    transaction_panel::<Sig>(app, transaction)
                }))
                .spacing(PaddingSizes::Small.size()),
            ]
        )
            .direction(Direction::Vertical(Scrollbar::hidden()))
            .width(Widths::SmallCard.size() * 2.0 + PaddingSizes::Medium.size() * 3.0)
            .height(Fill),
    )
        .center_x(Fill)
        .into()
}

pub fn transaction_panel<'a, Sig: 'a + Clone>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    panel::<Sig>(app, AppColors::Midground, true, PaddingSizes::Other(0.0), Some(Widths::SmallCard), None, {
        column![
            space().height(PaddingSizes::Medium.size()),

            row![
                space().width(PaddingSizes::Medium.size()),
                standard_text::<Sig>(app, TextSizes::SmallHeading, transaction.value.to_string()),
                space().width(PaddingSizes::Large.size()),
                standard_text::<Sig>(app, TextSizes::Body, transaction.date.display()),
                space().width(PaddingSizes::Large.size()),
                space::horizontal(),
                edit_transaction_button(app, transaction),
                space().width(PaddingSizes::Medium.size()),
            ],

            row![
                space().width(PaddingSizes::Medium.size()),
                standard_text::<Sig>(app, TextSizes::Body, transaction.description.clone()),
                space::horizontal(),
                space().width(PaddingSizes::Medium.size()),
            ],

            scrollable(
                row({
                    let mut list: Vec<Element<Signal>> = transaction.tags.iter().map(|tag| {
                        tag_panel::<Sig>(app, tag, app.bank.tag_registry.get(&tag).unwrap_or(AppColors::Aqua))
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
            .into()
    })
}

pub fn edit_transaction_button<'a>(
    app: &'a App,
    transaction: &Transaction,
) -> Element<'a, Signal> {
    button("Edit")
        .on_press(StartEditingTransaction(transaction.get_id().expect("Tried to edit a transaction without an id!")))
        .style(button_style(app, AppColors::Accent, false))
        .into()
}

pub fn tag_panel<'a, Sig: 'a + Clone>(
    app: &'a App,
    tag: &Tag,
    color: AppColors,
) -> Element<'a, Signal> {
    panel::<Sig>(app, color, false, PaddingSizes::Small, None, None, {
        standard_text::<Sig>(app, TextSizes::Interactable, tag.display(TagStyles::Lowercase))
    })
}

/// Returns a cash flow panel.
pub fn cash_flow_panel<'a, Sig: 'a + Clone>(
    app: &'a App,
    cash_flow: &CashFlow,
    value_display_format: ValueDisplayFormats
) -> Element<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            panel::<Sig>(app, AppColors::Accent, true, PaddingSizes::Medium, None, None,{
                column(cash_flow.value_flows.iter().map(|value| {
                    standard_text::<Sig>(app, TextSizes::Interactable, value.to_string())
                })).into()
            }).into()
        }

        ValueDisplayFormats::Time(price) => {
            panel::<Sig>(app, AppColors::Accent, true, PaddingSizes::Medium, None, None, {
                column(cash_flow.value_flows.iter().map(|value| {
                    standard_text::<Sig>(app, TextSizes::Interactable, Transaction::get_time_price(&value, price).to_string())
                })).into()
            }).into()
        }
    }
}

pub fn date_picker<'a, Sig: Clone + 'a>(
    app: &'a App,
    transaction_management: TransactionManagementTypes,
) -> Element<'a, Signal> {
    match transaction_management {
        TransactionManagementTypes::Adding => {
            match app.new_date_picker_mode {
                DatePickerModes::Hidden => {
                    panel::<Sig>(app, AppColors::Foreground, true, PaddingSizes::Medium, None, None, {
                        row![
                            standard_text::<Sig>(app, TextSizes::Interactable, app.new_transaction_date.display()),
                            space().width(PaddingSizes::Medium.size()),
                            panel_button::<Sig>(app, "Edit".to_string(), AppColors::Foreground, true, UpdateNewDatePickerMode(DatePickerModes::ShowingDaysInMonth)),
                        ].into()
                    })
                }

                DatePickerModes::ShowingMonthsInYear => {todo!()}

                DatePickerModes::ShowingDaysInMonth => {todo!()}
            }
        }



        TransactionManagementTypes::Editing => {
            match app.edit_date_picker_mode {
                DatePickerModes::Hidden => {
                    panel::<Sig>(app, AppColors::Foreground, true, PaddingSizes::Medium, None, None, {
                        row![
                            standard_text::<Sig>(app, TextSizes::Interactable, app.edit_transaction_date.display()),
                            space().width(PaddingSizes::Medium.size()),
                            panel_button::<Sig>(app, "Edit".to_string(), AppColors::Foreground, true, UpdateEditDatePickerMode(DatePickerModes::ShowingDaysInMonth)),
                        ].into()
                    })
                }

                DatePickerModes::ShowingMonthsInYear => {todo!()}

                DatePickerModes::ShowingDaysInMonth => {todo!()}
            }
        }
    }
}