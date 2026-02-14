use iced::Color;
use rust_decimal_macros::dec;
use rusty_money::iso::{Currency, CAD, USD};
use crate::vault::filter::Filter;
use crate::vault::transaction::*;

/// Holds a list of all the transactions.
pub struct Bank {
    /// The central list of all transactions.
    ledger: Vec<Transaction>,
    /// The tag registry.
    tag_registry: TagRegistry,
    /// The central id tracker for new transactions.
    id_tracker: Id,
    /// The primary filter.
    pub primary_filter: Filter,
    /// The first deep dive filter.
    pub deep_dive_1_filter: Filter,
    /// The second deep dive filter.
    pub deep_dive_2_filter: Filter,
}
impl Bank {
    // initializing
    /// Creates a new bank object.
    pub fn new() -> Bank {
        Bank { ledger: Vec::new(), tag_registry: TagRegistry::new(), id_tracker: 0, primary_filter: Filter::new(), deep_dive_1_filter: Filter::new(), deep_dive_2_filter: Filter::new() }
    }

    /// Initializes the bank.
    pub fn init(&mut self) {
        self.add_transaction(
            Value::from_decimal(dec!(85.23), USD),
            Date::new(2026, Months::January, 1),
            Tag::new("the first test".to_string()),
            vec![Tag::new("test".to_string())]
        );
        self.add_transaction(
            Value::from_decimal(dec!(-32.17), USD),
            Date::new(2026, Months::January, 7),
            Tag::new("the second test".to_string()),
            vec![Tag::new("test".to_string())]
        );
        self.add_transaction(
            Value::from_decimal(dec!(-127.76), CAD),
            Date::new(2026, Months::January, 13),
            Tag::new("the third test".to_string()),
            vec![Tag::new("test".to_string())]
        );

        self.primary_filter.filter(&self.ledger);
        self.deep_dive_1_filter.filter(&self.ledger);
        self.deep_dive_2_filter.filter(&self.ledger);
    }
    
    /// Loads transactions into the bank.
    /// This is used when loading from save data.
    pub fn load_transactions(&mut self, transactions: Vec<Transaction>) {
        for mut transaction in transactions {
            transaction.set_id(self.get_next_id()); // uses set_id() instead of override_id() to ensure proper data flow
            self.ledger.push(transaction);
        }
    }



    // management
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
    pub fn sort_ledger(&mut self) {
        // I could duplicate sorted_ledger() here, but this is faster
        self.ledger.sort_by(|a, b| a.date.as_value().cmp(&b.date.as_value()));
    }

    /// Adds a new transaction to the ledger.
    pub fn add_transaction(&mut self, value: Value, date: Date, description: Tag, tags: Vec<Tag>) {
        let id = self.get_next_id();
        self.ledger.push(Transaction::new(Some(id), value, date, description, tags));
        self.sort_ledger();
    }

    /// Removes a transaction from the ledger.
    pub fn remove_transaction(&mut self, id: Id) {
        for i in 0..self.ledger.len() {
            let mut transaction = &mut self.ledger[i];
            if let Some(transaction_id) = transaction.get_id() {
                if transaction_id == id {
                    self.ledger.remove(i);
                    return
                } 
            }
        }
        panic!("Transaction not found!")
    }



    // data retrieval and parsing
    /// Returns a mutable reference to the ledger.
    pub fn ledger(&mut self) -> &Vec<Transaction> {
        &mut self.ledger
    }

    /// Returns an immutable reference to a transaction.
    pub fn get(&self, id: Id) -> &Transaction {
        for transaction in &self.ledger { // todo start searching at index = id for efficiency
            if let Some(transaction_id) = transaction.get_id() {
                if transaction_id == id { return transaction }
            }
        }
        panic!("Transaction not found!")
    }

    /// Returns a mutable reference to a transaction.
    pub fn get_mut(&mut self, id: Id) -> &mut Transaction {
        for transaction in &mut self.ledger { // todo start searching at index = id for efficiency
            if let Some(transaction_id) = transaction.get_id() {
                if transaction_id == id { return transaction }
            }
        }
        panic!("Transaction not found!")
    }

    /// Returns a list of existing tags
    pub fn get_tags(&self) -> Vec<Tag> {
        let mut tags = Vec::new();
        for transaction in &self.ledger {
            tags.extend(transaction.tags.clone());
        }
        Tag::sorted(tags)
    }
}



/// Holds a list of tags with their bound colors.
/// This registry holds no duplicate tags.
pub struct TagRegistry {
    /// The list of tag registrations.
    registry: Vec<TagRegistration>,
}
impl TagRegistry {
    // initializing
    /// Creates a new tag registry.
    pub fn new() -> TagRegistry {
        TagRegistry { registry: Vec::new() }
    }



    // management
    /// Sets a registration.
    /// If the tag does not exist in the registry, a new registration is created.
    /// If the tag does exist in the registry, the existing registration is edited.
    pub fn set(&mut self, reference_tag: &Tag, color: Color) {
        if let Some(registration) = self.get_registration(reference_tag) {
            registration.edit_color(color);
            return
        }
        else {
            self.registry.push(TagRegistration::new(reference_tag.clone(), color));
        }
    }

    /// Edits an existing tag in the registry.
    pub fn change_tag(&mut self, reference_tag: &Tag, new_tag: &Tag) {
        if let Some(registration) = self.get_registration(reference_tag) {
            registration.edit_tag(new_tag.clone());
        }
        else { panic!("Tag not found!") }
    }

    /// Removes a tag from the registry.
    pub fn remove(&mut self, reference_tag: &Tag) {
        for registration in &mut self.registry {
            if &registration.tag == reference_tag {
                self.registry.retain(|reg| &reg.tag != reference_tag);
                return
            }
        }
    }



    // data retrieval and parsing
    /// Returns a mutable reference to a registration if it exists, else None.
    pub fn get_registration(&mut self, reference_tag: &Tag) -> Option<&mut TagRegistration> {
        for registration in &mut self.registry {
            if registration.tag == *reference_tag { return Some(registration) }
        }
        None
    }

    /// Returns the color of a tag.
    pub fn get_color(&mut self, reference_tag: &Tag) -> Color {
        if let Some(registration) = self.get_registration(reference_tag) {
            registration.color()
        }
        else { panic!("Tag registration not found!") }
    }

    /// Returns a list of all the tags that have a given color.
    pub fn get_tags_for_color(&self, color: Color) -> Vec<Tag> {
        let mut tags = Vec::new();
        for registration in &self.registry {
            if registration.color == color { tags.push(registration.tag.clone()) }
        }
        tags
    }
}



/// Holds a registration of a unique tag with a color.
pub struct TagRegistration {
    /// The unique tag.
    tag: Tag,
    /// The color of the tag.
    color: Color
}
impl TagRegistration {
    // initializing
    /// Creates a new tag registration.
    pub fn new(tag: Tag, color: Color) -> TagRegistration {
        TagRegistration { tag, color }
    }



    // management
    /// Edits the tag of the registration.
    pub fn edit_tag(&mut self, new_tag: Tag) {
        self.tag = new_tag
    }

    /// Edits the color of the registration.
    pub fn edit_color(&mut self, new_color: Color) {
        self.color = new_color
    }



    // data retrieval and parsing
    /// Returns the color.
    pub fn color(&self) -> Color {
        self.color
    }
}