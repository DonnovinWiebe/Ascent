use std::str::FromStr;
use chrono::{Local, Datelike};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use rusty_money::{iso, iso::Currency, Money};
use serde::{Deserialize, Serialize};
use crate::vault::{bank::CurrencyExchange};
use schrod::Schrod;
use schrod::Schrod::{Pass, Fail};
use std::hash::{Hash, Hasher};

/// A custom type that helps to clarify how the `Money` object is used in a `Transaction` context.
pub type Value = Money<'static, Currency>;

/// Used in each `Transaction` instead of a `usize` for clarity.
pub type Id = usize;



/// A list of styles for formatting `Tag` `label`s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TagStyles {
    Uppercase,
    Lowercase,
    Capitalized,
}



/// Stores all the information about a financial transaction.
/// Tags are relied upon heavily to create a fine-tuned web of information.
#[derive(Debug, Clone)]
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
    /// ( eating out, wants, ... ),
    /// ( home goods, ... ),
    /// ( gas, transportation, ... ),
    /// ...
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
    /// Creates a new `Transaction` from concrete values.
    /// This is intended to be used when a new `Transaction` is created from within the `App`.
    #[must_use]
    pub fn new_from_parts(id: Id, value: Value, date: Date, description: String, tags: Vec<Tag>) -> Schrod<Transaction> {
        if Transaction::are_parts_valid(&description, &tags) { Pass(Transaction { id: Some(id), value, date, description, tags }) }
        else {
            Schrod::new_fail("Invalid parts!", "Transaction::new_from_parts()")
                .fail("Failed to create Transaction.", "Transaction::new_from_parts()")
        }
    }

    /// Creates a new `Transaction` from raw data parts.
    /// This is intended to be used when a new `Transaction` is created from within the `App`.
    #[must_use]
    pub fn new_from_raw(id: Id, value_string: &str, currency_string: &str, date: Date, description: String, tags: Vec<Tag>) -> Schrod<Transaction> {
        if !Transaction::are_raw_parts_valid(value_string, currency_string, &description, &tags) {
            return Schrod::new_fail("Invalid parts!", "Transaction::new_from_raw()")
                .fail("Failed to create Transaction.", "Transaction::new_from_parts()")
        }

        let decimal_value_result = Schrod::from_result(Decimal::from_str(value_string), "Failed to convert value_string to Decimal.", "Transaction::new_from_raw()");
        let currency_result = Schrod::from_option(iso::find(&currency_string.to_uppercase()), "Failed to convert currency_string to Currency.", "Transaction::new_from_raw()");

        if let (Pass(value), Pass(currency)) = (&decimal_value_result, &currency_result) {
            Pass(Transaction { id: Some(id), value: Value::from_decimal(*value, currency), date, description, tags })
        }

        else {
            let results = vec![decimal_value_result, currency_result.convert("Transaction::new_from_raw()")];
            Schrod::collect_and_fail(&results, "Transaction::new_from_raw()")
                .convert("Transaction::new_from_raw()")
                .fail("Failed to create Transaction.", "Transaction::new_from_parts()")
        }
    }
    
    /// Creates a new `Transaction` from concrete values.
    /// This is intended to be used when an existing `Transaction` is loaded from `SaveData`.
    /// Please note that if this function is used, an `Id` must be filled in later with `set_id()`.
    #[must_use]
    pub fn load_from_parts(value: Value, date: Date, description: String, tags: Vec<Tag>) -> Schrod<Transaction> {
        if Transaction::are_parts_valid(&description, &tags) { Pass(Transaction { id: None, value, date, description, tags }) }
        else {
            Schrod::new_fail("Invalid parts!", "Transaction::load_from_parts()")
                .fail("Failed to load Transaction.", "Transaction::load_from_parts()")
        }
    }

    /// Loads a new `Transaction` from raw data parts.
    /// This is intended to be used when an existing `Transaction` is loaded from `SaveData`.
    /// Please note that if this function is used, an `Id` must be filled in later with `set_id()`.
    #[must_use]
    pub fn load_from_raw(value_string: &str, currency_string: &str, date: Date, description: String, tags: Vec<Tag>) -> Schrod<Transaction> {
        if !Transaction::are_raw_parts_valid(value_string, currency_string, &description, &tags) {
            return Schrod::new_fail("Invalid parts!", "Transaction::load_from_raw()")
                .fail("Failed to load Transaction.", "Transaction::load_from_raw()")
        }

        let decimal_value_result = Schrod::from_result(Decimal::from_str(value_string), "Failed to convert value_string to Decimal.", "Transaction::load_from_raw()");
        let currency_result = Schrod::from_option(iso::find(&currency_string.to_uppercase()), "Failed to convert currency_string to Currency.", "Transaction::load_from_raw()");

        if let (Pass(value), Pass(currency)) = (&decimal_value_result, &currency_result) {
            Pass(Transaction { id: None, value: Value::from_decimal(*value, currency), date, description, tags })
        }
        
        else {
            let results = vec![decimal_value_result, currency_result.convert("Transaction::load_from_raw()")];
            Schrod::collect_and_fail(&results, "Transaction::load_from_raw()")
                .convert("Transaction::load_from_raw()")
                .fail("Failed to load Transaction.", "Transaction::load_from_raw()")
        }
    }
    
    /// Sets the `Id` of a `Transaction` that does not have an id.
    /// Used primarily for `Transaction`s that are loaded from `SaveData`.
    #[must_use]
    pub fn set_id(&mut self, id: Id) -> Schrod<()> {
        if self.id.is_some() { return Schrod::new_fail("Failed to set Transaction ID because it is already set!", "Transaction::set_id()"); }
        self.id = Some(id);
        Pass(())
    }
    
    /// Overrides the `Id` of a `Transaction`.
    /// Used primarily for re-indexing `Transaction`s.
    pub fn override_id(&mut self, id: Id) {
        self.id = Some(id);
    }



    // validating
    /// Checks if a `Transaction` can be created from the given parts.
    #[must_use]
    pub fn are_parts_valid(description: &str, tags: &[Tag]) -> bool {
        let is_description_valid = Transaction::is_description_valid(description);
        let are_tags_valid = Transaction::are_tags_valid(tags);
        is_description_valid && are_tags_valid
    }
    
    /// Checks if a `Transaction` can be created from the given raw parts.
    #[must_use]
    pub fn are_raw_parts_valid(value_string: &str, currency_string: &str, description: &str, tags: &[Tag]) -> bool {
        let is_value_valid = Transaction::is_value_string_valid(value_string);
        let is_currency_valid = Transaction::is_currency_string_valid(currency_string);
        let is_description_valid = Transaction::is_description_valid(description);
        let are_tags_valid = Transaction::are_tags_valid(tags);
        is_value_valid && is_currency_valid && is_description_valid && are_tags_valid   
    }
    
    /// Returns whether a `String` can be parsed into a `Value`.
    #[must_use]
    pub fn is_value_string_valid(value_string: &str) -> bool {
        Decimal::from_str(value_string).is_ok()
    }

    /// Returns whether a `String` can be parsed into a `Currency`.
    #[must_use]
    pub fn is_currency_string_valid(currency_string: &str) -> bool {
        iso::find(&currency_string.to_uppercase()).is_some()
    }

    /// Determines if the given `description` is valid.
    #[must_use]
    pub fn is_description_valid(description: &str) -> bool {
        Tag::is_allowed(description)
    }

    /// Determines if the given list of `Tag`s is valid.
    /// Every `Tag` in the list is already guaranteed to be valid.
    #[must_use]
    pub fn are_tags_valid(tags: &[Tag]) -> bool {
        !tags.is_empty()
    }



    // management
    /// Edits a `Transaction` with raw parts.
    #[must_use]
    pub fn edit_with_raw_parts(&mut self, value_string: &str, currency_string: &str, date: Date, description: String, tags: Vec<Tag>) -> Schrod<()> {
        if !Transaction::are_raw_parts_valid(value_string, currency_string, &description, &tags) {
            return Schrod::new_fail("Invalid parts!", "Transaction::edit_with_raw_parts()")
                .fail("Failed to edit Transaction.", "Transaction::edit_with_raw_parts()")
        }

        let decimal_value_result = Schrod::from_result(Decimal::from_str(value_string), "Failed to convert value_string to Decimal.", "Transaction::edit_with_raw_parts()");
        let currency_result = Schrod::from_option(iso::find(&currency_string.to_uppercase()), "Failed to convert currency_string to Currency.", "Transaction::edit_with_raw_parts()");

        if let (Pass(decimal), Pass(currency)) = (&decimal_value_result, &currency_result) {
            let value = Value::from_decimal(*decimal, currency);
            self.value = value;
            self.date = date;
            self.description = description;
            self.tags = tags;
            Pass(())
        }
        
        else {
            let results = vec![decimal_value_result, currency_result.convert("Transaction::edit_with_raw_parts()")];
            Schrod::collect_and_fail(&results, "Transaction::edit_with_raw_parts()")
                .convert("Transaction::edit_with_raw_parts()")
                .fail("Failed to load Transaction.", "Transaction::edit_with_raw_parts()")
        }
        
    }
    
    /// Adds a new `Tag` and sorts the `Tag` list.
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
        self.tags = Tag::sorted(&self.tags);
    }

    /// Removes a given `Tag` from the list.
    pub fn remove_tag(&mut self, tag: &Tag) {
        if self.tags.len() <= 1 { return; }
        self.tags.retain(|t| t != tag);
    }



    // data retrieval and parsing
    /// Returns the internal `Id`.
    #[must_use]
    pub fn get_id(&self) -> Option<Id> {
        self.id
    }
    
    /// Returns a mutable reference to the `Transaction` with the given `Id`.
    #[must_use]
    pub fn get_from(transactions: &mut [Transaction], id: Id) -> Schrod<&mut Transaction> {
        let found_transaction = transactions.iter_mut().find(|trans|{
            if let Some(trans_id) = trans.id { return trans_id == id }
            false
        });

        match found_transaction {
            None => { Schrod::new_fail(&format!("Could not find transaction of id: {id}."), "Transaction::get_from()") }
            Some(transaction) => { Pass(transaction) }
        }
    }

    /// Returns if the `Transaction` contains the given `Tag`.
    #[must_use]
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.contains(tag)
    }
    
    /// Returns the sum value of all `Transaction`s in a given list.
    #[must_use]
    pub fn get_sum_value_from(transactions: &[&Transaction]) -> Decimal {
        transactions.iter().map(|t| { t.value.amount() }).sum()
    }
    
    /// Returns the value of the `Value` in hours.
    // todo: implement
    #[must_use]
    pub fn get_time_price(&self, currency_exchange: &CurrencyExchange) -> Decimal {
        let time_price_result = currency_exchange.as_time_price(&self.value);
        match time_price_result {
            Pass(time_price) => { time_price }
            Fail(_) => { Decimal::from(0) }
        }
    }
}



