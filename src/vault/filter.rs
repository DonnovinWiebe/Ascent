use schrod::Schrod;
use schrod::Schrod::Pass;
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
    pub fn toggle_mode(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        if let FilterModes::Or = self.mode { self.mode = FilterModes::And; }
        else { self.mode = FilterModes::Or; }
        self.filter(transactions)
    }
    
    /// Sets the `year`.
    #[must_use]
    pub fn set_year(&mut self, year: u32, transactions: &[Transaction]) -> Schrod<()> {
        self.year = Some(year);
        self.filter(transactions)
    }
    
    /// Clears the `year`.
    #[must_use]
    pub fn clear_year(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        self.year = None;
        self.filter(transactions)
    }
    
    /// Sets the `month`.
    #[must_use]
    pub fn set_month(&mut self, month: Months, transactions: &[Transaction]) -> Schrod<()> {
        self.month = Some(month);
        self.filter(transactions)
    }
    
    /// Clears the `month`.
    #[must_use]
    pub fn clear_month(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        self.month = None;
        self.filter(transactions)
    }
    
    /// Adds a given `Tag`.
    #[must_use]
    pub fn add_tag(&mut self, tag: &Tag, transactions: &[Transaction]) -> Schrod<()> {
        self.tags.push(tag.clone());
        self.tags = Tag::sorted(&self.tags);
        self.filter(transactions)
    }
    
    /// Removes a given `Tag`.
    #[must_use]
    pub fn remove_tag(&mut self, tag: &Tag, transactions: &[Transaction]) -> Schrod<()> {
        self.tags.retain(|t| t != tag);
        self.filter(transactions)
    }
    
    /// Clears all `Tag`s.
    #[must_use]
    pub fn clear_tags(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        self.tags.clear();
        self.filter(transactions)
    }

    /// Makes sure that the filtered `Tag`s all exist in the given list of `Tag`s.
    pub fn verify_filtered_tags(&mut self, existing_tags: &[Tag]) {
        self.tags.retain(|tag| existing_tags.contains(tag));
    }
    
    /// Adds a given search term.
    #[must_use]
    pub fn add_search_term(&mut self, search_term: &str, transactions: &[Transaction]) -> Schrod<()> {
        self.search_terms.push(search_term.to_lowercase());
        self.search_terms.sort();
        self.filter(transactions)
    }
    
    /// Removes a given search term.
    #[must_use]
    pub fn remove_search_term(&mut self, search_term: &str, transactions: &[Transaction]) -> Schrod<()> {
        self.search_terms.retain(|t| t.clone() != search_term.to_lowercase());
        self.filter(transactions)
    }
    
    /// Clears all search terms.
    #[must_use]
    pub fn clear_search_terms(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        self.search_terms.clear();
        self.filter(transactions)
    }
    
    /// Filters the source list based on the current filters.
    #[must_use]
    #[allow(clippy::too_many_lines)] // this holds the main filtering logic for what transactions are displayed at any given time, and is going to be large
    pub fn filter(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        // clears the collection before adding new transactions
        self.filtered_ids.clear();

        // filters
        let filter_result = match self.mode {
            FilterModes::Or => self.filter_or(transactions),
            FilterModes::And => self.filter_and(transactions),
        };
        if filter_result.is_fail() {
            return filter_result
                .fail("Failed to filter().", "Filter::filter()")
        }

        // finished successfully
        Pass(())
    }

    fn filter_or(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        // the filtered list being built
        let mut ids: Vec<Id> = Vec::new();
        // the base stats
        let is_year_set = self.year.is_some();
        let is_month_set = self.month.is_some();
        let is_tag_set = !self.tags.is_empty();
        let is_search_term_set = !self.search_terms.is_empty();
        let are_none_set = !is_year_set && !is_month_set && !is_tag_set && !is_search_term_set;

        // checking each transaction
        for transaction in transactions {
            // filters nothing if no filters are set
            if are_none_set {
                let id_result = Schrod::from_option(transaction.get_id(), "Failed to get Transaction ids!", "Filter::filter_or()");
                if id_result.is_fail() {
                    return id_result
                        .convert("Filter::filter_or()")
                        .fail("Failed to filter OR.", "Filter::filter_or()")
                }
                ids.push(id_result.wont_fail("This is past an is_fail() guard clause.", "Filter::filter_or()"));
            }

            // checks each filter
            else {
                let does_year_match = match self.year {
                    Some(year) => transaction.date.get_year() == year,
                    None => false,
                };
                
                let does_month_match = match self.month {
                    Some(month) => transaction.date.get_month() == month,
                    None => false,
                };
                
                let mut does_tag_match = false;
                for tag in &self.tags {
                    if transaction.has_tag(tag) {
                        does_tag_match = true;
                        break;
                    }
                };

                let mut does_search_term_match = false;
                for term in &self.search_terms {
                    if transaction.date.display().to_lowercase().contains(&term.to_lowercase()) {
                        does_search_term_match = true;
                        break;
                    }
                    if transaction.description.to_lowercase().contains(&term.to_lowercase()) {
                        does_search_term_match = true;
                        break;
                    }
                    for tag in &transaction.tags {
                        if tag.get_label().to_lowercase().contains(&term.to_lowercase()) {
                            does_search_term_match = true;
                            break;
                        }
                    }
                }

                // collects the statuses to see if it matches
                let mut required_filters: Vec<bool> = Vec::new();
                if is_year_set { required_filters.push(does_year_match) }
                if is_month_set { required_filters.push(does_month_match) }
                if is_tag_set { required_filters.push(does_tag_match) }
                if is_search_term_set { required_filters.push(does_search_term_match) }
                let mut matches = false;
                for status in required_filters { if status { matches = true; } }

                // adds if it matches
                if matches {
                    let id_result = Schrod::from_option(transaction.get_id(), "Failed to get Transaction ids!", "Filter::filter_or()");
                    if id_result.is_fail() {
                        return id_result
                            .convert("Filter::filter_or()")
                            .fail("Failed to filter OR.", "Filter::filter_or()")
                    }
                    ids.push(id_result.wont_fail("This is past an is_fail() guard clause.", "Filter::filter_or()"));
                }
            }
        }

        // finished successfully
        self.filtered_ids = ids;
        Pass(())
    }
    
    fn filter_and(&mut self, transactions: &[Transaction]) -> Schrod<()> {
        // the filtered list being built
        let mut ids: Vec<Id> = Vec::new();
        // the base stats
        let is_year_set = self.year.is_some();
        let is_month_set = self.month.is_some();
        let is_tag_set = !self.tags.is_empty();
        let is_search_term_set = !self.search_terms.is_empty();
        let are_none_set = !is_year_set && !is_month_set && !is_tag_set && !is_search_term_set;

        // checking each transaction
        for transaction in transactions {
            // filters nothing if no filters are set
            if are_none_set {
                let id_result = Schrod::from_option(transaction.get_id(), "Failed to get Transaction ids!", "Filter::filter_and()");
                if id_result.is_fail() {
                    return id_result
                        .convert("Filter::filter_and()")
                        .fail("Failed to filter OR.", "Filter::filter_and()")
                }
                ids.push(id_result.wont_fail("This is past an is_fail() guard clause.", "Filter::filter_and()"));
            }

            // checks each filter
            else {
                let does_year_match = match self.year {
                    Some(year) => transaction.date.get_year() == year,
                    None => false,
                };
                
                let does_month_match = match self.month {
                    Some(month) => transaction.date.get_month() == month,
                    None => false,
                };
                
                let mut does_tag_match = true;
                for tag in &self.tags {
                    if !transaction.has_tag(tag) {
                        does_tag_match = false;
                        break;
                    }
                };

                let mut does_search_term_match = true;
                for term in &self.search_terms {
                    let mut found = false;
                    if transaction.date.display().to_lowercase().contains(&term.to_lowercase()) {
                        found = true;
                    }
                    if transaction.description.to_lowercase().contains(&term.to_lowercase()) {
                        found = true;
                    }
                    for tag in &transaction.tags {
                        if tag.get_label().to_lowercase().contains(&term.to_lowercase()) {
                            found = true;
                        }
                    }
                    if !found { does_search_term_match = false; }
                }

                // collects the statuses to see if it matches
                let mut required_filters: Vec<bool> = Vec::new();
                if is_year_set { required_filters.push(does_year_match) }
                if is_month_set { required_filters.push(does_month_match) }
                if is_tag_set { required_filters.push(does_tag_match) }
                if is_search_term_set { required_filters.push(does_search_term_match) }
                let mut matches = true;
                for status in required_filters { if !status { matches = false; } }

                // adds if it matches
                if matches {
                    let id_result = Schrod::from_option(transaction.get_id(), "Failed to get Transaction ids!", "Filter::filter_and()");
                    if id_result.is_fail() {
                        return id_result
                            .convert("Filter::filter_and()")
                            .fail("Failed to filter OR.", "Filter::filter_and()")
                    }
                    ids.push(id_result.wont_fail("This is past an is_fail() guard clause.", "Filter::filter_and()"));
                }
            }
        }

        // finished successfully
        self.filtered_ids = ids;
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