use crate::ui::material::MaterialColors;
use crate::vault::filter::Filter;
use crate::vault::transaction::{Date, Id, Months, Tag, Transaction, Value};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};

/// The available filters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filters {
    Primary,
    DeepDive1,
    DeepDive2,
}



/// Holds a list of all the transactions.
pub struct Bank {
    /// The central list of all transactions.
    ledger: Vec<Transaction>,
    /// The tag registry.
    pub tag_registry: TagRegistry,
    /// The central id tracker for new transactions.
    id_tracker: Id,
    /// The primary filter.
    pub primary_filter: Filter,
    /// The first deep dive filter.
    pub deep_dive_1_filter: Filter,
    /// The second deep dive filter.
    pub deep_dive_2_filter: Filter,
}
impl Default for Bank {
    /// Creates a new default Bank object.
    fn default() -> Bank {
        Bank::new()
    }
}
impl Bank {
    // initializing
    /// Creates a new Bank object.
    fn new() -> Bank {
        Bank { ledger: Vec::new(), tag_registry: TagRegistry::new(), id_tracker: 0, primary_filter: Filter::default(), deep_dive_1_filter: Filter::default(), deep_dive_2_filter: Filter::default() }
    }

    /// Initializes the bank.
    pub fn init(&mut self, transactions: Vec<Transaction>, tag_registry: TagRegistry) {
        self.load_transactions(transactions);
        self.tag_registry = tag_registry;
    }
    
    /// Loads transactions into the bank.
    /// This is used when loading from save data.
    fn load_transactions(&mut self, transactions: Vec<Transaction>) -> ResultStack<()> {
        let mut new_ledger = Vec::new();
        for mut transaction in transactions {
            let set_result = transaction.set_id(self.get_next_id()); // uses set_id() instead of override_id() to ensure proper data flow
            if set_result.is_fail() { return set_result.fail("Could not load transactions into ledger!"); }
            new_ledger.push(transaction);
        }
        self.ledger = new_ledger;
        let filter_result = self.refilter();
        if filter_result.is_fail() { return filter_result.fail("Could not filter the new loaded ledger."); }
        Pass(())
    }



    // management
    /// Gets a copy of the ledger.
    /// Please note that modifying these transactions has no effect on the bank's real ledger.
    pub fn get_ledger_copy(&self) -> Vec<Transaction> {
        self.ledger.clone()
    }
    
    /// Gets the next available id.
    pub fn get_next_id(&mut self) -> Id {
        let id_to_return = self.id_tracker;
        self.id_tracker += 1;
        id_to_return
    }
    
    /// Re-indexes all transactions in the ledger to help make transaction id's more closely align with their index in the ledger.
    pub fn reindex_transactions(&mut self) {
        self.id_tracker = 0;
        for i in 0..self.ledger.len() {
            let id = self.get_next_id();
            self.ledger[i].override_id(id);
        }
    }
    
    /// Sorts a ledger by date.
    pub fn sorted_ledger(ledger: Vec<Transaction>) -> Vec<Transaction> {
        let mut ledger = ledger;
        ledger.sort_by(|a, b| a.date.as_value().cmp(&b.date.as_value()));
        ledger
    }

    /// Sorts the ledger by date.
    fn sort_ledger(&mut self) {
        // I could duplicate sorted_ledger() here, but this is faster
        self.ledger.sort_by(|a, b| a.date.as_value().cmp(&b.date.as_value()));
    }

