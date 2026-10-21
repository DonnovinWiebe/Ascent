use uuid::Uuid;
use crate::standard::transaction::{Months, Tag, Transaction};

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
    filtered_ids: Vec<Uuid>,
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
    pub fn toggle_mode(&mut self, transactions: &[Transaction]) {
        if let FilterModes::Or = self.mode { self.mode = FilterModes::And; }
        else { self.mode = FilterModes::Or; }
        self.filter(transactions);
    }
    
    /// Sets the `year`.
    pub fn set_year(&mut self, year: u32, transactions: &[Transaction]) {
        self.year = Some(year);
        self.filter(transactions);
    }
    
    /// Clears the `year`.
    pub fn clear_year(&mut self, transactions: &[Transaction]) {
        self.year = None;
        self.filter(transactions);
    }
    
    /// Sets the `month`.
    pub fn set_month(&mut self, month: Months, transactions: &[Transaction]) {
        self.month = Some(month);
        self.filter(transactions);
    }
    
    /// Clears the `month`.
    pub fn clear_month(&mut self, transactions: &[Transaction]) {
        self.month = None;
        self.filter(transactions);
    }
    
    /// Adds a given `Tag`.
    pub fn add_tag(&mut self, tag: &Tag, transactions: &[Transaction]) {
        self.tags.push(tag.clone());
        self.tags = Tag::sorted(&self.tags);
        self.filter(transactions);
    }
    
    /// Removes a given `Tag`.
    pub fn remove_tag(&mut self, tag: &Tag, transactions: &[Transaction]) {
        self.tags.retain(|t| t != tag);
        self.filter(transactions);
    }
    
    /// Clears all `Tag`s.
    pub fn clear_tags(&mut self, transactions: &[Transaction]) {
        self.tags.clear();
        self.filter(transactions);
    }

    /// Makes sure that the filtered `Tag`s all exist in the given list of `Tag`s.
    pub fn verify_filtered_tags(&mut self, existing_tags: &[Tag]) {
        self.tags.retain(|tag| existing_tags.contains(tag));
    }
    
    /// Adds a given search term.
    pub fn add_search_term(&mut self, search_term: &str, transactions: &[Transaction]) {
        self.search_terms.push(search_term.to_lowercase());
        self.search_terms.sort();
        self.filter(transactions);
    }
    
    /// Removes a given search term.
    pub fn remove_search_term(&mut self, search_term: &str, transactions: &[Transaction]) {
        self.search_terms.retain(|t| t.clone() != search_term.to_lowercase());
        self.filter(transactions);
    }
    
    /// Clears all search terms.
    pub fn clear_search_terms(&mut self, transactions: &[Transaction]) {
        self.search_terms.clear();
        self.filter(transactions);
    }
    
    /// Filters the source list based on the current filters.
    #[must_use]
    #[allow(clippy::too_many_lines)] // this holds the main filtering logic for what transactions are displayed at any given time, and is going to be large
    pub fn filter(&mut self, transactions: &[Transaction]) {
        // clears the collection before adding new transactions
        self.filtered_ids.clear();

        // filters
        match self.mode {
            FilterModes::Or => self.filter_or(transactions),
            FilterModes::And => self.filter_and(transactions),
        };
    }

    fn filter_or(&mut self, transactions: &[Transaction]) {
        // the filtered list being built
        let mut ids: Vec<Uuid> = Vec::new();
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
                ids.push(transaction.get_id());
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
                    ids.push(transaction.get_id());
                }
            }
        }

        // finished successfully
        self.filtered_ids = ids;
    }
    
    fn filter_and(&mut self, transactions: &[Transaction]) {
        // the filtered list being built
        let mut ids: Vec<Uuid> = Vec::new();
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
                ids.push(transaction.get_id());
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
                    ids.push(transaction.get_id());
                }
            }
        }

        // finished successfully
        self.filtered_ids = ids;
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
    pub fn get_filtered_ids(&self) -> Vec<Uuid> { self.filtered_ids.clone() }
}