use iced::{Theme, widget::text_editor::Content};
use materialui::{components::DatePickerModes, materials::MaterialThemes};
use schrod::Schrod;

use crate::{container::{app::Pages, warnings::Warnings}, vault::{parse::CashFlow, ring_parse::{RingParse, Segment}, save_engine::SaveData, transaction::{Date, Months, Tag, Transaction}, trend_parse::{Intervals, TrendParse}}};

// App related states
pub struct AppState {
    pub material_theme: MaterialThemes,
    pub iced_theme: Theme,
    pub critical_errors: Vec<String>,
    pub minor_errors: Vec<String>,
    pub warnings: Vec<Warnings>,
    pub is_logging_minor_errors: bool,
    pub page: Pages,
    pub is_helping: bool,
}

pub struct SaveState {
    pub saved_successfully: bool,
    pub loaded_successfully: bool,
    pub import_data: Option<SaveData>,
    pub legacy_import_data: Option<Vec<Transaction>>,
}

pub struct SettingsState {
    pub new_main_currency_string: String,
    pub new_time_price_string: String,
}



// bank related states
pub struct BankState {
    cash_flow_result: Schrod<CashFlow>,
}

pub struct TransactionState {
    pub value_string: String,
    pub currency_string: String,
    pub description_content: Content,
    pub date: DatePickerState,
    pub current_tag_string: String,
    pub tags: Vec<Tag>,
}

pub struct DatePickerState {
    pub mode: DatePickerModes,
    pub current_year: u32,
    pub current_month: Months,
    pub selected_date: Date,
}



// data parsing related states
pub struct FilterState {
    pub primary_filter_current_search_term_string: String,
    pub deep_dive_1_filter_current_search_term_string: String,
    pub deep_dive_2_filter_current_search_term_string: String,
}

pub struct RingChartsState {
    pub is_ready: bool,
    pub earning_result: Schrod<RingParse>,
    pub spending_result: Schrod<RingParse>,
    pub hovered_segment: Option<Segment>,
}

pub struct TrendsState {
    pub is_trend_chart_ready: bool,
    pub trend_parse_result: Schrod<TrendParse>,
    pub interval: Intervals,
    pub show_balance_line: bool,
    pub tags: Vec<Tag>,
    pub length: usize,
    pub last_trending_date: Date,
}