    /// Adds a new transaction from concrete values.
    /// This is intended to be used when a new transaction is created from within the app.
    pub fn add_transaction_from_parts(&mut self, value: Value, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<()> {
        let id = self.get_next_id();
        let transaction_result = Transaction::new_from_parts(id, value, date, description, tags);
        if let Pass(transaction) = transaction_result {
            self.ledger.push(transaction);
            let filter_result = self.refilter();
            if filter_result.is_fail() { return filter_result; }
            Pass(())
        }
        else {
            transaction_result.fail("Failed to add transaction from parts.").empty_type()
        }
    }

    /// Creates a new transaction from raw data parts.
    /// This is intended to be used when a new transaction is created from within the app.
    pub fn add_transaction_from_raw_parts(&mut self, value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<()> {
        let id = self.get_next_id();
        let transaction_result = Transaction::new_from_raw(id, value_string, currency_string, date, description, tags);
        
        if let Pass(transaction) = transaction_result {
            self.ledger.push(transaction);
            let filter_result = self.refilter();
            if filter_result.is_fail() { return filter_result; }
            Pass(())
        }
        else {
            transaction_result.fail("Failed to add a new transaction from raw parts.").empty_type()
        }
    }

    /// Edits a transaction with raw parts.
    pub fn edit_transaction_with_raw_parts(&mut self, id: Id, value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<()> {
        let transaction_result = self.get_mut(id);
        if let Pass(transaction) = transaction_result {
            let edit_result = transaction.edit_with_raw_parts(value_string, currency_string, date, description, tags);
            match edit_result {
                Pass(_) => { self.refilter() }
                Fail(_) => { edit_result }
            }
        }
        else {
            transaction_result.fail("Failed to edit a transaction with raw parts.").empty_type()
        }
    }

    /// Removes a transaction from the ledger.
    pub fn remove_transaction(&mut self, id: Id) -> ResultStack<()> {
        for i in 0..self.ledger.len() {
            let transaction = &mut self.ledger[i];
            if let Some(transaction_id) = transaction.get_id() && transaction_id == id {
                self.ledger.remove(i);
                let filter_result = self.refilter();
                if filter_result.is_fail() { return filter_result; }
                return Pass(());
            }
        }
        
        ResultStack::new_fail("Transaction could not be found!")
    }
    
    /// Returns an updated tag registry to match the current tags in the ledger.
    pub fn get_updated_tag_registry(tag_registry: TagRegistry, tags: Vec<Tag>) -> TagRegistry {
        let mut updated_tag_registry = tag_registry.clone();
        updated_tag_registry.update_registry(tags);
        updated_tag_registry
    }



    // data retrieval and parsing
    /// Returns a mutable reference to the ledger.
    pub fn ledger_mut(&mut self) -> &mut Vec<Transaction> {
        &mut self.ledger
    }

    /// Returns an immutable reference to the ledger.
    pub fn ledger(&self) -> &Vec<Transaction> {
        &self.ledger
    }

    /// Gets a list of the transaction ids filtered by the given filter.
    pub fn get_filtered_ids(&self, filter: Filters) -> Vec<Id> {
        self.get_filter(filter).get_filtered_ids()
    }

    /// Returns an immutable reference to a transaction.
    pub fn get(&self, id: Id) -> ResultStack<&Transaction> {
        for transaction in &self.ledger { // todo start searching at index = id for efficiency
            if let Some(transaction_id) = transaction.get_id() && transaction_id == id {
                return Pass(transaction);
            }
        }
        
        ResultStack::new_fail("Transaction could not be found!")
    }

    /// Returns a mutable reference to a transaction.
    pub fn get_mut(&mut self, id: Id) -> ResultStack<&mut Transaction> {
        for transaction in &mut self.ledger { // todo start searching at index = id for efficiency
            if let Some(transaction_id) = transaction.get_id() && transaction_id == id {
                return Pass(transaction);
            }
        }
        
        ResultStack::new_fail("Transaction could not be found!")
    }

    /// Returns a list of existing tags
    pub fn get_tags(&self) -> Vec<Tag> {
        let mut tags = Vec::new();
        for transaction in &self.ledger {
            tags.extend(transaction.tags.clone());
        }
        Tag::sorted(tags)
    }
    
    /// Gets the date of the latest transaction in the ledger.
    /// If the ledger is empty, this returns the default date.
    pub fn get_latest_date(&self) -> Date {
        self.ledger.last().map(|t| t.date.clone()).unwrap_or_default()
    }
    
    /// Gets the date of the latest transaction from a given filter.
    /// If the filter is empty, this returns the default date.
    pub fn get_latest_date_for_filter(&self, filter: Filters) -> Date {
        let filtered_ids = self.get_filter(filter).get_filtered_ids();
        
        let transactions = self.ledger.iter().filter(|ledger_transaction| {
            let ledger_transaction_id_result = &ledger_transaction.get_id();
            match ledger_transaction_id_result {
                Some(ledger_transaction_id) => filtered_ids.contains(ledger_transaction_id),
                None => false,
            }
        }).collect::<Vec<_>>();
        
        if !transactions.is_empty() { transactions[transactions.len() - 1].date.clone() }
        else { Date::default() }
    }
    
    /// Returns an immutable reference to a filter.
    pub fn get_filter(&self, filter: Filters) -> &Filter {
        match filter {
            Filters::Primary => &self.primary_filter,
            Filters::DeepDive1 => &self.deep_dive_1_filter,
            Filters::DeepDive2 => &self.deep_dive_2_filter,
        }
    }
    
    /// Returns a mutable reference to a filter.
    pub fn get_filter_mut(&mut self, filter: Filters) -> &mut Filter {
        match filter {
            Filters::Primary => &mut self.primary_filter,
            Filters::DeepDive1 => &mut self.deep_dive_1_filter,
            Filters::DeepDive2 => &mut self.deep_dive_2_filter,
        }
    }
    
    /// Toggles the mode of the given filter.
    pub fn toggle_filter_mode(&mut self, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.toggle_mode(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.toggle_mode(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.toggle_mode(&self.ledger),
        }
    }
    
    /// Sets the year of the given filter.
    pub fn set_filter_year(&mut self, year: u32, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.set_year(year, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.set_year(year, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.set_year(year, &self.ledger),
        }
    }
    
    /// Clears the year of the given filter.
    pub fn clear_filter_year(&mut self, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_year(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_year(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_year(&self.ledger),
        }
    }
    
    /// Sets the month of the given filter.
    pub fn set_filter_month(&mut self, month: Months, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.set_month(month, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.set_month(month, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.set_month(month, &self.ledger),
        }
    }
    
    /// Clears the month of the given filter.
    pub fn clear_filter_month(&mut self, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_month(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_month(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_month(&self.ledger),
        }
    }
    
    /// Adds a given tag of the given filter.
    pub fn add_filter_tag(&mut self, tag: Tag, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.add_tag(&tag, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.add_tag(&tag, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.add_tag(&tag, &self.ledger),
        }
    }
    
    /// Removes a given tag of the given filter.
    pub fn remove_filter_tag(&mut self, tag: Tag, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.remove_tag(&tag, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.remove_tag(&tag, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.remove_tag(&tag, &self.ledger),
        }
    }
    
    /// Clears all tags of the given filter.
    pub fn clear_filter_tags(&mut self, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_tags(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_tags(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_tags(&self.ledger),
        }
    }
    
    /// Checks if the given tag is filtered by the given filter.
    pub fn is_tag_filtered(&self, tag: Tag, filter: Filters) -> bool {
        match filter {
            Filters::Primary => self.primary_filter.is_tag_filtered(&tag),
            Filters::DeepDive1 => self.deep_dive_1_filter.is_tag_filtered(&tag),
            Filters::DeepDive2 => self.deep_dive_2_filter.is_tag_filtered(&tag),
        }
    }
    
    /// Adds a given search term of the given filter.
    pub fn add_filter_search_term(&mut self, term: String, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.add_search_term(&term, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.add_search_term(&term, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.add_search_term(&term, &self.ledger),
        }
    }
    
    /// Removes a given search term of the given filter.
    pub fn remove_filter_search_term(&mut self, term: String, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.remove_search_term(&term, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.remove_search_term(&term, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.remove_search_term(&term, &self.ledger),
        }
    }
    
    /// Clears all search terms of the given filter.
    pub fn clear_filter_search_terms(&mut self, filter: Filters) -> ResultStack<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_search_terms(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_search_terms(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_search_terms(&self.ledger),
        }
    }
    
    
    /// Refilters the transactions in the three bank's filters.
    fn refilter(&mut self) -> ResultStack<()> {
        self.sort_ledger();
        
        let primary_filter_result = self.primary_filter.filter(&self.ledger);
        if primary_filter_result.is_fail() { return primary_filter_result; }
        
        let deep_dive_1_filter_result = self.deep_dive_1_filter.filter(&self.ledger);
        if deep_dive_1_filter_result.is_fail() { return deep_dive_1_filter_result; }
        
        let deep_dive_2_filter_result = self.deep_dive_2_filter.filter(&self.ledger);
        if deep_dive_2_filter_result.is_fail() { return deep_dive_2_filter_result; }
        
        Pass(())
    }
}



/// Holds a list of tags with their bound colors.
/// This registry holds no duplicate tags.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TagRegistry {
    /// The list of tag registrations.
    registry: Vec<TagRegistration>,
}
impl Default for TagRegistry {
    /// Creates a new default TagRegistry object.
    fn default() -> TagRegistry {
        TagRegistry::new()
    }
}
impl TagRegistry {
    // initializing
    /// Creates a new tag registry.
    fn new() -> TagRegistry {
        TagRegistry { registry: Vec::new() }
    }



    // management
    /// Sets a registration.
    /// If the tag does not exist in the registry, a new registration is created.
    /// If the tag does exist in the registry, the existing registration is edited.
    pub fn set(&mut self, reference_tag: &Tag, color: MaterialColors) {
        if let Some(registration) = self.get_registration_mut(reference_tag) {
            registration.edit_color(color);
        }
        else {
            self.registry.push(TagRegistration::new(reference_tag.clone(), color));
        }
    }

    /// Edits an existing tag in the registry.
    pub fn change_tag(&mut self, reference_tag: &Tag, new_tag: &Tag) {
        if let Some(registration) = self.get_registration_mut(reference_tag) {
            registration.edit_tag(new_tag.clone());
        }
        else { panic!("Tag not found!") }
    }

    /// Removes a tag from the registry.
    pub fn remove(&mut self, reference_tag: &Tag) {
        self.registry.retain(|reg| &reg.tag != reference_tag);
    }
    
    /// Updates the registry to match the given tags, removing unnecessary registrations and adding unregistered tags.
    pub fn update_registry(&mut self, tags: Vec<Tag>) {
        // remove unnecessary registrations
        let mut unnecessary_registrations = Vec::new();
        for registration in &self.registry {
            if !tags.contains(&registration.tag) { unnecessary_registrations.push(registration.clone()); }
        }
        for registration in unnecessary_registrations {
            self.remove(&registration.tag);
        }
        
        // adds unregistered tags
        let mut unregistered_tags = Vec::new();
        for tag in tags {
            if self.get_registration(&tag).is_none() { unregistered_tags.push(tag); }
        }
        for tag in unregistered_tags {
            self.set(&tag, MaterialColors::Unavailable);
        }
    }



    // data retrieval and parsing
    /// Returns a mutable reference to a registration if it exists, else None.
    pub fn get_registration_mut(&mut self, reference_tag: &Tag) -> Option<&mut TagRegistration> {
        for registration in &mut self.registry {
            if registration.tag == *reference_tag { return Some(registration) }
        }
        None
    }

    /// Returns an immutable reference to a registration if it exists, else None.
    pub fn get_registration(&self, reference_tag: &Tag) -> Option<&TagRegistration> {
        for registration in &self.registry {
            if registration.tag == *reference_tag { return Some(registration) }
        }
        None
    }

    /// Returns the color of a tag.
    /// If the tag does not exist, a default color is returned.
    pub fn get(&self, reference_tag: &Tag) -> MaterialColors {
        if let Some(registration) = self.get_registration(reference_tag) {
            return registration.color()
        }
        MaterialColors::Unavailable
    }

    /// Returns a list of all the tags that have a given color.
    pub fn get_tags_for_color(&self, color: MaterialColors) -> Vec<Tag> {
        let mut tags = Vec::new();
        for registration in &self.registry {
            if registration.color == color { tags.push(registration.tag.clone()) }
        }
        tags
    }
}



/// Holds a registration of a unique tag with a color.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TagRegistration {
    /// The unique tag.
    tag: Tag,
    /// The color of the tag.
    color: MaterialColors,
}
impl TagRegistration {
    // initializing
    /// Creates a new tag registration.
    pub fn new(tag: Tag, color: MaterialColors) -> TagRegistration {
        TagRegistration { tag, color }
    }



    // management
    /// Edits the tag of the registration.
    pub fn edit_tag(&mut self, new_tag: Tag) {
        self.tag = new_tag
    }

    /// Edits the color of the registration.
    pub fn edit_color(&mut self, new_color: MaterialColors) {
        self.color = new_color
    }



    // data retrieval and parsing
    /// Returns the color.
    pub fn color(&self) -> MaterialColors {
        self.color
    }
}