use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{iso, iso::Currency, Money};

/// Value type helps to clarify how Money is used in a Transaction context.
pub type Value = Money<'static, Currency>;

/// Used in each transaction instead of a usize for clarity.
pub type Id = usize;



/// The different ways to display values.
pub enum ValueDisplayFormats {
    /// Displays the value as dollars and cents.
    Dollars,
    /// Displays the value as a time price.
    Time(f64), // dollars per hour
}



/// A list of styles for formatting tag labels.
pub enum TagStyles {
    Uppercase,
    Lowercase,
    Capitalized,
}



/// Stores all the information about a financial transaction.
/// Tags are relied upon heavily to create a fine-tuned web of information.
pub struct Transaction {
    /// The internal id.
    id: Option<Id>,
    /// The positive or negative dollar value.
    pub value: Value,
    /// The date.
    pub date: Date,
    /// A brief description.
    pub description: String,
    /// A list of tags or categories.
    /// Potential combinations:
    /// ( eating out, wants )
    /// ( home goods )
    /// ( gas, transportation )
    pub tags: Vec<Tag>,
}
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_none() || other.id.is_none() { return false }
        self.id.expect("Transaction equality comparison failed!") == other.id.expect("Transaction equality comparison failed!")
    }
}
impl Transaction {
    // initializing
    /// Creates a new transaction from concrete values.
    /// This is intended to be used when a new transaction is created from within the app.
    pub fn new_from_parts(id: Id, value: Value, date: Date, description: String, tags: Vec<Tag>) -> Transaction {
        if !Transaction::are_tags_valid(&tags) { panic!("Invalid tags!") }
        if !Transaction::is_description_valid(&description) { panic!("Invalid description!") }
        Transaction { id: Some(id), value, date, description, tags }
    }

    /// Creates a new transaction from raw data parts.
    /// This is intended to be used when a new transaction is created from within the app.
    pub fn new_from_raw(id: Id, value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> Transaction {
        if !Transaction::is_value_string_valid(&value_string) { panic!("Invalid value!") }
        if !Transaction::are_tags_valid(&tags) { panic!("Invalid tags!") }
        if !Transaction::is_description_valid(&description) { panic!("Invalid description!") }

        let decimal_value = Decimal::from_str(&value_string).expect("Invalid value!");
        let currency = iso::find(currency_string.as_str()).expect("Invalid currency!");
        let value = Value::from_decimal(decimal_value, currency);

        Transaction { id: Some(id), value, date, description, tags }
    }

    /// Loads a new transaction from raw data parts.
    /// This is intended to be used when an existing transaction is loaded from save data.
    /// Please note that if this function is used, an id must be filled in later with set_id().
    pub fn load_from_raw(value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> Transaction {
        if !Transaction::is_value_string_valid(&value_string) { panic!("Invalid value!") }
        if !Transaction::are_tags_valid(&tags) { panic!("Invalid tags!") }
        if !Transaction::is_description_valid(&description) { panic!("Invalid description!") }

        let decimal_value = Decimal::from_str(&value_string).expect("Invalid value!");
        let currency = iso::find(currency_string.as_str()).expect("Invalid currency!");
        let value = Value::from_decimal(decimal_value, currency);

        Transaction { id: None, value, date, description, tags }
    }
    
    /// Sets the id of a transaction that does not have an id.
    /// Used primarily for transactions that are loaded from save data.
    pub fn set_id(&mut self, id: Id) {
        if self.id.is_some() { panic!("Transaction already has an id!") }
        self.id = Some(id);
    }
    
    /// Overrides the id of a transaction.
    /// Used primarily for re-indexing transactions.
    pub fn override_id(&mut self, id: Id) {
        self.id = Some(id);
    }



    // validating
    /// Returns whether a string can be parsed into a value.
    pub fn is_value_string_valid(value_string: &String) -> bool {
        Decimal::from_str(value_string).is_ok()
    }

    /// Returns whether a string can be parsed into a currency.
    pub fn is_currency_string_valid(currency_string: &String) -> bool {
        iso::find(currency_string.as_str()).is_some()
    }

    /// Determines if the given description is valid.
    pub fn is_description_valid(description: &String) -> bool {
        Tag::is_allowed(description)
    }

    /// Determines if the given list of tags is valid.
    /// Every tag in the list is already guaranteed to be valid.
    pub fn are_tags_valid(tags: &Vec<Tag>) -> bool {
        !tags.is_empty()
    }



    // management
    /// Adds a new tag and sorts the tag list.
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
        self.tags = Tag::sorted(self.tags.clone());
    }

    /// Removes a given tag from the list.
    pub fn remove_tag(&mut self, tag: &Tag) {
        if self.tags.len() <= 1 { return; }
        self.tags.retain(|t| t != tag);
    }



    // data retrieval and parsing
    /// Returns the internal id.
    pub fn get_id(&self) -> Option<Id> {
        self.id
    }
    
    /// Returns a mutable reference to the transaction with the given id.
    pub fn get_from(transactions: &mut Vec<Transaction>, id: Id) -> &mut Transaction {
        transactions.iter_mut().find(|trans|{
            if let Some(trans_id) = trans.id { return trans_id == id }
            return false
        }).expect("Failed to find transaction!")
    }

    /// Returns if the transaction contains the given tag.
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }
    
