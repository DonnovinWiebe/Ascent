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
use crate::pages::filter_ui::*;
use crate::pages::transaction_management_pages::current_tag_field;
use crate::ui::components::*;
use crate::ui::material::{MaterialColors, Materials};
use crate::vault::bank::Filters;
use crate::vault::parse::CashFlow;
use crate::vault::transaction::{Tag, TagStyles, Transaction, ValueDisplayFormats};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::parse::*;

// transactions page
pub fn settings_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    stack![
        container(
            row![
                spacer(Orientations::Horizontal, Spacing::Small),
                navigation_panel(app),
                spacer(Orientations::Horizontal, Spacing::Fill),
                spacer(Orientations::Horizontal, Spacing::Small),
            ]
            .spacing(Spacing::None.size())
        )
        .center_x(Fill),
        
        header(
            app,
            Vec::new(),
            Vec::new(),
        ),
    ]
    .width(Fill)
    .height(Fill)
}