/// A custom `Date` object tailored for tracking and parsing financial `Transaction`s.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Date {
    year: u32,
    month: Months,
    day: u32,
}
impl Default for Date {
    /// Returns the default `Date`: April 5, 2026.
    fn default() -> Self { Date { year: Date::default_year(), month: Date::default_month(), day: Date::default_day() } }
}
impl Date {
    // initializing and defaults
    /// Creates a new `Date` object.
    #[must_use]
    pub fn new(year: u32, month: Months, day: u32) -> Schrod<Date> {
        if !Date::is_valid(year, month, day) {
            return Schrod::new_fail("Invalid date!", "Date::new()")
                    .fail("Failed to create Date.", "Date::new()")
        }
        Pass(Date { year, month, day })
    }
    
    /// Gets the default year.
    #[must_use]
    pub fn default_year() -> u32 {
        2026
    }
    
    /// Gets the default `Month`.
    #[must_use]
    pub fn default_month() -> Months {
        Months::April
    }
    
    /// Gets the default day.
    #[must_use]
    pub fn default_day() -> u32 {
        5
    }
    
    /// Creates a `Date` from a value (YYYYMMDD format).
    #[must_use]
    pub fn from_value(value: u32) -> Schrod<Date> {
        let year = value / 10000;
        let month_value = (value % 10000) / 100;
        let day = value % 100;
        
        let month_result = Months::from_value(month_value);
        if month_result.is_fail() {
            return month_result
                .convert("Date::from_value()")
                .fail("Failed to create Date!", "Date::from_value()")
        }
        let month = month_result.wont_fail("This is past an is_fail() guard clause.", "Date::from_value()");
        
        Date::new(year, month, day)
    }

