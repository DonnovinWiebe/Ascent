use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use crate::vault::transaction::{Id, Months, Tag, Transaction};

/// Determines whether the `Filter` must match all filters (AND) or any filter (OR).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterModes {
    Or,
    And,
}



/// Generates a filtered collection of `Transaction`s based on a set of filters.
pub struct Filter {
    /// Whether each `Transaction` must match all filters (AND) or any filter (OR).
    mode: FilterModes,
    /// The year to filter by.
    year: Option<u32>,
    /// The `Month` to filter by.
    month: Option<Months>,
    /// The `Tag`s to filter by.
    tags: Vec<Tag>,
    /// The search terms to filter by.
    search_terms: Vec<String>,
    /// The filtered collection of `Transaction`s.
    filtered_ids: Vec<Id>,
}
impl Default for Filter {
    /// Creates a new empty `Filter`.
    fn default() -> Filter {
        Filter::new()
    }
}
impl Filter {
    // initializing
    /// Creates a new empty `Filter`.
    #[must_use]
    fn new() -> Filter {
        Filter {
            mode: FilterModes::And,
            year: None,
            month: None,
            tags: Vec::new(),
            search_terms: Vec::new(),
            filtered_ids: Vec::new()
        }
    }
    
    
    
    // management
    /// Toggles the `mode`.
    #[must_use]
    pub fn toggle_mode(&mut self, transactions: &[Transaction]) -> ResultStack<()> {
        if let FilterModes::Or = self.mode { self.mode = FilterModes::And; }
        else { self.mode = FilterModes::Or; }
        self.filter(transactions)
    }
    
    /// Sets the `year`.
    #[must_use]
    pub fn set_year(&mut self, year: u32, transactions: &[Transaction]) -> ResultStack<()> {
        self.year = Some(year);
        self.filter(transactions)
    }
    
    /// Clears the `year`.
    #[must_use]
    pub fn clear_year(&mut self, transactions: &[Transaction]) -> ResultStack<()> {
        self.year = None;
        self.filter(transactions)
    }
    
    /// Sets the `month`.
    #[must_use]
    pub fn set_month(&mut self, month: Months, transactions: &[Transaction]) -> ResultStack<()> {
        self.month = Some(month);
        self.filter(transactions)
    }
    
    /// Clears the `month`.
    #[must_use]
    pub fn clear_month(&mut self, transactions: &[Transaction]) -> ResultStack<()> {
        self.month = None;
        self.filter(transactions)
    }
    
    /// Adds a given `Tag`.
    #[must_use]
    pub fn add_tag(&mut self, tag: &Tag, transactions: &[Transaction]) -> ResultStack<()> {
        self.tags.push(tag.clone());
        self.tags = Tag::sorted(self.tags.clone());
        self.filter(transactions)
    }
    
    /// Removes a given `Tag`.
    #[must_use]
    pub fn remove_tag(&mut self, tag: &Tag, transactions: &[Transaction]) -> ResultStack<()> {
        self.tags.retain(|t| t != tag);
        self.filter(transactions)
    }
    
    /// Clears all `Tag`s.
    #[must_use]
    pub fn clear_tags(&mut self, transactions: &[Transaction]) -> ResultStack<()> {
        self.tags.clear();
        self.filter(transactions)
    }
    
    /// Adds a given search term.
    #[must_use]
    pub fn add_search_term(&mut self, search_term: &str, transactions: &[Transaction]) -> ResultStack<()> {
        self.search_terms.push(search_term.to_lowercase());
        self.search_terms.sort();
        self.filter(transactions)
    }
    
    /// Removes a given search term.
    #[must_use]
    pub fn remove_search_term(&mut self, search_term: &str, transactions: &[Transaction]) -> ResultStack<()> {
        self.search_terms.retain(|t| t.clone() != search_term.to_lowercase());
        self.filter(transactions)
    }
    
    /// Clears all search terms.
    #[must_use]
    pub fn clear_search_terms(&mut self, transactions: &[Transaction]) -> ResultStack<()> {
        self.search_terms.clear();
        self.filter(transactions)
    }
    
