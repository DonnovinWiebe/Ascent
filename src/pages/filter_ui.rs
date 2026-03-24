use iced::{Center, Fill};
use iced::{Length};
use iced::{Color, Element, Size};
use iced::advanced::Widget;
use iced::widget::{Column, Stack, container, image, mouse_area, responsive, scrollable, space, stack};
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

pub fn filter_tags_area<'a>(
    app: &'a App,
    filter: Filters,
) -> Element<'a, Signal> {
    panel(
        app,
        Materials::Plastic,
        MaterialColors::Background,
        1,
        false,
        Widths::Fill,
        Heights::MicroCard,
        PaddingSizes::None, {
            let tags = app.bank.get_tags();
            let mut first_half = Vec::new();
            let mut second_half = Vec::new();
            for i in 0..tags.len() {
                let tag = tags[i].clone();
                if i % 2 == 0 { first_half.push(filter_tag_panel(app, tag, filter)); }
                else { second_half.push(filter_tag_panel(app, tag, filter)); }
            }
            first_half.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            first_half.push(spacer(Orientations::Horizontal, Spacing::Small));
            second_half.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
            second_half.push(spacer(Orientations::Horizontal, Spacing::Small));
            
            scrollable(
                row![
                    column![
                        spacer(Orientations::Vertical, Spacing::Fill),
                        
                        row(first_half)
                        .spacing(Spacing::Small.size()),
                        
                        spacer(Orientations::Vertical, Spacing::Nano),
                        
                        row(second_half)
                        .spacing(Spacing::Small.size()),
                        
                        spacer(Orientations::Vertical, Spacing::Fill),
                    ]
                    .spacing(Spacing::None.size())
                ]
                .spacing(Spacing::None.size())
            )
            .direction(Direction::Horizontal(Scrollbar::hidden()))
            .into()
        },
    )
}

/// A panel for filtering transactions by tag.
pub fn filter_tag_panel<'a>(
    app: &'a App,
    tag: Tag,
    filter: Filters
) -> Element<'a, Signal> {
    let signal = if app.bank.is_tag_filtered(tag.clone(), filter) {
        RemoveFilterTag(tag.clone(), filter)
    } else {
        AddFilterTag(tag.clone(), filter)
    };
    let color = if app.bank.is_tag_filtered(tag.clone(), filter) {
        app.bank.tag_registry.get(&tag)
    } else {
        MaterialColors::Background
    };
    
    panel_button(
        app,
        Materials::RimmedPlastic,
        color,
        3,
        true,
        ButtonShapes::Minimal,
        ui_string(app, 1, tag.get_label().to_string(), TextSizes::Interactable),
        signal,
        true,
    )
}