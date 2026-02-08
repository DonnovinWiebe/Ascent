use iced::Color;
use crate::vault::filter::Filter;
use crate::vault::transaction::*;

/// Holds a list of all the transactions.
pub struct Bank<'bank> {
    /// The central list of all transactions.
    ledger: Vec<Transaction>,
    /// The tag registry.
    tag_registry: TagRegistry,
    /// The central id tracker for new transactions.
    id_tracker: usize,
    /// The primary filter.
    primary_filter: Filter<'bank>,
    /// The first deep dive filter.
    deep_dive_1_filter: Filter<'bank>,
    /// The second deep dive filter.
    deep_dive_2_filter: Filter<'bank>,
}
impl<'bank> Bank<'bank> {
    // initializing
    /// Creates a new bank object.
    pub fn new() -> Bank<'bank> {
        Bank { ledger: Vec::new(), tag_registry: TagRegistry::new(), id_tracker: 0, primary_filter: Filter::new(), deep_dive_1_filter: Filter::new(), deep_dive_2_filter: Filter::new() }
    }

    /// Initializes the bank.
    pub fn init(&'bank mut self) {
        self.init_filter_sources();
    }

    /// Sets the source collection for each filter.
    fn init_filter_sources(&'bank mut self) {
        self.primary_filter.set_source(&self.ledger);
        self.deep_dive_1_filter.set_source(&self.ledger);
        self.deep_dive_2_filter.set_source(&self.ledger);
    }



    // management
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
        self.ledger.push(Transaction::new(self.id_tracker, value, date, description, tags));
        self.id_tracker += 1;
        self.sort_ledger();
    }

    /// Removes a transaction from the ledger.
    pub fn remove_transaction(&mut self, id: usize) {
        for (index, transaction) in self.ledger.iter().enumerate() {
            if transaction.get_id() == id { self.ledger.remove(index); return }
        }
        panic!("Transaction not found!")
    }



    // data retrieval and parsing
    /// Returns a mutable reference to the ledger.
    pub fn ledger(&mut self) -> &Vec<Transaction> {
        &mut self.ledger
    }

    /// Returns a mutable reference to a transaction.
    pub fn get(&mut self, id: usize) -> &mut Transaction {
        for transaction in &mut self.ledger {
            if transaction.get_id() == id { return transaction }
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