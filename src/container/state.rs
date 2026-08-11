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

/// Manages the states of every `TagRegistrationSlip` in the `App`.
pub struct TagRegistrationSlipStateManager {
    /// The states of all the slips.
    /// The slips correspond to the `Tag`s in the `Bank`.
    slips_states: Vec<TagRegistrationSlipState>,
}
impl TagRegistrationSlipStateManager {
    /// Creates a new `TagRegistrationSlipStateManager` with the given `Tag`s.
    #[must_use]
    pub fn new(tags: Vec<Tag>) -> TagRegistrationSlipStateManager {
        TagRegistrationSlipStateManager {
            slips_states: tags.into_iter().map(TagRegistrationSlipState::new).collect(),
        }
    }
    
    /// Returns a reference to the states of all the slips.
    #[must_use]
    pub fn get_states(&self) -> &Vec<TagRegistrationSlipState> {
        &self.slips_states
    }
    
    /// Expands the slip for the given `Tag` and collapses all others.
    pub fn expand(&mut self, tag: &Tag) {
        for state in &mut self.slips_states {
            state.is_expanded = state.tag == *tag;
        }
    }
    
    /// Collapses the slip for the given `Tag`.
    pub fn collapse(&mut self, tag: &Tag) {
        for state in &mut self.slips_states {
            if state.tag == *tag {
                state.is_expanded = false;
                return;
            }
        }
    }
}

/// Holds the state of a single `tag_registration_slip` (`tag_registry_page::tag_registration_slip`).
pub struct TagRegistrationSlipState {
    /// The `Tag` associated with the slip.
    tag: Tag,
    /// Whether the slip is expanded or not.
    is_expanded: bool,
}
impl TagRegistrationSlipState {
    /// Creates a new `TagRegistrationSlipState` for the given `Tag`.
    #[must_use]
    fn new(tag: Tag) -> TagRegistrationSlipState {
        TagRegistrationSlipState { tag, is_expanded: false }
    }
    
    /// Returns a reference to the `Tag` associated with the slip.
    #[must_use]
    pub fn get_tag(&self) -> &Tag {
        &self.tag
    }

    /// Cheks if the given slip is expanded.
    #[must_use]
    pub fn is_expanded(&self) -> bool {
        self.is_expanded
    }
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