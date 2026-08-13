use iced::{Theme, widget::text_editor::Content};
use materialui::{components::DatePickerModes, materials::MaterialThemes};
use schrod::Schrod;

use crate::{container::{app::Pages, warnings::Warnings}, vault::{bank::Bank, parse::CashFlow, ring_parse::{RingParse, Segment}, save_engine::SaveData, transaction::{Date, Id, Months, Tag, Transaction}, trend_parse::{Intervals, TrendParse}}};

// App related states
/// Manages the overall state for `App`-related information.
pub struct AppState {
    /// The material theme.
    /// This controls how the `App` looks.
    material_theme: MaterialThemes,
    /// The iced theme.
    /// This is only used by `Iced` in the background.
    iced_theme: Theme,
    /// The critical errors.
    critical_errors: Vec<String>,
    /// The minor errors from the last interaction.
    minor_errors: Vec<String>,
    /// The warnings from the last interaction.
    warnings: Vec<Warnings>,
    /// Tracks if the `App` is currently logging minor errors and warnings to only log
    /// them from the last interaction.
    is_logging_minor_errors: bool,
    /// The current page.
    page: Pages,
    /// Tracks if the user is currently asking for help on the current `page`.
    is_helping: bool,
}
impl AppState {
    // initializing
    /// Creates a new 'AppState' object.
    #[must_use]
    pub fn new(material_theme: MaterialThemes, iced_theme: Theme) -> AppState {
        AppState {
            material_theme,
            iced_theme,
            critical_errors: Vec::new(),
            minor_errors: Vec::new(),
            warnings: Vec::new(),
            is_logging_minor_errors: true,
            page: Pages::Transactions,
            is_helping: false,
        }
    }



    // getting
    /// Gets the current `material_theme.
    #[must_use]
    pub fn get_material_theme(&self) -> MaterialThemes { self.material_theme }

    /// Gets the current `iced_theme`.
    #[must_use]
    pub fn get_iced_theme(&self) -> Theme { self.iced_theme.clone() }

    /// Gets the current `critical_errors`.
    #[must_use]
    pub fn get_critical_errors(&self) -> Vec<String> { self.critical_errors.clone() }

    /// Gets the current `minor_errors`.
    #[must_use]
    pub fn get_minor_errors(&self) -> Vec<String> { self.minor_errors.clone() }

    /// Gets the current `warnings`.
    #[must_use]
    pub fn get_warnings(&self) -> Vec<Warnings> { self.warnings.clone() }

    /// Checks if there are any active warnings or minor errors.
    #[must_use]
    pub fn is_warning(&self, bank: &Bank) -> bool { (self.warnings.len() > 0 || self.minor_errors.len() > 0) && bank.get_ledger().len() > 0 }

    /// Gets the current `page`.
    #[must_use]
    pub fn get_page(&self) -> Pages { self.page }

    /// Gets the current `is_helping` state.
    #[must_use]
    pub fn is_helping(&self) -> bool { self.is_helping }



    // udpating
    /// Updates the current `material_theme`.
    pub fn update_material_theme(&mut self, new_material_theme: MaterialThemes) { self.material_theme = new_material_theme; }

    /// Updates the current `iced_theme`.
    pub fn update_iced_theme(&mut self, new_iced_theme: Theme) { self.iced_theme = new_iced_theme; }

    /// Sorts a given error into the critical or minor error list.
    pub fn pass_error<T>(&mut self, error: Schrod<T>) {
        // logs minor errors
        if error.is_silenced() {
            // clears minor errors if this is the first minor error found since last interaction loop
            if self.is_logging_minor_errors == false {
                self.minor_errors.clear();
                self.is_logging_minor_errors = true;
            }
            // logs the error
            self.minor_errors.extend(error.results());
        }

        // logs critical errors
        else { self.critical_errors.extend(error.results()); }
    }
    