    /// Returns a formated string of the time equivalent of the value
    pub fn get_time_price(value: &Value, price: f64) -> String {
        format!("{:.2} hrs", value.amount().to_f64().expect("Failed to get transaction value!") / price)
    }
}



/// A custom date object tailored for tracking and parsing financial transactions.
#[derive(Debug, Clone)]
pub struct Date {
    year: u32,
    month: Months,
    day: u32,
}
impl Default for Date {
    /// Returns the default date: January 1, 1970.
    fn default() -> Self { Date::new(1970, Months::January, 1) }
}
impl Date {
    // initializing
    /// Creates a new date object.
    pub fn new(year: u32, month: Months, day: u32) -> Date {
        if !Date::is_valid(year, &month, day) { panic!("Invalid date!") }
        Date { year, month, day }
    }



    // validating
    /// Determines if a date can exist with the given data.
    fn is_valid(year: u32, month: &Months, day: u32) -> bool {
        let is_year_valid = year >= 1000 && year <= 9999; // ensures that as_value() is in the correct format
        let is_day_valid = day <= month.days_in_month(year);
        is_year_valid && is_day_valid // month is always valid as it is an enum
    }



    // management
    /// Updates the date with new values.
    pub fn edit(&mut self, year: u32, month: Months, day: u32) {
        if !Date::is_valid(year, &month, day) { panic!("Invalid date!") }
        self.year = year;
        self.month = month;
        self.day = day;
    }

    /// Advances the date by one year.
    pub fn advance_by_year(&mut self) {
        self.year += 1;
    }

    /// Recedes the date by one year.
    pub fn recede_by_year(&mut self) {
        self.year -= 1;
    }

    /// Advances the date by one month.
    pub fn advance_by_month(&mut self) {
        self.month = self.month.get_next();
        if self.month == Months::January { self.advance_by_year(); }
    }

    /// Recedes the date by one month.
    pub fn recede_by_month(&mut self) {
        self.month = self.month.get_previous();
        if self.month == Months::December { self.recede_by_year(); }
    }

    /// Advances the date by one day.
    pub fn advance_by_day(&mut self) {
        if self.day < self.month.days_in_month(self.year) {
            self.day += 1;
        } else {
            self.day = 1;
            self.advance_by_month();
        }
    }

    /// Recedes the date by one day.
    pub fn recede_by_day(&mut self) {
        if self.day > 1 {
            self.day -= 1;
        } else {
            self.day = self.month.days_in_month(self.year);
            self.recede_by_month();
        }
    }



    // data retrieval and parsing
    /// Returns a formatted string representation of the date.
    pub fn display(&self) -> String {
        format!("{} {} {}", self.month.display(), self.day, self.year)
    }

    /// Returns the year.
    pub fn get_year(&self) -> u32 {
        self.year
    }

    /// Returns a reference to the month.
    pub fn get_month(&self) -> &Months {
        &self.month
    }

    /// Returns the day.
    pub fn get_day(&self) -> u32 {
        self.day
    }

    /// Returns the date as a u32 value.
    /// Example: 20260206 - February 5, 2026
    pub fn as_value(&self) -> u32 {
        let year = self.year * 10000;
        let month = self.month.as_value() * 100;
        let day = self.day;
        year + month + day
    }

    /// Determines whether the given year is a leap year.
    fn is_leap_year(year: u32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
}



/// A custom enum for the month component of the date struct.
#[derive(Debug, Clone)]
pub enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl PartialEq for Months {
    /// Determines that two months are equal based on their numeric equivalents.
    fn eq(&self, other: &Self) -> bool {
        self.as_value() == other.as_value()
    }
}
impl Months {
    // data retrieval and parsing
    /// Returns a formatted string representation of the month.
    pub fn display(&self) -> String {
        match self {
            Months::January => { "January".to_string() }
            Months::February => { "February".to_string() }
            Months::March => { "March".to_string() }
            Months::April => { "April".to_string() }
            Months::May => { "May".to_string() }
            Months::June => { "June".to_string() }
            Months::July => { "July".to_string() }
            Months::August => { "August".to_string() }
            Months::September => { "September".to_string() }
            Months::October => { "October".to_string() }
            Months::November => { "November".to_string() }
            Months::December => { "December".to_string() }
        }
    }

