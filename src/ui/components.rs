use iced::Length::Fill;
use iced::{Center, Length, Renderer, Theme};
use iced::{Color, Element};
use iced::widget::{Text, text, container, space, text_editor, text_input, stack};
use iced::widget::{column, row};
use iced::widget::button;
use iced::widget::text_editor::{Content, Action};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::{App, Pages};
use crate::container::signal::Signal;
use crate::pages::help_page::help_button;
use crate::ui::material::{Depths, MaterialColors, MaterialStyle, Materials};

// modes
/// The different modes that a date picker can be in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatePickerModes {
    Hidden,
    ShowingDaysInMonth,
    ShowingMonthsInYear,
}

/// The difference ways individual transactions are managed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionManagementTypes {
    Adding,
    Editing,
}



// standard parameters
/// Allows Widgets to be sized with a single size object instead of with separate width and height parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanelSize {
    pub width: Widths,
    pub height: Heights,
}

/// Allows custom widgets to use standardized widths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Widths {
    Shrink,
    Fill,
    NanoCard,
    MicroCard,
    SmallCard,
    MediumCard,
    LargeCard,
    GinormousCard,
    MicroField,
    SmallField,
    MediumField,
    LargeField,
    Other(f32),
}
impl Widths {
    /// Gets the size as an f32.
    #[must_use]
    pub fn size(&self) -> f32 {
        match self {
            Widths::Shrink | Widths::Fill => { 1.0 }
            Widths::NanoCard => { 100.0 }
            Widths::MicroCard => { 175.0 }
            Widths::SmallCard => { 350.0 }
            Widths::MediumCard => { 550.0 }
            Widths::LargeCard => { 750.0 }
            Widths::GinormousCard => { 1000.0 }
            Widths::MicroField => { Widths::MicroCard.size() - (PaddingSizes::Medium.size() * 2.0) }
            Widths::SmallField => { Widths::SmallCard.size() - (PaddingSizes::Medium.size() * 2.0) }
            Widths::MediumField => { Widths::MediumCard.size() - (PaddingSizes::Medium.size() * 2.0) }
            Widths::LargeField => { Widths::LargeCard.size() - (PaddingSizes::Medium.size() * 2.0) }
            Widths::Other(size) => { *size }
        }
    }
}