    /// Flags the end of a user interaction since some interactions chain several `Signal`s in sequence.
    /// This should only be used after `Signal` chains that can be directly caused by a user.
    pub fn finished_interaction(&mut self, bank: &Bank) {
        // scans for warnings after the end of an interaction loop
        let warnings = Warnings::scan(bank);
        self.warnings = warnings;
        
        // marks the end of the interaction loop so the minor errors log can be reset upon a new interaction
        self.is_logging_minor_errors = false;
    }

    /// Clears the `critical_errors`.
    pub fn clear_critical_errors(&mut self) { self.critical_errors.clear(); }
    
    /// Clears the `minor_errors`.
    pub fn clear_minor_errors(&mut self) { self.minor_errors.clear(); }
    
    /// Clears the `warnings`.
    pub fn clear_warnings(&mut self) { self.warnings.clear(); }
    
    /// Updates the current `page`.
    pub fn update_page(&mut self, new_page: Pages) { self.page = new_page; }

    /// Updates the `is_helping` state.
    pub fn update_is_helping(&mut self, is_helping: bool) { self.is_helping = is_helping; }
}



/// Manages saving and loading related information for the `App`.
pub struct SaveState {
    /// Tracks if the `App` saved successfully on the last save attempt.
    saved_successfully: bool,
    /// Tracks if the `App` loaded successfully on launch.
    loaded_successfully: bool,
    /// Holds the data that the user wants to import.
    import_data: Option<SaveData>,
    /// Holds the legacy data that the user wants to import.
    /// This should not be used by anyone but me as previous projects that use this format
    /// were never made public. This may even be removed entirely at a later date.
    legacy_import_data: Option<Vec<Transaction>>,
}
impl SaveState {
    // initializing
    /// Creates a new `SaveState` object.
    #[must_use]
    pub fn new(loaded_successfully: bool) -> SaveState {
        SaveState {
            saved_successfully: true,
            loaded_successfully: loaded_successfully,
            import_data: None,
            legacy_import_data: None,
        }
    }



    // getting
    /// Gets the current `saved_successfully` state.
    #[must_use]
    pub fn saved_successfully(&self) -> bool { self.saved_successfully }

    /// Gets the current `loaded_successfully` state.
    #[must_use]
    pub fn loaded_successfully(&self) -> bool { self.loaded_successfully }

    /// Gets the current `import_data`.
    #[must_use]
    pub fn get_import_data(&self) -> &Option<SaveData> { &self.import_data }

    /// Gets the current `legacy_import_data`.
    #[must_use]
    pub fn get_legacy_import_data(&self) -> &Option<Vec<Transaction>> { &self.legacy_import_data }



    // updating
    /// Updates the `saved_successfully` state.
    pub fn update_saved_successfully(&mut self, saved_successfully: bool) { self.saved_successfully = saved_successfully; }

    /// Updates the `loaded_successfully` state.
    pub fn update_loaded_successfully(&mut self, loaded_successfully: bool) { self.loaded_successfully = loaded_successfully; }

    /// Updates the `import_data` state.
    pub fn update_import_data(&mut self, new_import_data: Option<SaveData>) { self.import_data = new_import_data; }

    /// Updates the `legacy_import_data` state.
    pub fn update_legacy_import_data(&mut self, new_legacy_import_data: Option<Vec<Transaction>>) { self.legacy_import_data = new_legacy_import_data; }
}



/// Manages settings related information for the `App`.
pub struct SettingsState {
    /// Tracks the user input for setting a new main `Currency`.
    new_main_currency_string: String,
    /// Tracks the user input for setting a new time price.
    new_time_price_string: String,
}
impl SettingsState {
    // initializing
    /// Creates a new `SettingsState`.
    #[must_use]
    pub fn new() -> SettingsState {
        SettingsState {
            new_main_currency_string: String::new(),
            new_time_price_string: String::new(),
        }
    }



    // getting
    /// Returns the current `new_main_currency_string`.
    #[must_use]
    pub fn get_new_main_currency_string(&self) -> &str { &self.new_main_currency_string }

    /// Returns the current `new_time_price_string`.
    #[must_use]
    pub fn get_new_time_price_string(&self) -> &str { &self.new_time_price_string }
    


