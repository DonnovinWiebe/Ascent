use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{iso, iso::Currency, Money};
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::{Pass, Fail};

/// Value type helps to clarify how Money is used in a Transaction context.
pub type Value = Money<'static, Currency>;

/// Used in each transaction instead of a usize for clarity.
pub type Id = usize;



/// The different ways to display values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueDisplayFormats {
    /// Displays the value as dollars and cents.
    Dollars,
    /// Displays the value as a time price.
    Time(f64), // dollars per hour
}



/// A list of styles for formatting tag labels.
#[derive(Debug, Clone, Copy, PartialEq)]
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
        // no error handling happens here as this should never ever fail
        self.id.expect("Transaction equality comparison failed catastrophically!") == other.id.expect("Transaction equality comparison failed catastrophically!")
    }
}
impl Transaction {
    // initializing
    /// Creates a new transaction from concrete values.
    /// This is intended to be used when a new transaction is created from within the app.
    pub fn new_from_parts(id: Id, value: Value, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<Transaction> {
        if !Transaction::are_parts_valid(&description, &tags) { ResultStack::new_fail("Failed to create a transaction from parts!") }
        else { Pass(Transaction { id: Some(id), value, date, description, tags }) }
    }

    /// Creates a new transaction from raw data parts.
    /// This is intended to be used when a new transaction is created from within the app.
    pub fn new_from_raw(id: Id, value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<Transaction> {
        if !Transaction::are_raw_parts_valid(&value_string, &currency_string, &description, &tags) { return ResultStack::new_fail("Failed to create a transaction from raw data!") }

        let decimal_value_result = ResultStack::from_result(Decimal::from_str(&value_string), "Failed to convert value_string to Decimal.");
        let currency_result = ResultStack::from_option(iso::find(&currency_string.to_uppercase()), "Failed to convert currency_string to Currency.");

        match (&decimal_value_result, &currency_result) {
            (Pass(value), Pass(currency)) => {
                Pass(Transaction { id: Some(id), value: Value::from_decimal(value.clone(), currency), date, description, tags })
            }
            _ => ResultStack::new_fail_from_unknown_failure(vec![decimal_value_result.get_possible_failures(), currency_result.get_possible_failures()])
        }
    }

    /// Loads a new transaction from raw data parts.
    /// This is intended to be used when an existing transaction is loaded from save data.
    /// Please note that if this function is used, an id must be filled in later with set_id().
    pub fn load_from_raw(value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<Transaction> {
        if !Transaction::are_raw_parts_valid(&value_string, &currency_string, &description, &tags) { return ResultStack::new_fail("Failed to load a transaction from raw data!") }

        let decimal_value_result = ResultStack::from_result(Decimal::from_str(&value_string), "Failed to convert value_string to Decimal.");
        let currency_result = ResultStack::from_option(iso::find(&currency_string.to_uppercase()), "Failed to convert currency_string to Currency.");

        match (&decimal_value_result, &currency_result) {
            (Pass(value), Pass(currency)) => {
                Pass(Transaction { id: None, value: Value::from_decimal(value.clone(), currency), date, description, tags })
            }
            _ => ResultStack::new_fail_from_unknown_failure(vec![decimal_value_result.get_possible_failures(), currency_result.get_possible_failures()])
        }
    }
    
    /// Sets the id of a transaction that does not have an id.
    /// Used primarily for transactions that are loaded from save data.
    pub fn set_id(&mut self, id: Id) -> ResultStack<()> {
        if self.id.is_some() { return ResultStack::new_fail("Failed to set transaction id because it is already set."); }
        self.id = Some(id);
        Pass(())
    }
    
    /// Overrides the id of a transaction.
    /// Used primarily for re-indexing transactions.
    pub fn override_id(&mut self, id: Id) {
        self.id = Some(id);
    }



    // validating
    /// Checks if a transaction can be created from the given parts.
    pub fn are_parts_valid(description: &String, tags: &Vec<Tag>) -> bool {
        let is_description_valid = Transaction::is_description_valid(description);
        let are_tags_valid = Transaction::are_tags_valid(tags);
        is_description_valid && are_tags_valid
    }
    
    /// Checks if a transaction can be created from the given raw parts.
    pub fn are_raw_parts_valid(value_string: &String, currency_string: &String, description: &String, tags: &Vec<Tag>) -> bool {
        let is_value_valid = Transaction::is_value_string_valid(value_string);
        let is_currency_valid = Transaction::is_currency_string_valid(currency_string);
        let is_description_valid = Transaction::is_description_valid(description);
        let are_tags_valid = Transaction::are_tags_valid(tags);
        is_value_valid && is_currency_valid && is_description_valid && are_tags_valid   
    }
    
    /// Returns whether a string can be parsed into a value.
    pub fn is_value_string_valid(value_string: &String) -> bool {
        Decimal::from_str(value_string).is_ok()
    }

    /// Returns whether a string can be parsed into a currency.
    pub fn is_currency_string_valid(currency_string: &String) -> bool {
        iso::find(&currency_string.to_uppercase()).is_some()
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
    /// Edits a transaction with raw parts.
    pub fn edit_with_raw_parts(&mut self, value_string: String, currency_string: String, date: Date, description: String, tags: Vec<Tag>) -> ResultStack<()> {
        if !Transaction::are_raw_parts_valid(&value_string, &currency_string, &description, &tags) { return ResultStack::new_fail("Failed to edit transaction from raw data!"); }

        let decimal_value_result = ResultStack::from_result(Decimal::from_str(&value_string), "Failed to convert value_string to Decimal.");
        let currency_result = ResultStack::from_option(iso::find(&currency_string.to_uppercase()), "Failed to convert currency_string to Currency.");

        match (&decimal_value_result, &currency_result) {
            (Pass(value), Pass(currency)) => {
                let value = Value::from_decimal(value.clone(), currency);
                self.value = value;
                self.date = date;
                self.description = description;
                self.tags = tags;
                Pass(())
            }
            _ => ResultStack::new_fail_from_unknown_failure(vec![decimal_value_result.get_possible_failures(), currency_result.get_possible_failures()])
        }
    }
    
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
    pub fn get_from(transactions: &mut Vec<Transaction>, id: Id) -> ResultStack<&mut Transaction> {
        let found_transaction = transactions.iter_mut().find(|trans|{
            if let Some(trans_id) = trans.id { return trans_id == id }
            return false
        });

        match found_transaction {
            None => { ResultStack::new_fail(&format!("Could not find transaction of id: {}.", id)) }
            Some(transaction) => { Pass(transaction) }
        }
    }

    /// Returns if the transaction contains the given tag.
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }
    
    /// Returns the sum value of all transactions in a given list..
    pub fn get_sum_value_from(transactions: &Vec<&Transaction>) -> ResultStack<f64> {
        let mut value_retrieval_failures = Vec::new();
        
        let sum_value = transactions.iter().map(|t| {
            let value_result = t.value.amount().to_f64();
            match value_result {
                Some(value) => value,
                None => {
                    value_retrieval_failures.push(ResultStack::from_option(value_result, "Failed to convert transaction value to f64."));
                    0.0
                }
            }
        }).sum();
        
        if value_retrieval_failures.is_empty() {
            ResultStack::Pass(sum_value)
        } else {
            value_retrieval_failures[0].fail("Failed to get sum value from transactions.")
        }
    }
    
    /// Returns a formated string of the time equivalent of the value
    pub fn get_time_price(value: &Value, price: f64) -> ResultStack<String> {
        let value_f64_option = match value.amount().to_f64() {
            Some(value_f64) => Some(value_f64.to_string()),
            None => None,
        };
        let value_f64_result = ResultStack::from_option(value_f64_option, "Failed to convert transaction value amount to f64");
        
        if let Pass(_) = value_f64_result {
            value_f64_result
        }
        else {
            return value_f64_result.fail("Failed to get time price.");
        }
    }
}



/// A custom date object tailored for tracking and parsing financial transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    year: u32,
    month: Months,
    day: u32,
}
impl Default for Date {
    /// Returns the default date: January 1, 1970.
    fn default() -> Self { Date { year: 1970, month: Months::January, day: 1 } }
}
impl Date {
    // initializing
    /// Creates a new date object.
    pub fn new(year: u32, month: Months, day: u32) -> ResultStack<Date> {
        if !Date::is_valid(year, &month, day) { return ResultStack::new_fail("Invalid date!"); }
        Pass(Date { year, month, day })
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
    pub fn edit(&mut self, year: u32, month: Months, day: u32) -> ResultStack<()> {
        if !Date::is_valid(year, &month, day) { return ResultStack::new_fail("Invalid date!"); }
        self.year = year;
        self.month = month;
        self.day = day;
        Pass(())
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
        format!("{} {}, {}", self.month.display(), self.day, self.year)
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn get_enum(month: u32) -> ResultStack<Months> {
        match month {
            1 => { Pass(Months::January) }
            2 => { Pass(Months::February) }
            3 => { Pass(Months::March) }
            4 => { Pass(Months::April) }
            5 => { Pass(Months::May) }
            6 => { Pass(Months::June) }
            7 => { Pass(Months::July) }
            8 => { Pass(Months::August) }
            9 => { Pass(Months::September) }
            10 => { Pass(Months::October) }
            11 => { Pass(Months::November) }
            12 => { Pass(Months::December) }
            _ => { ResultStack::new_fail("Invalid month value!") }
        }
    }

    /// Returns the number of days in the month for the given year (for leap year conditions).
    pub fn days_in_month(&self, year: u32) -> u32 {
        match self {
            Months::January | Months::March | Months::May | Months::July | Months::August | Months::October | Months::December => { 31 }
            Months::April | Months::June | Months::September | Months::November => { 30 }
            Months::February => { if Date::is_leap_year(year) { 29 } else { 28 } }
        }
    }

    /// Returns the next month.
    pub fn get_next(&self) -> Months {
        if self.as_value() >= 12 { return Months::January }
        Months::get_enum(self.as_value() + 1).wont_fail("Getting the next Month from an existing Month should never fail.")
    }

    /// Returns the previous month.
    pub fn get_previous(&self) -> Months {
        if self.as_value() <= 1 { return Months::December }
        Months::get_enum(self.as_value() - 1).wont_fail("Getting the previous Month from an existing Month should never fail.")
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
    pub fn new(label: String) -> ResultStack<Tag> {
        let validated_label_result = Self::validated_label(label);
        if let Pass(validated_label) = validated_label_result {
            Pass(Tag { label: validated_label })
        }
        else {
            ResultStack::new_fail("Failed to create new tag.")
        }
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
    pub fn edit(&mut self, new_label: String) -> ResultStack<()> {
        let validated_label_result = Self::validated_label(new_label);
        if let Pass(validated_label) = validated_label_result {
            self.label = validated_label;
            Pass(())
        }
        else {
            ResultStack::new_fail("Failed to edit tag.")
        }
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
    fn validated_label(new_label: String) -> ResultStack<String> {
        let new_label = new_label.trim().to_lowercase();
        if !Self::is_allowed(&new_label) { return ResultStack::new_fail("Invalid tag!"); }
        Pass(new_label)
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
    
    /// Returns the tags that are in a given list of transactions.
    pub fn get_tags_from(transactions: &Vec<&Transaction>) -> Vec<Tag> {
        Tag::sorted(transactions.into_iter().flat_map(|t| t.tags.clone()).collect())
    }
    
    /// Gets the percentage of the values of the transactions tagged with a given tag from a list of transactions.
    pub fn get_tag_percentage(tag: &Tag, transactions: &Vec<&Transaction>) -> ResultStack<f64> {
        let tagged_transactions = transactions.clone().into_iter().filter(|t| t.has_tag(tag)).collect::<Vec<&Transaction>>();
        let sum_value_result = Transaction::get_sum_value_from(&transactions);
        let tagged_value_result = Transaction::get_sum_value_from(&tagged_transactions);
        match (sum_value_result, tagged_value_result) {
            (Pass(sum_value), Pass(tagged_value)) => {
                if sum_value == 0.0 {
                    return ResultStack::new_fail("Sum value cannot be zero!").fail("Failed to calculate tag percentage.")
                }
                ResultStack::Pass(tagged_value / sum_value)
            }
            _ => ResultStack::new_fail("Failed to calculate tag percentage."),
        }
    }
}