/// Allows custom widgets to use standardized widths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heights {
    Shrink,
    Fill,
    Header,
    ManagementPanel,
    NanoCard,
    MicroCard,
    SmallCard,
    MediumCard,
    LargeCard,
    GinormousCard,
    Other(f32),
}
impl Heights {
    /// Gets the size as an f32.
    #[must_use]
    pub fn size(&self) -> f32 {
        match self {
            Heights::Shrink | Heights::Fill => { 1.0 }
            Heights::Header => { 80.0 }
            Heights::ManagementPanel => { 150.0 }
            Heights::NanoCard => { 60.0 }
            Heights::MicroCard => { 100.0 }
            Heights::SmallCard => { 200.0 }
            Heights::MediumCard => { 350.0 }
            Heights::LargeCard => { 500.0 }
            Heights::GinormousCard => { 700.0 }
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
    /// Gets the size as an f32.
    #[must_use]
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
    ManagementPanelSpace,
}
impl Spacing {
    /// Gets the size as an f32.
    #[must_use]
    pub fn size(&self) -> f32 {
        match self {
            Spacing::Fill | Spacing::None => { PaddingSizes::None.size() }
            Spacing::Nano => { PaddingSizes::Nano.size() }
            Spacing::Micro => { PaddingSizes::Micro.size() }
            Spacing::Small => { PaddingSizes::Small.size() }
            Spacing::Medium => { PaddingSizes::Medium.size() }
            Spacing::Large => { PaddingSizes::Large.size() }
            Spacing::Ginormous => { PaddingSizes::Ginormous.size() }
            Spacing::Other(size) => { PaddingSizes::Other(*size).size() }
            Spacing::HeaderSpace => { Heights::Header.size() + (PaddingSizes::Small.size() * 2.0) }
            Spacing::ManagementPanelSpace => { Heights::ManagementPanel.size() + (PaddingSizes::Small.size() * 2.0) }
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
    /// Gets the size as an f32.
    #[must_use]
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
    /// Gets the size as an f32.
    #[must_use]
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
    /// Gets the size as an f32.
    #[must_use]
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

/// Allows custom buttons to follow a certain shape style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonShapes {
    LowProfile,
    Minimal,
    Bloated,
    Standard,
    Wide,
}

/// Allows choosing between two general forward and backward directions (advance and recede).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Directions {
    Advance,
    Recede,
}



// standard styles
/// Returns a standard rounded background style.
fn panel_container_style(
    app: &App,
    material_style: MaterialStyle,
) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(
            material_style.color.materialized(
                material_style.material,
                material_style.depth,
                false,
                app.theme_selection,
            ).into()
        ),
        
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(0)
            .color(Color::TRANSPARENT),
        
        shadow: iced::Shadow {
            color: if material_style.casts_shadow() {
                material_style.color.materialized(
                    material_style.material,
                    material_style.depth,
                    true,
                    app.theme_selection,
                ).into()
            }
            else { Color::TRANSPARENT },
            offset: if material_style.depth == Depths::Proud { iced::Vector::new(0.0, 4.0) } else { iced::Vector::new(0.0, -4.0) },
            blur_radius: 0.0,
        },
        
        text_color: Some(
            MaterialColors::text().materialized(
                material_style.material,
                material_style.depth,
                false,
                app.theme_selection,
            )
        ),
        
        snap: false,
    }
}

/// Returns standard button style.
fn panel_button_style(
    app: &App,
    material_style: MaterialStyle,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| button::Style {
        background: Some(match status {
            button::Status::Active | button::Status::Hovered => {
                material_style.color.materialized(
                    material_style.material,
                    material_style.depth,
                    false,
                    app.theme_selection,
                ).into()
            }
            
            button::Status::Pressed | button::Status::Disabled => {
                material_style.color.materialized(
                    material_style.material,
                    material_style.depth,
                    true,
                    app.theme_selection,
                ).into()
            }
        }),
        
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Thin.size())
            .color(match status {
                button::Status::Active => {
                    material_style.color.materialized(
                        material_style.material,
                        material_style.depth,
                        false,
                        app.theme_selection,
                    )
                }
                
                button::Status::Hovered | button::Status::Pressed | button::Status::Disabled=> {
                    material_style.color.materialized(
                        material_style.material,
                        material_style.depth,
                        true,
                        app.theme_selection,
                    )
                }
            }),
        
        shadow: iced::Shadow {
            color: if material_style.casts_shadow() {
                material_style.color.materialized(
                    material_style.material,
                    material_style.depth,
                    true,
                    app.theme_selection,
                ).into()
            }
            else { Color::TRANSPARENT },
            offset: if material_style.depth == Depths::Proud { iced::Vector::new(0.0, 4.0) } else { iced::Vector::new(0.0, -4.0) },
            blur_radius: 0.0,
        },
        
        text_color: {
            MaterialColors::text().materialized(
                material_style.material,
                material_style.depth,
                false,
                app.theme_selection,
            )
        },
        
        snap: false,
    }
}

/// Returns a standard text input style.
fn text_input_style(
    app: &App,
    material_style: MaterialStyle,
) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |_, status| text_input::Style {
        background: match status {
            text_input::Status::Active | text_input::Status::Hovered | text_input::Status::Focused { is_hovered: true | false } => {
                material_style.color.materialized(
                    material_style.material,
                    material_style.depth,
                    false,
                    app.theme_selection,
                ).into()
            }
            
            text_input::Status::Disabled => {
                MaterialColors::Unavailable.materialized(
                    material_style.material,
                    material_style.depth,
                    true,
                    app.theme_selection,
                ).into()
            }
        },
        
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Thin.size())
            .color(match status {
                text_input::Status::Active => {
                    material_style.color.materialized(
                        material_style.material,
                        material_style.depth,
                        false,
                        app.theme_selection,
                    )
                }
                
                text_input::Status::Hovered | text_input::Status::Focused { is_hovered: true | false } | text_input::Status::Disabled => {
                    material_style.color.materialized(
                        material_style.material,
                        material_style.depth,
                        true,
                        app.theme_selection,
                    )
                }
            }),
        
        icon: MaterialColors::CardContent.materialized(
            material_style.material,
            material_style.depth,
            false,
            app.theme_selection,
        ),
        
        placeholder: MaterialColors::text().materialized(Materials::Plastic, Depths::Flat, true, app.theme_selection),
        
        value: MaterialColors::text().materialized(Materials::Plastic, Depths::Flat, false, app.theme_selection),
        
        selection: MaterialColors::primary().materialized(Materials::Plastic, Depths::Flat, false, app.theme_selection),
    }
}