    /// Returns today's date as a `Date`.
    #[must_use]
    pub fn today() -> Schrod<Date> {
        let now = Local::now();
        let month_result = Months::from_value(now.month());
        if month_result.is_fail() {
            return month_result
                .convert("Date::today()")
                .fail("Failed to get today's Date!", "Date::today()")
        }
        let month = month_result.wont_fail("This is past an is_fail() guard clause.", "Date::today()");
        #[allow(clippy::cast_sign_loss)]
        let today_result = Date::new(now.year() as u32, month, now.day());
        if today_result.is_fail() {
            return today_result
                .convert("Date::today()")
                .fail("Failed to get today's Date!", "Date::today()")
        }
        today_result
    }



    // validating
    /// Determines if a `Date` can exist with the given data.
    #[must_use]
    fn is_valid(year: u32, month: Months, day: u32) -> bool {
        Date::is_year_valid(year) && Date::is_day_valid(day, month, year)
    }
    
    /// Determines if a year is valid.
    /// Some formatting assumes the year to always be four digits long.
    #[must_use]
    fn is_year_valid(year: u32) -> bool {
        (1000..=9999).contains(&year)
    }
    
    /// Determines if a day is valid for the given `Month` and year.
    #[must_use]
    fn is_day_valid(day: u32, month: Months, year: u32) -> bool {
        day <= month.days_in_month(year)
    }



