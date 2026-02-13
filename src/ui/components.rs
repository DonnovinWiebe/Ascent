use iced::Color;
use iced::widget::*;
use crate::ui::palette::{ColorThemes, Palette};
use crate::vault::parse::*;
use crate::vault::transaction::{Transaction, ValueDisplayFormats};

// standard values
pub const STANDARD_PADDING: f32 = 10.0;
pub const STANDARD_CORNER_RADIUS: f32 = 16.0;



// standard parts
/// Returns a rounded background Style.
pub fn rounded_background_style(color: Color) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(color.into()),
        border: iced::Border::default().rounded(STANDARD_CORNER_RADIUS),
        ..Default::default()
    }
}



// bank overview parts
/// Returns a cash flow panel.
pub fn cash_flow_panel<'a, Signal: 'a>(cash_flow_grouping: CashFlowGrouping, value_display_format: ValueDisplayFormats) -> Container<'a, Signal> {
    match value_display_format {
        ValueDisplayFormats::Dollars => {
            container(
                column(cash_flow_grouping.money_list.into_iter().map(|money| {
                    text(money.to_string()).into() // todo create standard function to format Money values (with cu
                }))
            )
                .padding(STANDARD_PADDING)
                .style(rounded_background_style(Palette::Foreground.themed(ColorThemes::Dark)))
        }

        ValueDisplayFormats::Time(price) => {
            container(
                column(cash_flow_grouping.money_list.into_iter().map(|money| {
                    text(Transaction::get_time_price(&money, price)).into() // todo create standard function to format time prices
                }))
            )
                .padding(STANDARD_PADDING)
                .style(rounded_background_style(Palette::Foreground.themed(ColorThemes::Dark)))
        }
    }

}


// content: impl Into<Element<'a, Message>>