/// Returns a standard text editor style.
fn text_editor_style(
    app: &App,
    material_style: MaterialStyle,
) -> impl Fn(&Theme, text_editor::Status) -> text_editor::Style {
    move |_, status| text_editor::Style {
        background: match status {
            text_editor::Status::Active | text_editor::Status::Hovered | text_editor::Status::Focused { is_hovered: true | false } => {
                material_style.color.materialized(
                    material_style.material,
                    material_style.depth,
                    false,
                    app.theme_selection,
                ).into()
            }
            
            text_editor::Status::Disabled => {
                MaterialColors::Unavailable.materialized(
                    material_style.material,
                    material_style.depth,
                    true,
                    app.theme_selection,
                ).into()
            }
        },
        
        border: iced::Border::default()
            .rounded(CornerRadii::Medium.size())
            .width(BorderThickness::Thin.size())
            .color(match status {
                text_editor::Status::Active => {
                    material_style.color.materialized(
                        material_style.material,
                        material_style.depth,
                        false,
                        app.theme_selection,
                    )
                }
                
                text_editor::Status::Hovered | text_editor::Status::Focused { is_hovered: true | false } | text_editor::Status::Disabled => {
                    material_style.color.materialized(
                        material_style.material,
                        material_style.depth,
                        true,
                        app.theme_selection,
                    )
                }
            }),
        
        placeholder: MaterialColors::text().materialized(Materials::Plastic, Depths::Flat, true, app.theme_selection),
        
        value: MaterialColors::text().materialized(Materials::Plastic, Depths::Flat, false, app.theme_selection),
        
        selection: MaterialColors::primary().materialized(Materials::Plastic, Depths::Flat, false, app.theme_selection),
    }
}



// standard ui components
/// A standard spacing widget.
/// Please note that these will compete for space in a layout when set to Fill when there are more than one.
/// In a layout such as row![spacer(Fill), content, spacer(Fill)], each spacer will take 1/3 of the available space,
/// even if the content is set to be larger than its 1/3 slice, shrinking the container.
#[must_use]
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

/// Adds padding around a widget.
#[must_use]
pub fn pad<'a>(
    padding: PaddingSizes,
    content: Element<'a, Signal>,
) -> Element<'a, Signal> {
    container(content)
        .padding(padding.size())
        .into()
}

/// A standard text widget.
#[must_use]
pub fn ui_string<'a>(
    app: &'a App,
    text: impl Into<String>,
    size: TextSizes,
    color: MaterialColors,
) -> Element<'a, Signal> {
    Text::new(text.into())
        .size(size.size())
        .style(move |_theme| {
            text::Style { color: Some(color.materialized(Materials::Plastic, Depths::Flat, false, app.theme_selection)) }
        }).into()
}

/// A standard box with rounded corners.
#[must_use]
pub fn panel<'a>(
    app: &'a App,
    material_style: MaterialStyle,
    panel_size: PanelSize,
    internal_padding: PaddingSizes,
    content: Element<'a, Signal>,
) -> Element<'a, Signal> {
    container(
        container(content)
            .padding(internal_padding.size())
            .style(panel_container_style(app, material_style))
            .width(match panel_size.width {
                Widths::Shrink => { Length::Shrink }
                Widths::Fill => { Length::Fill }
                _ => { Length::Fixed(panel_size.width.size()) }
            })
            .height(match panel_size.height {
                Heights::Shrink => { Length::Shrink }
                Heights::Fill => { Length::Fill }
                _ => { Length::Fixed(panel_size.height.size()) }
            })
    )
    .padding(PaddingSizes::Micro.size())
    .into()
}

/// A standard button with rounded corners.
#[must_use]
pub fn panel_button<'a>(
    app: &'a App,
    material_style: MaterialStyle,
    shape: ButtonShapes,
    label: impl Into<Element<'a, Signal, Theme, Renderer>>,
    signal: Signal,
    active: bool,
) -> Element<'a, Signal> {
    let button = button(label)
        .style(panel_button_style(app, material_style))
        .padding(match shape {
            ButtonShapes::LowProfile => { [PaddingSizes::Micro.size(), PaddingSizes::Small.size()] }
            ButtonShapes::Minimal => { [PaddingSizes::Small.size(), PaddingSizes::Small.size()] }
            ButtonShapes::Bloated => { [PaddingSizes::Small.size(), PaddingSizes::Medium.size()] }
            ButtonShapes::Standard => { [PaddingSizes::Small.size(), PaddingSizes::Large.size()] }
            ButtonShapes::Wide => { [PaddingSizes::Small.size(), PaddingSizes::Ginormous.size()] }
        });
    
    container(
        if active { button.on_press(signal) }
        else { button }
    )
    .padding(PaddingSizes::Micro.size())
    .into()
}

/// A standard text input panel with rounded corners.
#[must_use]
#[allow(clippy::too_many_arguments)] // this has a lot of arguments, but they're all necessary
pub fn panel_text_input<'a>(
    app: &'a App,
    material_style: MaterialStyle,
    width: Widths,
    placeholder: &str,
    value: &str,
    on_change: fn(String) -> Signal,
    on_submit_option: Option<Signal>,
    can_submit: bool,
) -> Element<'a, Signal> {
    container(
        panel(
            app,
            material_style,
            PanelSize { width, height: Heights::Shrink },
            PaddingSizes::None, {
                if let Some(on_submit) = on_submit_option && can_submit {
                    text_input(placeholder, value)
                        .style(text_input_style(app, material_style))
                        .on_input(on_change)
                        .on_submit(on_submit)
                        .into()
                }
                else {
                    text_input(placeholder, value)
                        .style(text_input_style(app, material_style))
                        .on_input(on_change)
                        .into()
                }
            }
        )
    )
    .padding(PaddingSizes::Micro.size())
    .into()
}