    // updating
    /// Updates the `new_main_currency_string`.
    pub fn update_new_main_currency_string(&mut self, new_main_currency_string: String) { self.new_main_currency_string = new_main_currency_string; }

    /// Updates the `new_time_price_string`.
    pub fn update_new_time_price_string(&mut self, new_time_price_string: String) { self.new_time_price_string = new_time_price_string; }
}



// bank related states
/// Tracks all `Bank` related information for the `App`.
pub struct BankState {
    /// The cash flow for the current `Filter` settings.
    cash_flow_result: Schrod<CashFlow>,
}
impl BankState {
    // initializing
    /// Creates a new `BankState`.
    #[must_use]
    pub fn new(cash_flow_result: Schrod<CashFlow>) -> BankState {
        BankState { cash_flow_result }
    }



    // getting
    /// Returns the current `cash_flow_result`.
    #[must_use]
    pub fn get_cash_flow_result(&self) -> &Schrod<CashFlow> { &self.cash_flow_result }
    

    
    // updating
    /// Updates the `cash_flow_result`.
    pub fn update_cash_flow_result(&mut self, new_cash_flow_result: Schrod<CashFlow>) { self.cash_flow_result = new_cash_flow_result; }
    
}



/// Tracks all information related to creating or editing a `Transaction` in the `App`.
pub struct TransactionState {
    /// The 'id' that this state is referencing (optional).
    id: Option<Id>,
    /// The user input for the `Value` of a `Transaction`.
    value_string: String,
    /// The user input for the `Currency` of a `Transaction`.
    currency_string: String,
    /// The user input for the `Description` of a `Transaction`.
    description_content: Content,
    /// The state object for the `Date` of a `Transaction`.
    date_picker_state: DatePickerState,
    /// The user input for a new `Tag` of a `Transaction`.
    current_tag_string: String,
    /// The state for the current `Tag`s of a `Transaction`.
    tags: Vec<Tag>,
    /// Tracks if a the given `Transaction` is primed for deletion.
    is_delete_primed: bool,
}
impl TransactionState {
    // initializing
    /// Creates a new `TransactionState`.
    #[must_use]
    pub fn new(id: Option<Id>, date: Date) -> TransactionState {
        TransactionState {
            id,
            value_string: String::new(),
            currency_string: String::new(),
            description_content: Content::default(),
            date_picker_state: DatePickerState::new(date),
            current_tag_string: String::new(),
            tags: Vec::new(),
            is_delete_primed: false,
        }
    }

    

    // getters
    /// Gets the current `id`.
    #[must_use]
    pub fn get_id(&self) -> Option<Id> { self.id }
    
    /// Gets the current `value_string`.
    #[must_use]
    pub fn get_value_string(&self) -> &str { &self.value_string }

    /// Gets the current `currency_string`.
    #[must_use]
    pub fn get_currency_string(&self) -> &str { &self.currency_string }

    /// Gets the current `description_content` (immutable).
    #[must_use]
    pub fn get_description_content(&self) -> &Content { &self.description_content }

    /// Gets the current `description_content` (mutable).
    #[must_use]
    pub fn get_description_content_mut(&mut self) -> &mut Content { &mut self.description_content }

    /// Gets the current `DatePickerState` (immutable ref).
    #[must_use]
    pub fn get_date_picker_state(&self) -> &DatePickerState { &self.date_picker_state }
    
    /// Gets the current `DatePickerState` (mutable ref).
    #[must_use]
    pub fn get_date_picker_state_mut(&mut self) -> &mut DatePickerState { &mut self.date_picker_state }

    /// Gets the `current_tag_string`.
    #[must_use]
    pub fn get_current_tag_string(&self) -> &str { &self.current_tag_string }

    /// Gets the current `tags`.
    #[must_use]
    pub fn get_tags(&self) -> Vec<Tag> { self.tags.clone() }

    /// Gets the current `is_delete_primed` state.
    #[must_use]
    pub fn is_delete_primed(&self) -> bool { self.is_delete_primed }