    /// Returns the numeric equivalent of the month.
    pub fn as_value(&self) -> u32 {
        match self {
            Months::January => { 1 }
            Months::February => { 2 }
            Months::March => { 3 }
            Months::April => { 4 }
            Months::May => { 5 }
            Months::June => { 6 }
            Months::July => { 7 }
            Months::August => { 8 }
            Months::September => { 9 }
            Months::October => { 10 }
            Months::November => { 11 }
            Months::December => { 12 }
        }
    }

    /// Returns the enum equivalent of a month numeric value.
    pub fn get_enum(month: u32) -> Months {
        match month {
            1 => { Months::January }
            2 => { Months::February }
            3 => { Months::March }
            4 => { Months::April }
            5 => { Months::May }
            6 => { Months::June }
            7 => { Months::July }
            8 => { Months::August }
            9 => { Months::September }
            10 => { Months::October }
            11 => { Months::November }
            12 => { Months::December }
            _ => { panic!("Invalid month value!") }
        }
    }

    /// Returns the number of days in the month for the given year (for leap year conditions).
    fn days_in_month(&self, year: u32) -> u32 {
        match self {
            Months::January | Months::March | Months::May | Months::July | Months::August | Months::October | Months::December => { 31 }
            Months::April | Months::June | Months::September | Months::November => { 30 }
            Months::February => { if Date::is_leap_year(year) { 29 } else { 28 } }
        }
    }

    /// Returns the next month.
    fn get_next(&self) -> Months {
        if self.as_value() >= 12 { return Months::January }
        Months::get_enum(self.as_value() + 1)
    }

    /// Returns the previous month.
    fn get_previous(&self) -> Months {
        if self.as_value() <= 1 { return Months::December }
        Months::get_enum(self.as_value() - 1)
    }
}



/// A custom tag object tailored for parsing and sorting transactions with overlapping categories.
#[derive(Debug, Clone)]
pub struct Tag {
    /// The label of the tag.
    label: String,
}
impl PartialEq for Tag {
    /// Determines that two tags are equal based on their labels.
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}
impl Tag {
    // initializing
    /// Creates a new tag object.
    pub fn new(label: String) -> Tag {
        Tag { label: Self::validated_label(label) }
    }



    // validating
    pub fn is_allowed(input: &String) -> bool {
        let trimmed_input = input.trim();
        if trimmed_input.is_empty() { return false }
        let allowed_characters = vec![
            ' ', '\'', '.', '!', '?', ':', ';', '"', '-', '_', '(', ')', '[', ']', '{', '}',
            '*', '@', '#', '$', '%', '&', '~', '+', '=', '/', '<', '\\', '<', '>', '^', '|',
        ];
        for char in trimmed_input.chars() {
            if !char.is_alphanumeric() && !allowed_characters.contains(&char) { return false }
        }

        true
    }



    // management
    /// Edits the tag label.
    pub fn edit(&mut self, new_label: String) {
        self.label = Self::validated_label(new_label);
    }



    // data retrieval and parsing
    /// Returns a formatted label based on a given style.
    pub fn display(&self, style: TagStyles) -> String {
        match style {
            TagStyles::Uppercase => { self.label.to_uppercase() }
            TagStyles::Lowercase => { self.label.clone() } // already lowercase
            TagStyles::Capitalized => {
                let words: Vec<&str> = self.label.split(' ').collect();
                let mut capitalized_words: Vec<String> = Vec::new();

                for word in words {
                    let mut characters: Vec<char> = word.chars().collect();
                    characters[0] = characters[0].to_uppercase().nth(0).unwrap();
                    capitalized_words.push(characters.into_iter().collect());
                }

                capitalized_words.join(" ")
            }
        }
    }

    /// Returns a reference to the tag label.
    pub fn get_label(&self) -> &String {
        &self.label
    }

    /// Returns a validated tag label to ensure it only contains allowed characters.
    fn validated_label(new_label: String) -> String {
        let new_label = new_label.trim().to_lowercase();
        if !Self::is_allowed(&new_label) { panic!("Invalid tag!") }
        new_label
    }

    /// Determines if the tag contains another tag.
    pub fn contains(&self, other_tag: &Tag) -> bool {
        self.label.contains(&other_tag.label)
    }

    /// Returns a new list of tags that doesn't have duplicates.
    pub fn without_duplicates(list: Vec<Tag>) -> Vec<Tag> {
        let mut unique_tags: Vec<Tag> = Vec::new();

        for tag in list {
            if !unique_tags.contains(&tag) { unique_tags.push(tag); }
        }

        unique_tags
    }

    /// Returns a new list of tags sorted alphabetically.
    pub fn sorted(list: Vec<Tag>) -> Vec<Tag> {
        let mut sorted_tags: Vec<Tag> = list.clone();
        sorted_tags.sort_by(|a, b| a.label.cmp(&b.label));
        sorted_tags = Self::without_duplicates(sorted_tags);
        sorted_tags
    }
}