    // management
    /// Updates the `Date` with new values.
    #[must_use]
    pub fn edit(&mut self, year: u32, month: Months, day: u32) -> Schrod<()> {
        if !Date::is_valid(year, month, day) {
            return Schrod::new_fail("Invalid parts!", "Date::edit()")
                .fail("Failed to edit Date.", "Date::edit()")
        }
        self.year = year;
        self.month = month;
        self.day = day;
        Pass(())
    }

    /// Gets the next year.
    #[must_use]
    pub fn get_advanced_year(year: u32) -> u32 {
        let new_year = year + 1;
        if Date::is_year_valid(new_year) { new_year }
        else { year }
    }
    
    /// Gets the previous year.
    #[must_use]
    pub fn get_receded_year(year: u32) -> u32 {
        let new_year = year - 1;
        if Date::is_year_valid(new_year) { new_year }
        else { year }
    }
    
    /// Gets the next day.
    #[must_use]
    pub fn get_advanced_day(day: u32, month: Months, year: u32) -> u32 {
        if day < month.days_in_month(year) { day + 1 } else { 1 }
    }
    
    /// Gets the previous day.
    #[must_use]
    pub fn get_receded_day(day: u32, month: Months, year: u32) -> u32 {
        if day > 1 { day - 1 } else { month.get_previous().days_in_month(year) }
    }
    
    /// Advances the `Date` by one year.
    pub fn advance_by_year(&mut self) {
        self.year = Date::get_advanced_year(self.year);
    }