    // updating
    /// Updates the `id`.
    pub fn update_id(&mut self, new_id: Option<Id>) { self.id = new_id; }
    
    /// Updates the `value_string`.
    pub fn update_value_string(&mut self, value_string: String) { self.value_string = value_string; }

    /// Updates the `currency_string`.
    pub fn update_currency_string(&mut self, currency_string: String) { self.currency_string = currency_string; }

    /// Updates the `description_content`.
    pub fn update_description_content(&mut self, description_content: Content) { self.description_content = description_content; }
    
    /// Updates the `current_tag_string`.
    pub fn update_current_tag_string(&mut self, current_tag_string: String) { self.current_tag_string = current_tag_string; }

    /// Adds a new `Tag` to `tags`.
    pub fn add_tag(&mut self, new_tag: Tag) {
        self.tags.push(new_tag);
        self.tags = Tag::sorted(&self.tags);
    }
    
    /// Updates the `tags`.
    pub fn update_tags(&mut self, tags: Vec<Tag>) { self.tags = tags; }

    /// Updates if the given `Transaction` is primed for deletion.
    pub fn update_is_delete_primed(&mut self, is_delete_primed: bool) { self.is_delete_primed = is_delete_primed; }
}



/// Tracks all information related to picking a new `Date`.
pub struct DatePickerState {
    /// The mode of the date picker.
    mode: DatePickerModes,
    /// The current year of the date picker.
    current_year: u32,
    /// The current `Month` of the date picker.
    current_month: Months,
    /// The selected `Date` of the date picker.
    selected_date: Date,
}
impl DatePickerState {
    // initializing
    /// Creates a new `DatePickerState`.
    #[must_use]
    pub fn new(date: Date) -> DatePickerState {
        DatePickerState {
            mode: DatePickerModes::Hidden,
            current_year: date.get_year(),
            current_month: date.get_month(),
            selected_date: date,
        }
    }



    // getters
    /// Returns the current `mode`.
    #[must_use]
    pub fn get_mode(&self) -> DatePickerModes { self.mode }

    /// Returns the `current_year`.
    #[must_use]
    pub fn get_current_year(&self) -> u32 { self.current_year }

    /// Returns the `current_month`.
    #[must_use]
    pub fn get_current_month(&self) -> Months { self.current_month }

    /// Returns the current `selected_date`.
    #[must_use]
    pub fn get_selected_date(&self) -> Date { self.selected_date }



    // updating
    /// Updates the `mode`.
    pub fn update_mode(&mut self, new_mode: DatePickerModes) { self.mode = new_mode; }

    /// Updates the `current_year`.
    pub fn update_current_year(&mut self, new_year: u32) { self.current_year = new_year; }

    /// Updates the `current_month`.
    pub fn update_current_month(&mut self, new_month: Months) { self.current_month = new_month; }

    /// Updates the `selected_date`.
    pub fn update_selected_date(&mut self, new_date: Date) { self.selected_date = new_date; }
}



/// Manages the states of all `TagRegistrationSlip`s in the `App`.
pub struct TagRegistrationSlipStateManager {
    /// The states of all the slips.
    /// The slips correspond to the `Tag`s in the `Bank`.
    slips_states: Vec<TagRegistrationSlipState>,
}
impl TagRegistrationSlipStateManager {
    // initializing
    /// Creates a new `TagRegistrationSlipStateManager` with the given `Tag`s.
    #[must_use]
    pub fn new(tags: Vec<Tag>) -> TagRegistrationSlipStateManager {
        TagRegistrationSlipStateManager {
            slips_states: tags.into_iter().map(TagRegistrationSlipState::new).collect(),
        }
    }



    // getting
    /// Gets the current states of all slips.
    #[must_use]
    pub fn get_states(&self) -> &Vec<TagRegistrationSlipState> {
        &self.slips_states
    }



    // updating
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
    // initializing
    /// Creates a new `TagRegistrationSlipState` for the given `Tag`.
    #[must_use]
    fn new(tag: Tag) -> TagRegistrationSlipState {
        TagRegistrationSlipState { tag, is_expanded: false }
    }