/// A standard text editor panel with rounded corners.
#[must_use]
pub fn panel_text_editor<'a>(
    app: &'a App,
    material_style: MaterialStyle,
    panel_size: PanelSize,
    value: &'a Content,
    on_change: fn(Action) -> Signal,
) -> Element<'a, Signal> {
    container(
        panel(
            app,
            material_style,
            panel_size,
            PaddingSizes::None, {
                text_editor(value)
                    .style(text_editor_style(app, material_style))
                    .on_action(on_change)
                    .into()
            }
        )
    )
    .padding(PaddingSizes::Micro.size())
    .into()
}



// standard app widgets
/// A header for every page.
#[must_use]
pub fn header<'a>(
    app: &'a App,
    additional_content: Vec<Element<'a, Signal>>,
) -> Element<'a, Signal> {
    let mut positioned_addition_content = additional_content;
    positioned_addition_content.insert(0, spacer(Orientations::Horizontal, Spacing::Fill));

    // holds the header and the spacer under it to guarantee it is "pushed" to the top of the page
    column![
        // this is the main header background bar
        panel(
            app,
            MaterialStyle { material: Materials::Acrylic, color: MaterialColors::Card, depth: Depths::Proud, },
            PanelSize { width: Widths::Fill, height: Heights::Header, },
            PaddingSizes::Small, {
                // this holds the title and the additional content all within the main header background bar
                stack![
                    // help button
                    container(
                        row![
                            help_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ].spacing(0),
                    )
                    .height(Length::Fill)
                    .align_y(Center),

                    // additional content
                    container(
                        row(positioned_addition_content)
                        .align_y(Center)
                        .spacing(Spacing::Large.size())
                    )
                    .height(Length::Fill)
                    .align_y(Center),

                    // main content
                    container(
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),

                            panel(
                                app,
                                MaterialStyle { material: Materials::Acrylic, color: MaterialColors::CardContent, depth: Depths::Proud, },
                                PanelSize { width: Widths::Shrink, height: Heights::Shrink, },
                                PaddingSizes::Small, {
                                    row![
                                        spacer(Orientations::Horizontal, Spacing::Medium),
                                        icon(app.page.icon_name()),
                                        spacer(Orientations::Horizontal, Spacing::Small),
                                        ui_string(app, app.page.name(), TextSizes::LargeHeading, MaterialColors::StrongText),
                                        spacer(Orientations::Horizontal, Spacing::Medium),
                                    ]
                                    .align_y(Center)
                                    .spacing(Spacing::None.size())
                                    .into()
                                }
                            ),

                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                        .align_y(Center)
                        .spacing(Spacing::Large.size())
                    )
                    .height(Length::Fill)
                    .align_y(Center),
                ]
                .into()
            }
        ),

        space::vertical(),
    ]
    .spacing(Spacing::Micro.size())
    .padding(PaddingSizes::Micro.size())
    .into()
}

/// The panel used to navigate between pages in the app.
#[must_use]
pub fn navigation_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    pad(PaddingSizes::Small,
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),
            
            panel(
                app,
                MaterialStyle { material: Materials::Plastic, color: MaterialColors::Card, depth: Depths::Proud, },
                PanelSize { width: Widths::Shrink, height: Heights::Fill, },
                PaddingSizes::Small, {
                    column![
                        page_pointer(app, Pages::Transactions),
                        page_pointer(app, Pages::TagRegistry),
                        page_pointer(app, Pages::Settings),
                    ]
                    .spacing(Spacing::Small.size())
                    .into()
                }
            )
        ]
        .spacing(0)
        .into()
    )
}

/// A button that navigates to a specific page.
#[must_use]
pub fn page_pointer<'a>(
    app: &'a App,
    page: Pages,
) -> Element<'a, Signal> {
    let color = if app.page == page { app.theme_selection.accent() }
    else { MaterialColors::CardContent };
    
    panel_button(
        app,
        MaterialStyle { material: Materials::Plastic, color, depth: Depths::Proud, },
        ButtonShapes::Wide,
        row![
            icon(page.icon_name()),
            ui_string(app, page.name(), TextSizes::Interactable, MaterialColors::StrongText),
        ]
        .spacing(Spacing::Large.size()),
        Signal::ChangePageTo(page),
        true,
    )
}