    /// Recedes the `Date` by one year.
    pub fn recede_by_year(&mut self) {
        self.year = Date::get_receded_year(self.year);
    }

    /// Advances the `Date` by one `Month`.
    pub fn advance_by_month(&mut self) {
        self.month = self.month.get_next();
        if self.month == Months::January { self.advance_by_year(); }
        
        if self.day > self.month.days_in_month(self.year) { self.day = self.month.days_in_month(self.year); }
    }

    /// Recedes the `Date` by one `Month`.
    pub fn recede_by_month(&mut self) {
        self.month = self.month.get_previous();
        if self.month == Months::December { self.recede_by_year(); }
        
        if self.day > self.month.days_in_month(self.year) { self.day = self.month.days_in_month(self.year); }
    }

    /// Advances the `Date` by one day.
    pub fn advance_by_day(&mut self) {
        // advances the day
        self.day += 1;
        
        // if the day exceeds the number of days in the current month,
        // the month is advanced and the day is set to 1
        if self.day > self.month.days_in_month(self.year) {
            self.month = self.month.get_next();
            self.day = 1;
            
            // if the month that was advanced to is January, the year is advanced as well
            if self.month == Months::January { self.year = Date::get_advanced_year(self.year); }
        }
    }

    /// Recedes the `Date` by one day.
    pub fn recede_by_day(&mut self) {
        // recedes the day
        self.day -= 1;
        
        // if the day is less than 1, the month is receded and the
        // day is set to the last day of the month
        if self.day < 1 {
            self.month = self.month.get_previous();
            self.day = self.month.days_in_month(self.year);
            
            // if the month that was receded to is December, the year is receded as well
            if self.month == Months::December { self.year = Date::get_receded_year(self.year); }
        }
    }



    // data retrieval and parsing
    /// Returns a formatted `String` representation of the `Date`.
    #[must_use]
    pub fn display(&self) -> String {
        format!("{} {}, {}", self.month.display(), self.day, self.year)
    }

    /// Returns the year.
    #[must_use]
    pub fn get_year(&self) -> u32 {
        self.year
    }

    /// Returns the `Month`.
    #[must_use]
    pub fn get_month(&self) -> Months {
        self.month
    }

    /// Returns the day.
    #[must_use]
    pub fn get_day(&self) -> u32 {
        self.day
    }

    /// Returns the `Date` as a `u32` value.
    /// Example: 20260206 - February 5, 2026
    #[must_use]
    pub fn as_value(&self) -> u32 {
        let year = self.year * 10000;
        let month = self.month.as_value() * 100;
        let day = self.day;
        year + month + day
    }

    /// Determines whether the given year is a leap year.
    #[must_use]
    fn is_leap_year(year: u32) -> bool {
        year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
    }

    /// Gets the number of days between two `Date`s.
    #[must_use]
    pub fn get_days_between(&self, other: Date) -> usize {
        // the date info
        let earlier;
        let later;
        if self.as_value() > other.as_value() {
            later = *self;
            earlier = other;
        }
        else {
            later = other;
            earlier = *self;
        }

        // same year and month
        if earlier.year == later.year && earlier.month == later.month {
            return (later.day - earlier.day) as usize
        }

        // same year
        if earlier.year == later.year {
            let mut days = later.day as usize + (earlier.month.days_in_month(earlier.year) - earlier.day) as usize;
            if later.month.as_value() - earlier.month.as_value() > 0 {
                for i in (earlier.month.as_value() + 1)..later.month.as_value() {
                    let month = Months::from_value(i).wont_fail("This will never fail.", "Date::get_days_between()");
                    days += month.days_in_month(earlier.year) as usize;
                }
            }
            return days
        }

        // all different
        // days in months for earlier and later date months
        let mut days = later.day as usize + (earlier.month.days_in_month(earlier.year) - earlier.day) as usize;
        // days until later date month
        if later.month.as_value() > 1 {
            for i in 1..later.month.as_value() {
                let month = Months::from_value(i).wont_fail("This will never fail.", "Date::get_days_between()");
                days += month.days_in_month(later.year) as usize;
            }
        }
        // days from earlier date month
        if earlier.month.as_value() < 12 {
            for i in (earlier.month.as_value() + 1)..=12 {
                let month = Months::from_value(i).wont_fail("This will never fail.", "Date::get_days_between()");
                days += month.days_in_month(earlier.year) as usize;
            }
        }
        // other years between
        if later.year - earlier.year > 0 {
            for year in (earlier.year + 1)..later.year {
                if Date::is_leap_year(year) { days += 366; }
                else { days += 365; }
            }
        }
        // returning
        days
    }
}