    // getting
    /// Gets the `Tag` associated with the given slip.
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
    primary_filter_current_search_term_string: String,
    deep_dive_1_filter_current_search_term_string: String,
    deep_dive_2_filter_current_search_term_string: String,
}
impl FilterState {
    // initializing
    /// Creates a new `FilterState` with the given search terms.
    #[must_use]
    pub fn new() -> FilterState {
        FilterState {
            primary_filter_current_search_term_string: String::new(),
            deep_dive_1_filter_current_search_term_string: String::new(),
            deep_dive_2_filter_current_search_term_string: String::new(),
        }
    }


    
    // getting
    /// Gets the `primary_filter_current_search_term_string`.
    #[must_use]
    pub fn get_primary_filter_current_search_term_string(&self) -> String { self.primary_filter_current_search_term_string.clone() }

    /// Gets the `deep_dive_1_filter_current_search_term_string`.
    #[must_use]
    pub fn get_deep_dive_1_filter_current_search_term_string(&self) -> String { self.deep_dive_1_filter_current_search_term_string.clone() }

    /// Gets the `deep_dive_2_filter_current_search_term_string`.
    #[must_use]
    pub fn get_deep_dive_2_filter_current_search_term_string(&self) -> String { self.deep_dive_2_filter_current_search_term_string.clone() }



    // updating
    /// Updates the `primary_filter_current_search_term_string`.
    pub fn update_primary_filter_current_search_term_string(&mut self, new_search_term: String) { self.primary_filter_current_search_term_string = new_search_term; }

    /// Updates the `deep_dive_1_filter_current_search_term_string`.
    pub fn update_deep_dive_1_filter_current_search_term_string(&mut self, new_search_term: String) { self.deep_dive_1_filter_current_search_term_string = new_search_term; }

    /// Updates the `deep_dive_2_filter_current_search_term_string`.
    pub fn update_deep_dive_2_filter_current_search_term_string(&mut self, new_search_term: String) { self.deep_dive_2_filter_current_search_term_string = new_search_term; }
}



/// Tracks all information for displaying Ring Charts for the `App`.
pub struct RingChartsState {
    /// Tracks if the Ring Charts are ready to be displayed.
    is_ready: bool,
    /// The `RingParse` result for all earning `Transaction`s.
    earning_result: Schrod<RingParse>,
    /// The `RingParse` result for all spending `Transaction`s.
    spending_result: Schrod<RingParse>,
    /// Tracks which `Segment` the user is currenlty hovering over.
    hovered_segment: Option<Segment>,
}
impl RingChartsState {
    // initializing
    /// Creates a new `RingChartsState`.
    pub fn new() -> RingChartsState {
        RingChartsState {
            is_ready: false,
            earning_result: Schrod::new_fail("No RingParse has been created.", "RingChartsState::new()"),
            spending_result: Schrod::new_fail("No RingParse has been created.", "RingChartsState::new()"),
            hovered_segment: None,
        }
    }

    

    // getting
    /// Checks if the `RingChartsState` is ready.
    #[must_use]
    pub fn is_ready(&self) -> bool { self.is_ready }

    /// Gets the current `earning_result` (immutable).
    #[must_use]
    pub fn earning_result(&self) -> &Schrod<RingParse> { &self.earning_result }

    /// Gets the current `earning_result` (mutable).
    #[must_use]
    pub fn earning_result_mut(&mut self) -> &mut Schrod<RingParse> { &mut self.earning_result }

    /// Gets the current `spending_result` (immutable).
    #[must_use]
    pub fn spending_result(&self) -> &Schrod<RingParse> { &self.spending_result }

    /// Gets the current `spending_result` (mutable).
    #[must_use]
    pub fn spending_result_mut(&mut self) -> &mut Schrod<RingParse> { &mut self.spending_result }

    /// Gets the current `hovered_segment`.
    #[must_use]
    pub fn hovered_segment(&self) -> Option<&Segment> { self.hovered_segment.as_ref() }



