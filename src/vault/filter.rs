use crate::vault::transaction::*;

/// Generates a filtered collection of transactions based on a set of filters.
pub struct Filter<'bank> {
    /// The source collection of transactions.
    source: Option<&'bank Vec<Transaction>>,
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
    collection: Vec<&'bank Transaction>,
}
impl<'bank> Filter<'bank> {
    // initializing
    /// Creates a new empty teller.
    pub fn new() -> Filter<'bank> {
        Filter {
            source: None,
            mode: TellerModes::And,
            year: None,
            month: None,
            tags: Vec::new(),
            search_terms: Vec::new(),
            collection: Vec::new()
        }
    }
    
    /// Sets the source collection.
    pub fn set_source(&mut self, source: &'bank Vec<Transaction>) {
        self.source = Some(source);
        self.filter();
    }
    
    
    
    // management
    /// Toggles the mode.
    pub fn toggle_mode(&mut self) {
        if let TellerModes::Or = self.mode { self.mode = TellerModes::And; }
        else { self.mode = TellerModes::Or; }
        self.filter();
    }
    
    /// Sets the year.
    pub fn set_year(&mut self, year: u32) {
        self.year = Some(year);
        self.filter();
    }
    
    /// Clears the year.
    pub fn clear_year(&mut self) {
        self.year = None;
        self.filter();
    }
    
    /// Sets the month.
    pub fn set_month(&mut self, month: Months) {
        self.month = Some(month);
        self.filter();
    }
    
    /// Clears the month.
    pub fn clear_month(&mut self) {
        self.month = None;
        self.filter();
    }
    
    /// Adds a given tag.
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
        self.tags = Tag::sorted(self.tags.clone());
        self.filter();
    }
    
    /// Removes a given tag.
    pub fn remove_tag(&mut self, tag: &Tag) {
        self.tags.retain(|t| t != tag);
        self.filter();
    }
    
    /// Clears all tags.
    pub fn clear_tags(&mut self) {
        self.tags.clear();
        self.filter();
    }
    
    /// Adds a given search term.
    pub fn add_search_term(&mut self, search_term: Tag) {
        self.search_terms.push(search_term);
        self.search_terms = Tag::sorted(self.search_terms.clone());
        self.filter();
    }
    
    /// Removes a given search term.
    pub fn remove_search_term(&mut self, search_term: &Tag) {
        self.search_terms.retain(|t| t != search_term);
        self.filter();
    }
    
    /// Clears all search terms.
    pub fn clear_search_terms(&mut self) {
        self.search_terms.clear();
        self.filter();
    }
    
    /// Filters the source list based on the current filters.
    fn filter(&mut self) {
        // returns early if there is no source collection
        if self.source.is_none() { return; }

        // clears the collection before adding new transactions
        self.collection.clear();

        // variables for checking if a transaction matches the filters
        let mut does_year_match = true;
        let mut does_month_match = true;
        let mut does_tag_match = false;
        let mut does_search_term_match = false;

        // checks each source transaction
        for i in 0..self.source.unwrap().len() {
            let transaction = &self.source.unwrap()[i];
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
                        self.collection.push(transaction);
                    }
                }
                TellerModes::And => {
                    if does_year_match && does_month_match && does_tag_match && does_search_term_match {
                        self.collection.push(transaction);
                    }
                }
            }
        }
    }
    
    
    
    // data retrieval and parsing
    /// Returns the mode.
    pub fn get_mode(&self) -> &TellerModes { &self.mode }
    
    /// Returns the current collection.
    pub fn get_collection(&self) -> &Vec<&'bank Transaction> { &self.collection }
}



/// Determines whether the teller must match all filters (AND) or any filter (OR).
pub enum TellerModes {
    Or,
    And,
}