/// A custom `enum` for the `Month` component of the `Date` `struct`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// Returns a formatted `String` representation of the `Month`.
    #[must_use]
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

    /// Returns the numeric equivalent of the `Month`.
    #[must_use]
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

    /// Returns the `enum` equivalent of a `Month`'s numeric value.
    #[must_use]
    pub fn from_value(month: u32) -> Schrod<Months> {
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
            _ => {
                Schrod::new_fail("Invalid month value!", "Months::from_value()")
                    .fail("Failed to get Month from value.", "Months::from_value()")
            }
        }
    }

    /// Returns the number of days in the `Month` for the given year (for leap year conditions).
    #[must_use]
    pub fn days_in_month(&self, year: u32) -> u32 {
        match self {
            Months::January | Months::March | Months::May | Months::July | Months::August | Months::October | Months::December => { 31 }
            Months::April | Months::June | Months::September | Months::November => { 30 }
            Months::February => { if Date::is_leap_year(year) { 29 } else { 28 } }
        }
    }

    /// Returns the next `Month`.
    #[must_use]
    pub fn get_next(&self) -> Months {
        if self.as_value() >= 12 { return Months::January }
        Months::from_value(self.as_value() + 1).wont_fail("Getting the next Month from an existing Month should never fail.", "Months::get_next()")
    }

    /// Returns the previous `Month`.
    #[must_use]
    pub fn get_previous(&self) -> Months {
        if self.as_value() <= 1 { return Months::December }
        Months::from_value(self.as_value() - 1).wont_fail("Getting the previous Month from an existing Month should never fail.", "Months::get_previous()")
    }

    /// Returns the previous `Month`s in the year before the given `Month`.
    /// The given `Month` itself is not included.
    #[must_use]
    pub fn get_previous_months(&self) -> Vec<Months> {
        match self {
            Months::January   => Vec::new(),
            Months::February  => vec![Months::January],
            Months::March     => vec![Months::January, Months::February],
            Months::April     => vec![Months::January, Months::February, Months::March],
            Months::May       => vec![Months::January, Months::February, Months::March, Months::April],
            Months::June      => vec![Months::January, Months::February, Months::March, Months::April, Months::May],
            Months::July      => vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June],
            Months::August    => vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July],
            Months::September => vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August],
            Months::October   => vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September],
            Months::November  => vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September, Months::October],
            Months::December  => vec![Months::January, Months::February, Months::March, Months::April, Months::May, Months::June, Months::July, Months::August, Months::September, Months::October, Months::November],
        }
    }
}



/// A custom tag object tailored for parsing and sorting `Transaction`s with overlapping categories.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
impl Eq for Tag {}
impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}
impl Tag {
    // initializing
    /// Creates a new `Tag`.
    #[must_use]
    pub fn new(label: &str) -> Schrod<Tag> {
        let validated_label_result = Self::validated_label(label);
        if let Pass(validated_label) = validated_label_result {
            Pass(Tag { label: validated_label })
        }
        else {
            Schrod::new_fail("Invalid label!", "Tag::new()")
                .fail("Failed to create new tag.", "Tag::new()")
        }
    }



