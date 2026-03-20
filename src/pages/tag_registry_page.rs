use iced::{Center, Fill};
use iced::{Length};
use iced::{Color, Element, Size};
use iced::advanced::Widget;
use iced::widget::{Column, Stack, button, container, scrollable, space, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::ui::components::*;
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::parse::*;
use crate::ui::charting::*;

// tag registry page
/// The page used for managing the coloring of tags.
pub fn tag_registry_page(
    app: &App,
) -> Stack<Signal> {
    stack![
        tag_registry_panel(app),
        
        header(
            app,
            true,
            Vec::new(),
            Vec::new(),
        )
    ]
        .width(Fill)
        .height(Fill)
}

/// A panel used to edit the tag registry
pub fn tag_registry_panel(
    app: &App,
) -> Element<Signal> {
    container(
        panel(
            app,
            Materials::Plastic,
            MaterialColors::Background,
            2,
            true,
            Widths::LargeCard,
            Heights::LargeCard,
            PaddingSizes::None, {
                let tags: Vec<Tag> = app.bank.get_tags();
                
                column![
                    // title
                    row![
                        ui_string(app, 1, "Tag Registry".to_string(), TextSizes::LargeHeading),
                        spacer(Orientations::Horizontal, Spacing::Fill),
                    ]
                    .align_y(Center),
                    
                    scrollable(
                        column(
                            tags.into_iter().map(|tag| {
                                tag_registration_slip(app, tag)
                            })
                        )
                    )
                    .direction(Direction::Vertical(Scrollbar::hidden())),
                    
                ]
                .spacing(Spacing::None.size())
                .into()
            }
        )
    )
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

pub fn tag_registration_slip(
    app: &App,
    tag: Tag,
) -> Element<Signal> {
    
    row![
        ui_string(app, 1, tag.get_label().to_string(), TextSizes::Interactable),
        scrollable(
            row(
                MaterialColors::standard_colors().into_iter().map(|color| {
                    let button_color = if app.bank.tag_registry.get(&tag) == color { color } else { MaterialColors::Unavailable };
                    panel_button(
                        app,
                        Materials::RimmedPlastic,
                        button_color,
                        1,
                        true,
                        ButtonShapes::LowProfile,
                        ui_string(app, 1, color.name(), TextSizes::Interactable),
                        Signal::SetTagColor(tag.clone(), color),
                        true,
                    )
                }).collect::<Vec<_>>()
            )
        )
        .direction(Direction::Horizontal(Scrollbar::hidden())),
    ]
    .spacing(Spacing::None.size())
    .into()
}