    // updating
    /// Updates the `is_ready` state.
    pub fn update_is_ready(&mut self, is_ready: bool) { self.is_ready = is_ready; }
    
    /// Updates the `earning_result`.
    pub fn update_earning_result(&mut self, earning_result: Schrod<RingParse>) { self.earning_result = earning_result; }

    /// Updates the `spending_result`.
    pub fn update_spending_result(&mut self, spending_result: Schrod<RingParse>) { self.spending_result = spending_result; }

    /// Updates the `hovered_segment`.
    pub fn update_hovered_segment(&mut self, hovered_segment: Option<Segment>) { self.hovered_segment = hovered_segment; }
}



/// Tracks all Trends Chart related information for the `App`.
pub struct TrendsState {
    /// Tracks if the Trends Chart is ready to be displayed.
    is_ready: bool,
    /// Tracks if the current `TrendParse` was created successfully.
    trend_parse_result: Schrod<TrendParse>,
    /// The time interval of the Trends Chart.
    interval: Intervals,
    /// Tracks if an overall balance line is being displayed.
    show_balance_line: bool,
    /// Tracks which `Tag`s included in the Trends Chart.
    tags: Vec<Tag>,
    /// Tracks how many time intervals are being displayed.
    length: usize,
    /// Tracks the last `Date` included in the Trends Chart.
    last_trending_date: Date,
}
impl TrendsState {
    // initializing
    /// Creates a new `TrendsState` with the given `interval`.
    pub fn new(date: Date) -> TrendsState {
        TrendsState {
            is_ready: false,
            trend_parse_result: Schrod::new_fail("No TrendParse has been created.", "TrendsState::new()"),
            interval: Intervals::Quarterly,
            show_balance_line: true,
            tags: Vec::new(),
            length: 6,
            last_trending_date: date,
        }
    }



    // getting
    /// Checks if the `TrendsState` is ready.
    #[must_use]
    pub fn is_ready(&self) -> bool { self.is_ready }

    /// Gets the current `trend_parse_result`.
    #[must_use]
    pub fn trend_parse_result(&self) -> &Schrod<TrendParse> { &self.trend_parse_result }

    /// Gets the current `interval`.
    #[must_use]
    pub fn interval(&self) -> Intervals { self.interval }

    /// Gets the current `show_balance_line` state.
    #[must_use]
    pub fn show_balance_line(&self) -> bool { self.show_balance_line }

    /// Gets the current trending `tags`.
    #[must_use]
    pub fn tags(&self) -> Vec<Tag> { self.tags.clone() }

    /// Gets the current trend `length`.
    #[must_use]
    pub fn length(&self) -> usize { self.length }

    /// Gets the current `last_trending_date`.
    #[must_use]
    pub fn last_trending_date(&self) -> Date { self.last_trending_date }


    
    // updating
    /// Updates the `is_ready` state.
    pub fn update_is_ready(&mut self, is_ready: bool) { self.is_ready = is_ready; }

    /// Updates the `trend_parse_result`.
    pub fn update_trend_parse_result(&mut self, trend_parse_result: Schrod<TrendParse>) { self.trend_parse_result = trend_parse_result; }

    /// Updates the `interval`.
    pub fn update_interval(&mut self, interval: Intervals) { self.interval = interval; }

    /// Toggles the `show_balance_line`.
    pub fn toggle_show_balance_line(&mut self) { self.show_balance_line = !self.show_balance_line; }

    /// Adds a new `Tag` to `tags`.
    pub fn add_tag(&mut self, new_tag: Tag) {
        self.tags.push(new_tag);
        self.tags = Tag::sorted(&self.tags);
    }
    
    /// Updates the trending `tags`.
    pub fn update_tags(&mut self, tags: Vec<Tag>) { self.tags = tags; }

    /// Updates the trend `length`.
    pub fn update_length(&mut self, length: usize) { self.length = length; }

    /// Updates the `last_trending_date`.
    pub fn update_last_trending_date(&mut self, last_trending_date: Date) { self.last_trending_date = last_trending_date; }
}