    // validating
    /// Returns whether the input is a valid `Tag` `label`.
    #[must_use]
    pub fn is_allowed(input: &str) -> bool {
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
    /// Edits the `Tag` `label`.
    #[must_use]
    pub fn edit(&mut self, new_label: &str) -> Schrod<()> {
        let validated_label_result = Self::validated_label(new_label);
        if let Pass(validated_label) = validated_label_result {
            self.label = validated_label;
            Pass(())
        }
        else {
            Schrod::new_fail("Invalid label!", "Tag::edit()")
                .fail("Failed to edit tag.", "Tag::edit()")
        }
    }



    // data retrieval and parsing
    /// Returns a formatted `label` based on a given `TagStyle`.
    #[must_use]
    pub fn display(&self, style: TagStyles) -> String {
        match style {
            TagStyles::Uppercase => { self.label.to_uppercase() }
            TagStyles::Lowercase => { self.label.clone() } // already lowercase
            TagStyles::Capitalized => {
                let words: Vec<&str> = self.label.split(' ').collect(); // This means that each word will have at least one character
                let mut capitalized_words: Vec<String> = Vec::new();

                for word in words {
                    let mut characters: Vec<char> = word.chars().collect();
                    characters[0] = characters[0].to_uppercase().nth(0).unwrap_or('#'); // This should never fail, but "#" is used as a failsafe
                    capitalized_words.push(characters.into_iter().collect());
                }

                capitalized_words.join(" ")
            }
        }
    }

    /// Returns a reference to the `Tag` `label`.
    #[must_use]
    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    /// Returns a validated `Tag` `label` to ensure it only contains allowed characters.
    #[must_use]
    fn validated_label(new_label: &str) -> Schrod<String> {
        let new_label = new_label.trim().to_lowercase();
        if !Self::is_allowed(&new_label) {
            return Schrod::new_fail("Tag contains invalid characters!", "Tag::validated_label()")
                .fail("Failed to get validated Tag label!", "Tag::validated_label()")
        }
        Pass(new_label)
    }

    /// Determines if the `Tag` contains another `Tag`.
    #[must_use]
    pub fn contains(&self, other_tag: &Tag) -> bool {
        self.label.contains(&other_tag.label)
    }

    /// Returns a new list of `Tag`s that doesn't have duplicates.
    #[must_use]
    pub fn without_duplicates(list: Vec<Tag>) -> Vec<Tag> {
        let mut unique_tags: Vec<Tag> = Vec::new();

        for tag in list {
            if !unique_tags.contains(&tag) { unique_tags.push(tag); }
        }

        unique_tags
    }

    /// Returns a new list of `Tag`s sorted alphabetically.
    #[must_use]
    pub fn sorted(list: &[Tag]) -> Vec<Tag> {
        let mut sorted_tags: Vec<Tag> = list.to_vec();
        sorted_tags.sort_by(|a, b| a.label.cmp(&b.label));
        sorted_tags = Self::without_duplicates(sorted_tags);
        sorted_tags
    }
    
    /// Returns the `Tag`s that are in a given list of `Transaction`s.
    #[must_use]
    pub fn get_tags_from(transactions: &Vec<&Transaction>) -> Vec<Tag> {
        Tag::sorted(&transactions.iter().flat_map(|t| t.tags.clone()).collect::<Vec<Tag>>())
    }
    
    /// Gets the percentage of the `Value`s of the `Transaction`s tagged with a given `Tag` from a list of `Transaction`s.
    #[must_use]
    pub fn get_tag_percentage(tag: &Tag, transactions: &Vec<&Transaction>) -> Schrod<f64> {
        let tagged_transactions = transactions.clone().into_iter().filter(|t| t.has_tag(tag)).collect::<Vec<&Transaction>>();
        let sum_value = Transaction::get_sum_value_from(transactions);
        let tagged_value = Transaction::get_sum_value_from(&tagged_transactions);
        Schrod::from_option((tagged_value / sum_value).to_f64(), "Failed to convert Tag percentage to f64.", "Tag::get_tag_percentage()")
    }
}