    /// Filters the source list based on the current filters.
    #[must_use]
    #[allow(clippy::too_many_lines)] // this holds the main filtering logic for what transactions are displayed at any given time, and is going to be large
    pub fn filter(&mut self, transactions: &[Transaction]) -> ResultStack<()> {
        // clears the collection before adding new transactions
        self.filtered_ids.clear();

        // checks which filters are set
        let is_year_set = self.year.is_some();
        let is_month_set = self.month.is_some();
        let is_tag_set = !self.tags.is_empty();
        let is_search_term_set = !self.search_terms.is_empty();
        let no_filters_set = !(is_year_set || is_month_set || is_tag_set || is_search_term_set);

        match self.mode {
            // filters each transactions based on the mode
            FilterModes::Or => {
                for transaction in transactions {
                    // tracks if the various filters pass
                    let mut does_year_filter_pass = false;
                    let mut does_month_filter_pass = false;
                    let mut does_tag_filter_pass = false;
                    let mut does_search_term_filter_pass = false;
                    
                    // checks the filter year
                    if let Some(year) = self.year && transaction.date.get_year() == year {
                        does_year_filter_pass = true;
                    }
                    
                    // checks the filter month
                    if let Some(month) = self.month && transaction.date.get_month() == month {
                        does_month_filter_pass = true;
                    }
                    
                    // checks each filter tag
                    for tag in &self.tags {
                        if transaction.has_tag(tag) {
                            does_tag_filter_pass = true;
                            break;
                        }
                    }
                    
                    // checks each search term
                    for search_term in &self.search_terms {
                        if transaction.value.amount().to_string().to_lowercase().contains(search_term) {
                            does_search_term_filter_pass = true;
                            break;
                        }
                        if transaction.date.display().to_lowercase().contains(search_term) {
                            does_search_term_filter_pass = true;
                            break;
                        }
                        if transaction.description.to_lowercase().contains(search_term) {
                            does_search_term_filter_pass = true;
                            break;
                        }
                        for tag in transaction.tags.clone() {
                            if tag.get_label().to_lowercase().contains(search_term) {
                                does_search_term_filter_pass = true;
                                break;
                            }
                            if does_search_term_filter_pass { break; }
                        }
                    }
                    
                    // filters
                    let id_result = ResultStack::from_option(transaction.get_id(), "Tried to filter a transaction without an id!");
                    match id_result {
                        Pass(id) => {
                            if no_filters_set || does_year_filter_pass || does_month_filter_pass || does_tag_filter_pass || does_search_term_filter_pass {
                                self.filtered_ids.push(id); 
                            }
                        },
                        Fail(_) => { return id_result.empty_type().fail("Failed to filter transactions"); },
                    }
                }
            }
            
            FilterModes::And => {
                for transaction in transactions {
                    // tracks if the various filters pass
                    let mut does_year_filter_pass = false;
                    let mut does_month_filter_pass = false;
                    let mut does_tag_filter_pass = false;
                    let mut does_search_term_filter_pass = false;
                    
                    // keeps track of if any filters have failed
                    let mut wont_pass = false;
                    
                    // checks the filter year
                    if let Some(year) = self.year {
                        if transaction.date.get_year() == year {
                            does_year_filter_pass = true;
                        }
                        else {
                            does_year_filter_pass = false;
                            wont_pass = true;
                        }
                    }
                    if !is_year_set { does_year_filter_pass = true; }
                    
                    // checks the filter month
                    if !wont_pass {
                        if let Some(month) = self.month {
                            if transaction.date.get_month() == month {
                                does_month_filter_pass = true;
                            }
                            else {
                                does_month_filter_pass = false;
                                wont_pass = true;
                            }
                        }
                        if !is_month_set { does_month_filter_pass = true; }
                    }
                    
                    // checks each filter tag
                    if !wont_pass {
                        for tag in &self.tags {
                            if transaction.has_tag(tag) {
                                does_tag_filter_pass = true;
                            }
                            else {
                                does_tag_filter_pass = false;
                                wont_pass = true;
                                break;
                            }
                        }
                        if !is_tag_set { does_tag_filter_pass = true; }
                    }
                    
                    // checks each search term
                    if !wont_pass {
                        for search_term in &self.search_terms {
                            let mut term_found = false;
                            if transaction.value.amount().to_string().to_lowercase().contains(search_term) {
                                term_found = true;
                            }
                            if transaction.date.display().to_lowercase().contains(search_term) {
                                term_found = true;
                            }
                            if transaction.description.to_lowercase().contains(search_term) {
                                term_found = true;
                            }
                            for tag in transaction.tags.clone() {
                                if tag.get_label().to_lowercase().contains(search_term) {
                                    term_found = true;
                                }
                            }
                            
                            if !term_found {
                                does_search_term_filter_pass = false;
                                wont_pass = true;
                                break;
                            }
                        }
                        if !is_search_term_set { does_search_term_filter_pass = true; }
                    }
                    
                    // filters
                    let id_result = ResultStack::from_option(transaction.get_id(), "Tried to filter a transaction without an id!");
                    match id_result {
                        Pass(id) => {
                            if no_filters_set || (!wont_pass && does_year_filter_pass && does_month_filter_pass && does_tag_filter_pass && does_search_term_filter_pass) {
                                self.filtered_ids.push(id); 
                            }
                        },
                        Fail(_) => { return id_result.empty_type().fail("Failed to filter transactions"); },
                    }
                }
            }
        }
        
        Pass(())
    }
    
    
    
    // data retrieval and parsing
    /// Gets the `mode`.
    #[must_use]
    pub fn get_filter_mode(&self) -> FilterModes { self.mode }
    
    /// Gets the optional filtered `year`.
    #[must_use]
    pub fn get_filter_year(&self) -> Option<u32> { self.year }
    
    /// Gets the optional filtered `month`.
    #[must_use]
    pub fn get_filter_month(&self) -> Option<Months> { self.month }
    
    /// Checks if the given `Tag` is filtered.
    #[must_use]
    pub fn is_tag_filtered(&self, tag: &Tag) -> bool { self.tags.contains(tag) }
    
    /// Gets the `search_terms`.
    #[must_use]
    pub fn get_search_terms(&self) -> Vec<String> { self.search_terms.clone() }

    /// Gets the list of filtered `Transaction` `Id`s.
    #[must_use]
    pub fn get_filtered_ids(&self) -> Vec<Id> { self.filtered_ids.clone() }
}