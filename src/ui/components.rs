use iced::Color;
use iced::widget::*;
use crate::ui::palette::{ColorThemes, Palette};
use crate::vault::parse::*;
use crate::vault::transaction::{Transaction, ValueDisplayFormats};

pub const STANDARD_PADDING: f32 = 10.0;
pub const STANDARD_CORNER_RADIUS: f32 = 5.0;

pub fn rounded_background_style(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(color.into()),
        border: iced::Border::default().rounded(STANDARD_CORNER_RADIUS),
        ..Default::default()
    }
}

pub fn cash_flow_pane<'a, Signal: 'a>(cash_flow_grouping: CashFlowGrouping, value_display_format: ValueDisplayFormats) -> Container<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            container(
                column(cash_flow_grouping.money_list.into_iter().map(|money| {
                    text(money.to_string()).into()
                }))
            )
                .padding(STANDARD_PADDING)
                .style(rounded_background_style(Palette::Foreground.themed(ColorThemes::Dark)))
        }

        ValueDisplayFormats::Time(price) => {
            container(
                column(cash_flow_grouping.money_list.into_iter().map(|money| {
                    text(Transaction::get_time_price(&money, price)).into()
                }))
            )
                .padding(STANDARD_PADDING)
                .style(rounded_background_style(Palette::Foreground.themed(ColorThemes::Dark)))
        }
    }

}


// content: impl Into<Element<'a, Message>>