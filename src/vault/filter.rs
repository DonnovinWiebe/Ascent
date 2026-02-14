use crate::vault::transaction::*;

/// Determines whether the teller must match all filters (AND) or any filter (OR).
pub enum TellerModes {
    Or,
    And,
}



/// Generates a filtered collection of transactions based on a set of filters.
pub struct Filter {
    /// Whether each transaction must match all filters (AND) or any filter (OR).
    mode: TellerModes,
    /// The year to filter by.
    year: Option<u32>,
    /// The month to filter by.
    month: Option<Months>,
    /// The tags to filter by.
    tags: Vec<Tag>,
    /// The search terms to filter by.
    search_terms: Vec<Tag>,
    /// The filtered collection of transactions.
    filtered_ids: Vec<Id>,
}
impl Filter {
    // initializing
    /// Creates a new empty teller.
    pub fn new() -> Filter {
        Filter {
            mode: TellerModes::And,
            year: None,
            month: None,
            tags: Vec::new(),
            search_terms: Vec::new(),
            filtered_ids: Vec::new()
        }
    }
    
    
    
    // management
    /// Toggles the mode.
    pub fn toggle_mode(&mut self, transactions: &Vec<Transaction>) {
        if let TellerModes::Or = self.mode { self.mode = TellerModes::And; }
        else { self.mode = TellerModes::Or; }
        self.filter(transactions);
    }
    
    /// Sets the year.
    pub fn set_year(&mut self, year: u32, transactions: &Vec<Transaction>) {
        self.year = Some(year);
        self.filter(transactions);
    }
    
    /// Clears the year.
    pub fn clear_year(&mut self, transactions: &Vec<Transaction>) {
        self.year = None;
        self.filter(transactions);
    }
    
    /// Sets the month.
    pub fn set_month(&mut self, month: Months, transactions: &Vec<Transaction>) {
        self.month = Some(month);
        self.filter(transactions);
    }
    
    /// Clears the month.
    pub fn clear_month(&mut self, transactions: &Vec<Transaction>) {
        self.month = None;
        self.filter(transactions);
    }
    
    /// Adds a given tag.
    pub fn add_tag(&mut self, tag: Tag, transactions: &Vec<Transaction>) {
        self.tags.push(tag);
        self.tags = Tag::sorted(self.tags.clone());
        self.filter(transactions);
    }
    
    /// Removes a given tag.
    pub fn remove_tag(&mut self, tag: &Tag, transactions: &Vec<Transaction>) {
        self.tags.retain(|t| t != tag);
        self.filter(transactions);
    }
    
    /// Clears all tags.
    pub fn clear_tags(&mut self, transactions: &Vec<Transaction>) {
        self.tags.clear();
        self.filter(transactions);
    }
    
    /// Adds a given search term.
    pub fn add_search_term(&mut self, search_term: Tag, transactions: &Vec<Transaction>) {
        self.search_terms.push(search_term);
        self.search_terms = Tag::sorted(self.search_terms.clone());
        self.filter(transactions);
    }
    
    /// Removes a given search term.
    pub fn remove_search_term(&mut self, search_term: &Tag, transactions: &Vec<Transaction>) {
        self.search_terms.retain(|t| t != search_term);
        self.filter(transactions);
    }
    
    /// Clears all search terms.
    pub fn clear_search_terms(&mut self, transactions: &Vec<Transaction>) {
        self.search_terms.clear();
        self.filter(transactions);
    }
    
    /// Filters the source list based on the current filters.
    pub fn filter(&mut self, transactions: &Vec<Transaction>) {
        // clears the collection before adding new transactions
        self.filtered_ids.clear();

        // variables for checking if a transaction matches the filters
        let mut does_year_match = true;
        let mut does_month_match = true;
        let mut does_tag_match = false;
        let mut does_search_term_match = false;

        // checks each source transaction
        for i in 0..transactions.len() {
            let transaction = &transactions[i];
            // checks the year
            if let Some(year) = self.year {
                does_year_match = transaction.date.get_year() == year;
            }

            // checks the month
            if let Some(month) = &self.month {
                does_month_match = transaction.date.get_month() == month;
            }

            // checks the tags
            if self.tags.is_empty() { does_tag_match = true; }
            else {
                does_tag_match = false;
                for tag in &self.tags {
                    if transaction.has_tag(tag) { does_tag_match = true; }
                }
            }

            // checks the search terms
            if self.search_terms.is_empty() { does_search_term_match = true; }
            else {
                does_search_term_match = false;
                for search_term in &self.search_terms {
                    if transaction.description.contains(search_term) { does_search_term_match = true; }
                }
            }

            // adds the transaction to the collection if it matches based on the mode
            match self.mode {
                TellerModes::Or => {
                    if does_year_match || does_month_match || does_tag_match || does_search_term_match {
                        self.filtered_ids.push(transaction.get_id());
                    }
                }
                TellerModes::And => {
                    if does_year_match && does_month_match && does_tag_match && does_search_term_match {
                        self.filtered_ids.push(transaction.get_id());
                    }
                }
            }
        }
    }
    
    
    
    // data retrieval and parsing
    /// Returns the mode.
    pub fn get_mode(&self) -> &TellerModes { &self.mode }

    /// Gets the list of filtered transaction ids.
    pub fn get_filtered_ids(&self) -> Vec<Id> { self.filtered_ids.clone() }
}