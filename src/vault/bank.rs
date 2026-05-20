use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rusty_money::iso;
use rusty_money::iso::Currency;
use serde::{Deserialize, Serialize};

use crate::ui::material::MaterialColors;
use crate::vault::filter::Filter;
use crate::vault::transaction::{Date, Id, Months, Tag, Transaction, Value};
use crate::vault::schrod::Schrod;
use crate::vault::schrod::Schrod::{Pass, Fail};

/// An enumeration of the available `Filter`s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filters {
    Primary,
    DeepDive1,
    DeepDive2,
}



/// Holds a list of all the `Transaction`s.
pub struct Bank {
    /// The central list of all `Transaction`s.
    ledger: Vec<Transaction>,
    /// Holds local copies of all necessary currency exchange rates.
    pub currency_exchange: CurrencyExchange,
    /// The `TagRegistry`.
    pub tag_registry: TagRegistry,
    /// The central `Id` tracker for new `Transaction`s.
    id_tracker: Id,
    /// The primary `Filter`.
    pub primary_filter: Filter,
    /// The first deep dive `Filter`.
    pub deep_dive_1_filter: Filter,
    /// The second deep dive `Filter`.
    pub deep_dive_2_filter: Filter,
}
impl Default for Bank {
    /// Creates a new default `Bank` object.
    fn default() -> Bank {
        Bank::new()
    }
}
impl Bank {
    // initializing
    /// Creates a new `Bank` object.
    #[must_use]
    fn new() -> Bank {
        Bank {
            ledger: Vec::new(),
            currency_exchange: CurrencyExchange::default(),
            tag_registry: TagRegistry::new(),
            id_tracker: 0,
            primary_filter: Filter::default(),
            deep_dive_1_filter: Filter::default(),
            deep_dive_2_filter: Filter::default(),
        }
    }

    /// Initializes the `Bank`.
    pub fn init(&mut self, transactions: Vec<Transaction>, currency_exchange: CurrencyExchange, tag_registry: TagRegistry) -> Schrod<()> {
        let load_result = self.load_transactions(transactions);
        if load_result.is_fail() { return load_result.fail("Failed to initialize the Bank!", "Bank::init()"); }
        let init_filter_dates_result = self.init_filter_dates();
        if init_filter_dates_result.is_fail() { return init_filter_dates_result.fail("Failed to initialize the Bank!", "Bank::init()"); }
        self.currency_exchange = currency_exchange;
        self.tag_registry = tag_registry;
        Pass(())
    }
    
    /// Loads `Transaction`s into the `Bank`.
    /// This is used when loading from `SaveData`.
    #[must_use]
    pub fn load_transactions(&mut self, transactions: Vec<Transaction>) -> Schrod<()> {
        let mut new_ledger = Vec::new();
        for mut transaction in transactions {
            let set_result = transaction.set_id(self.get_next_id()); // uses set_id() instead of override_id() to ensure proper data flow
            if set_result.is_fail() { return set_result.fail("Could not load transactions into ledger!", "Bank::load_transactions()"); }
            new_ledger.push(transaction);
        }
        self.ledger = new_ledger;
        let filter_result = self.refilter();
        if filter_result.is_fail() { return filter_result.fail("Could not filter the new loaded ledger.", "Bank::load_transactions()"); }
        Pass(())
    }



    // management
    /// Gets a copy of the `ledger`.
    /// Please note that modifying these `Transaction`s has no effect on the `Bank`'s internal `ledger`.
    #[must_use]
    pub fn get_ledger_copy(&self) -> Vec<Transaction> {
        self.ledger.clone()
    }
    
    /// Gets the next available `Id`.
    #[must_use]
    pub fn get_next_id(&mut self) -> Id {
        let id_to_return = self.id_tracker;
        self.id_tracker += 1;
        id_to_return
    }
    
    /// Re-indexes all `Transaction`s in the `ledger` to help make `Transaction` `Id`s more closely align with their index in the `ledger`.
    pub fn reindex_transactions(&mut self) {
        self.id_tracker = 0;
        for i in 0..self.ledger.len() {
            let id = self.get_next_id();
            self.ledger[i].override_id(id);
        }
    }
    
    /// Sorts a ledger by `Date`.
    #[must_use]
    pub fn sorted_ledger(ledger: Vec<Transaction>) -> Vec<Transaction> {
        let mut ledger = ledger;
        ledger.sort_by(|a, b| b.date.as_value().cmp(&a.date.as_value()));
        ledger
    }

    /// Sorts the `ledger` by `Date`.
    fn sort_ledger(&mut self) {
        // I could duplicate sorted_ledger() here, but this is faster
        self.ledger.sort_by(|a, b| b.date.as_value().cmp(&a.date.as_value()));
    }

    /// Adds a new `Transaction` from concrete values.
    /// This is intended to be used when a new `Transaction` is created from within the `App`.
    #[must_use]
    pub fn add_transaction_from_parts(&mut self, value: Value, date: Date, description: String, tags: Vec<Tag>) -> Schrod<()> {
        let id = self.get_next_id();
        let transaction_result = Transaction::new_from_parts(id, value, date, description, tags);
        
        if let Pass(transaction) = transaction_result {
            self.ledger.push(transaction);
            let filter_result = self.refilter();
            if filter_result.is_fail() { return filter_result; }
            Pass(())
        }
        
        else {
            transaction_result
                .fail("Failed to add transaction from parts.", "Bank::add_transaction_from_parts()")
                .convert("Bank::add_transaction_from_parts()")
        }
    }

    /// Creates a new `Transaction` from raw data parts.
    /// This is intended to be used when a new `Transaction` is created from within the `App`.
    #[must_use]
    pub fn add_transaction_from_raw_parts(&mut self, value_string: &str, currency_string: &str, date: Date, description: String, tags: Vec<Tag>) -> Schrod<()> {
        let id = self.get_next_id();
        let transaction_result = Transaction::new_from_raw(id, value_string, currency_string, date, description, tags);
        
        if let Pass(transaction) = transaction_result {
            self.ledger.push(transaction);
            let filter_result = self.refilter();
            if filter_result.is_fail() { return filter_result; }
            Pass(())
        }
        
        else {
            transaction_result
                .fail("Failed to add a new transaction from raw parts.", "Bank::add_transaction_from_raw_parts()")
                .convert("Bank::add_transaction_from_raw_parts()")
        }
    }

    /// Edits a `Transaction` with raw parts.
    #[must_use]
    pub fn edit_transaction_with_raw_parts(&mut self, id: Id, value_string: &str, currency_string: &str, date: Date, description: String, tags: Vec<Tag>) -> Schrod<()> {
        let transaction_result = self.get_mut(id);
        
        if let Pass(transaction) = transaction_result {
            let edit_result = transaction.edit_with_raw_parts(value_string, currency_string, date, description, tags);
            match edit_result {
                Pass(()) => { self.refilter() }
                Fail(_) => { edit_result }
            }
        }
        
        else {
            transaction_result
                .fail("Failed to edit a transaction with raw parts.", "Bank::edit_transaction_with_raw_parts()")
                .convert("Bank::edit_transaction_with_raw_parts()")
        }
    }

    /// Removes a `Transaction` from the `ledger`.
    #[must_use]
    pub fn remove_transaction(&mut self, id: Id) -> Schrod<()> {
        for i in 0..self.ledger.len() {
            let transaction = &mut self.ledger[i];
            if let Some(transaction_id) = transaction.get_id() && transaction_id == id {
                self.ledger.remove(i);
                let filter_result = self.refilter();
                if filter_result.is_fail() { return filter_result; }
                return Pass(());
            }
        }
        
        Schrod::new_fail("Transaction could not be found!", "Bank::remove_transaction()")
    }
    
    /// Returns an updated `TagRegistry` to match the current `Tag`s in the `ledger`.
    #[must_use]
    pub fn get_updated_tag_registry(tag_registry: TagRegistry, tags: Vec<Tag>) -> TagRegistry {
        let mut updated_tag_registry = tag_registry;
        updated_tag_registry.update_registry(tags);
        updated_tag_registry
    }



    // data retrieval and parsing
    /// Returns a mutable reference to the `ledger`.
    #[must_use]
    pub fn get_ledger_mut(&mut self) -> &mut Vec<Transaction> {
        &mut self.ledger
    }

    /// Returns an immutable reference to the `ledger`.
    #[must_use]
    pub fn get_ledger(&self) -> &Vec<Transaction> {
        &self.ledger
    }

    /// Returns an immutable reference to a `Transaction`.
    #[must_use]
    pub fn get(&self, id: Id) -> Schrod<&Transaction> {
        for transaction in &self.ledger { // todo start searching at index = id for efficiency
            if let Some(transaction_id) = transaction.get_id() && transaction_id == id {
                return Pass(transaction);
            }
        }
        
        Schrod::new_fail("Transaction could not be found!", "Bank::get()")
    }

    /// Returns a mutable reference to a `Transaction`.
    #[must_use]
    pub fn get_mut(&mut self, id: Id) -> Schrod<&mut Transaction> {
        for transaction in &mut self.ledger { // todo start searching at index = id for efficiency
            if let Some(transaction_id) = transaction.get_id() && transaction_id == id {
                return Pass(transaction);
            }
        }
        
        Schrod::new_fail("Transaction could not be found!", "Bank::get_mut()")
    }

    /// Gets the `Id`s from a list of `Transaction`s.
    pub fn get_ids_from(transactions: &Vec<&Transaction>) -> Vec<Id> {
        transactions.into_iter().map(|t| t.get_id()).flatten().collect()
    }

    /// Returns a list of existing `Tag`s
    #[must_use]
    pub fn get_tags(&self) -> Vec<Tag> {
        let mut tags = Vec::new();
        for transaction in &self.ledger {
            tags.extend(transaction.tags.clone());
        }
        Tag::sorted(&tags)
    }
    
    /// Gets the `Date` of the latest `Transaction` in the ledger.
    /// If the `ledger` is empty, this returns the default `Date`.
    #[must_use]
    pub fn get_latest_date(&self) -> Date {
        self.ledger.first().map(|t| t.date).unwrap_or_default()
    }
    
    /// Gets a list of the `Transaction` `Id`s filtered by the given `Filter`.
    #[must_use]
    pub fn get_filtered_ids(&self, filter: Filters) -> Vec<Id> {
        self.get_filter(filter).get_filtered_ids()
    }
    
    /// Gets the `Date` of the latest `Transaction` from a given `Filter`.
    /// If the `Filter` is empty, this returns the default `Date`.
    #[must_use]
    pub fn get_latest_date_for_filter(&self, filter: Filters) -> Date {
        let filtered_ids = self.get_filter(filter).get_filtered_ids();
        
        let transactions = self.ledger.iter().filter(|ledger_transaction| {
            let ledger_transaction_id_result = &ledger_transaction.get_id();
            match ledger_transaction_id_result {
                Some(ledger_transaction_id) => filtered_ids.contains(ledger_transaction_id),
                None => false,
            }
        }).collect::<Vec<_>>();

        transactions.first().map(|t| t.date).unwrap_or_default()
    }
    
    /// Returns an immutable reference to a `Filter`.
    #[must_use]
    pub fn get_filter(&self, filter: Filters) -> &Filter {
        match filter {
            Filters::Primary => &self.primary_filter,
            Filters::DeepDive1 => &self.deep_dive_1_filter,
            Filters::DeepDive2 => &self.deep_dive_2_filter,
        }
    }
    
    /// Returns a mutable reference to a `Filter`.
    #[must_use]
    pub fn get_filter_mut(&mut self, filter: Filters) -> &mut Filter {
        match filter {
            Filters::Primary => &mut self.primary_filter,
            Filters::DeepDive1 => &mut self.deep_dive_1_filter,
            Filters::DeepDive2 => &mut self.deep_dive_2_filter,
        }
    }
    
    /// Sets the `year` and `month` of each `Filter` to the latest `Date` in the `ledger`.
    #[must_use]
    pub fn init_filter_dates(&mut self) -> Schrod<()> {
        let latest_date = self.get_latest_date();
        
        let set_year_result = self.set_filter_year(latest_date.get_year(), Filters::Primary);
        if set_year_result.is_fail() { return set_year_result.fail("Failed to initialize filter dates!", "Bank::init_filter_dates()"); }
        let set_month_result = self.set_filter_month(latest_date.get_month(), Filters::Primary);
        if set_month_result.is_fail() { return set_month_result.fail("Failed to initialize filter dates!", "Bank::init_filter_dates()"); }
        
        let set_year_result = self.set_filter_year(latest_date.get_year(), Filters::DeepDive1);
        if set_year_result.is_fail() { return set_year_result.fail("Failed to initialize filter dates!", "Bank::init_filter_dates()"); }
        let set_month_result = self.set_filter_month(latest_date.get_month(), Filters::DeepDive1);
        if set_month_result.is_fail() { return set_month_result.fail("Failed to initialize filter dates!", "Bank::init_filter_dates()"); }
        
        let set_year_result = self.set_filter_year(latest_date.get_year(), Filters::DeepDive2);
        if set_year_result.is_fail() { return set_year_result.fail("Failed to initialize filter dates!", "Bank::init_filter_dates()"); }
        let set_month_result = self.set_filter_month(latest_date.get_month(), Filters::DeepDive2);
        if set_month_result.is_fail() { return set_month_result.fail("Failed to initialize filter dates!", "Bank::init_filter_dates()"); }
        
        Pass(())
    }
    
    /// Toggles the `mode` of the given `Filter`.
    #[must_use]
    pub fn toggle_filter_mode(&mut self, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.toggle_mode(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.toggle_mode(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.toggle_mode(&self.ledger),
        }
    }
    
    /// Sets the year of the given `Filter`.
    #[must_use]
    pub fn set_filter_year(&mut self, year: u32, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.set_year(year, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.set_year(year, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.set_year(year, &self.ledger),
        }
    }
    
    /// Clears the year of the given `Filter`.
    #[must_use]
    pub fn clear_filter_year(&mut self, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_year(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_year(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_year(&self.ledger),
        }
    }
    
    /// Sets the `Month` of the given `Filter`.
    #[must_use]
    pub fn set_filter_month(&mut self, month: Months, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.set_month(month, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.set_month(month, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.set_month(month, &self.ledger),
        }
    }
    
    /// Clears the `Month` of the given `Filter`.
    #[must_use]
    pub fn clear_filter_month(&mut self, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_month(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_month(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_month(&self.ledger),
        }
    }
    
    /// Adds a given `Tag` to the given `Filter`.
    #[must_use]
    pub fn add_filter_tag(&mut self, tag: &Tag, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.add_tag(tag, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.add_tag(tag, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.add_tag(tag, &self.ledger),
        }
    }
    
    /// Removes a given `Tag` from the given `Filter`.
    #[must_use]
    pub fn remove_filter_tag(&mut self, tag: &Tag, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.remove_tag(tag, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.remove_tag(tag, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.remove_tag(tag, &self.ledger),
        }
    }
    
    /// Clears all `Tag`s in the given `Filter`.
    #[must_use]
    pub fn clear_filter_tags(&mut self, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_tags(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_tags(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_tags(&self.ledger),
        }
    }
    
    /// Checks if the given `Tag` is filtered by the given `Filter`.
    #[must_use]
    pub fn is_tag_filtered(&self, tag: &Tag, filter: Filters) -> bool {
        match filter {
            Filters::Primary => self.primary_filter.is_tag_filtered(tag),
            Filters::DeepDive1 => self.deep_dive_1_filter.is_tag_filtered(tag),
            Filters::DeepDive2 => self.deep_dive_2_filter.is_tag_filtered(tag),
        }
    }
    
    /// Adds a given search term of the given `Filter`.
    #[must_use]
    pub fn add_filter_search_term(&mut self, term: &str, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.add_search_term(term, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.add_search_term(term, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.add_search_term(term, &self.ledger),
        }
    }
    
    /// Removes a given search term of the given `Filter`.
    #[must_use]
    pub fn remove_filter_search_term(&mut self, term: &str, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.remove_search_term(term, &self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.remove_search_term(term, &self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.remove_search_term(term, &self.ledger),
        }
    }
    
    /// Clears all search terms of the given `Filter`.
    #[must_use]
    pub fn clear_filter_search_terms(&mut self, filter: Filters) -> Schrod<()> {
        match filter {
            Filters::Primary => self.primary_filter.clear_search_terms(&self.ledger),
            Filters::DeepDive1 => self.deep_dive_1_filter.clear_search_terms(&self.ledger),
            Filters::DeepDive2 => self.deep_dive_2_filter.clear_search_terms(&self.ledger),
        }
    }

    /// Makes sure that all filtered `Tag`s exist.
    #[must_use]
    pub fn verify_filtered_tags(&mut self) -> Schrod<()>{
        let tags = self.get_tags();
        self.get_filter_mut(Filters::Primary).verify_filtered_tags(&tags);
        self.get_filter_mut(Filters::DeepDive1).verify_filtered_tags(&tags);
        self.get_filter_mut(Filters::DeepDive2).verify_filtered_tags(&tags);
        self.refilter()
    }
    
    /// Refilters the `Transaction`s in the three `Bank`'s `Filter`s.
    #[must_use]
    fn refilter(&mut self) -> Schrod<()> {
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




/// Defines an exchange rate for converting different `Currency`s.
/// The actual currency information is held in the `CurrencyExchange`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub rate: Decimal,
    pub date: Date, // todo: make this matter
}



/// This defines how new exchange rates are received when called for over the internet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExchangeResponse {
    rates: HashMap<String, f64>,
}



/// Holds all the exchange rates used by the `Bank` and how old they are.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyExchange {
    main_currency: String,
    pub rates: HashMap<String, HashMap<String, ExchangeRate>>,
}
impl Default for CurrencyExchange {
    fn default() -> CurrencyExchange {
        CurrencyExchange { main_currency: "USD".to_string(), rates: HashMap::new() }
    }
}
impl CurrencyExchange {
    /// Sets the main currency of the exchange.
    #[must_use]
    pub fn set_main_currency(&mut self, new_currency: String) -> Schrod<()> {
        if Transaction::is_currency_string_valid(&new_currency) {
            self.main_currency = new_currency.to_uppercase();
            Pass(())
        }
        else {
            Schrod::new_fail(&format!("Cannot find currency {new_currency}!"), "CurrencyExchange::set_main_currency()")
                .fail("Failed to set main currency.", "CurrencyExchange::set_main_currency()")
        }
    }

    /// Gets the main currency of the exchange.
    #[must_use]
    pub fn get_main_currency(&self) -> &'static Currency {
        let currency_result = Schrod::from_option(iso::find(&self.main_currency), "Failed to find main currency set in CurrencyExchange!", "CurrencyExchange::get_main_currency()");
        currency_result.wont_fail("These are guaranteed to be real currencies.", "CurrencyExchange::get_main_currency()")
    }
    
    /// Sets an exchange rate.
    #[must_use]
    pub fn set(&mut self, from: &str, to: &str, rate: Decimal) -> Schrod<()> {
        let today_result = Date::today();
        if today_result.is_fail() {
            return today_result
                .convert("CurrencyExchange::set()")
                .fail("Failed to set exchange rate!", "CurrencyExchange::set()");
        }
        
        let today = today_result.wont_fail("This is past an is_guard clause.", "CurrencyExchange::set()");
        let exchange_rate = self.rates.entry(from.to_string()).or_default();
        exchange_rate.insert(to.to_string(), ExchangeRate { rate, date: today });
        
        Pass(())
    }

    /// Gets an exchange rate from the `Bank`.
    #[must_use]
    fn get(&self, from: &str, to: &str) -> Option<ExchangeRate> {
        self.rates.get(from)?.get(to).copied()
    }

    /// Converts one `Currency` to another.
    #[must_use]
    pub fn convert(&self, value: &Decimal, from: &Currency, to: &Currency) -> Schrod<Decimal> {
        let from_str = from.to_string();
        let to_str = to.to_string();
        let rate_result = Schrod::from_option(self.get(&from_str, &to_str), &format!("Cannot find saved exchange rate on disc for {from_str} -> {to_str}"), "CurrencyExchange::convert()");
        if rate_result.is_fail() {
            return rate_result
                .convert("CurrencyExchange::convert()")
                .fail(&format!("Failed to change values from {from_str} -> {to_str}."), "CurrencyExchange::convert()");
        }

        let rate = rate_result.wont_fail("This is past an is_fail() guard clause.", "CurrencyExchange::convert()");
        Pass(value * rate.rate)
    }

    /// Fetches a new exhange rate over the internet.
    #[must_use]
    async fn fetch_rate(from: &str, to: &str) -> Schrod<f64> {
        println!("Fetching rate for {from} -> {to}");
        let url = format!("https://api.frankfurter.app/latest?from={from}&to={to}");
        println!("Parsing response");
        let response: ExchangeResponse = match reqwest::get(&url).await {
            Ok(initial_response) => {
                println!("Inital response OK");
                match initial_response.json().await {
                    Ok(json) => {
                        println!("Got json");
                        json
                    }
                    Err(_) => {
                        println!("Failed to get json");
                        return Schrod::new_fail("Failed to parse exchange rate response.", "CurrencyExchange::fetch_rate()")
                    }
                }
            }
            Err(_) => {
                println!("Failed to reach exchange rate API.");
                return Schrod::new_fail("Failed to reach exchange rate API.", "CurrencyExchange::fetch_rate()")
            }
        };
        
        println!("Got rate successfully!");
        Schrod::from_option(response.rates.get(to).copied(), &format!("Failed to fetch exchange rate for {from} -> {to}."), "CurrencyExchange::fetch_rate()")
    }

    /// Updates all exchange rates for the `Currency`s used by the `Bank` (sourced from a duplicate `ledger`).
    #[must_use]
    pub async fn update(&mut self, transactions: Vec<Transaction>) -> Schrod<()> {
        println!("Starting update");
        let mut currencies_used = Vec::new();
        println!("Collecting currencies used");
        for transaction in transactions {
            let currency = transaction.value.currency().clone();
            if !currencies_used.contains(&currency) { currencies_used.push(currency); }
        }
        
        println!("Looking at combinations");
        for from in &currencies_used {
            for to in &currencies_used {
                println!("Looking at {} -> {}", from.to_string(), to.to_string());
                if from == to { continue; }
                let from_str = from.to_string();
                let to_str = to.to_string();
                
                println!("Fething rate");
                let new_rate_f64_result = CurrencyExchange::fetch_rate(&from_str, &to_str).await;
                if new_rate_f64_result.is_fail() {
                    return new_rate_f64_result
                        .convert("CurrencyExchange::update()")
                        .fail("Failed to update exchange rates.", "CurrencyExchange::update()")
                }
                let new_rate_f64 = new_rate_f64_result.await.wont_fail("This is past an is_fail() guard clause.", "CurrencyExchange::update()");
                
                println!("Converting to Decimal");
                let new_rate_decimal_result = Schrod::from_option(Decimal::from_f64(new_rate_f64), "Failed to convert exchange rate to Decimal format!", "CurrencyExchange::update()");
                if new_rate_decimal_result.is_fail() {
                    return new_rate_decimal_result
                        .convert("CurrencyExchange::update()")
                        .fail("Failed to update exchange rates.", "CurrencyExchange::update()")
                }
                let rate = new_rate_decimal_result.wont_fail("This is past an is_fail() guard clause.", "CurrencyExchange::update()");
                
                println!("Setting");
                let set_result = self.set(&from_str, &to_str, rate);
                if set_result.is_fail() { return set_result.fail("Failed to update exchange rates.", "CurrencyExchange::update()") }
                
                println!("Success");
            }
        }

        Pass(())
    }
}



/// Holds a list of `Tag`s with their bound colors.
/// This registry holds no duplicate `Tag`s.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TagRegistry {
    /// The list of `TagRegistration`s.
    registry: Vec<TagRegistration>,
}
impl Default for TagRegistry {
    /// Creates a new default `TagRegistry` object.
    fn default() -> TagRegistry {
        TagRegistry::new()
    }
}
impl TagRegistry {
    // initializing
    /// Creates a new `TagRegistry`.
    #[must_use]
    fn new() -> TagRegistry {
        TagRegistry { registry: Vec::new() }
    }



    // management
    /// Sets a `TagRegistration`.
    /// If the `Tag` does not exist in the `registry`, a new `TagRegistration` is created.
    /// If the `Tag` does exist in the `registry`, the existing `TagRegistration` is edited.
    pub fn set(&mut self, reference_tag: &Tag, color: MaterialColors) {
        if let Some(registration) = self.get_registration_mut(reference_tag) {
            registration.edit_color(color);
        }
        else {
            self.registry.push(TagRegistration::new(reference_tag.clone(), color));
        }
    }

    /// Edits an existing `Tag` in the `registry`.
    #[must_use]
    pub fn change_tag(&mut self, reference_tag: &Tag, new_tag: &Tag) -> Schrod<()> {
        if let Some(registration) = self.get_registration_mut(reference_tag) {
            registration.edit_tag(new_tag.clone());
            Pass(())
        }
        else { Schrod::new_fail("Failed to get Tag Registration to edit!", "TagRegistry::change_tag()") }
    }

    /// Removes a `Tag` from the `registry`.
    pub fn remove(&mut self, reference_tag: &Tag) {
        self.registry.retain(|reg| &reg.tag != reference_tag);
    }
    
    /// Updates the `registry` to match the given `Tag`s, removing unnecessary `TagRegistration`s and adding unregistered `Tag`s.
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
    /// Returns a mutable reference to a `TagRegistration` if it exists, else `None`.
    #[must_use]
    pub fn get_registration_mut(&mut self, reference_tag: &Tag) -> Option<&mut TagRegistration> {
        for registration in &mut self.registry {
            if registration.tag == *reference_tag { return Some(registration) }
        }
        None
    }

    /// Returns an immutable reference to a `TagRegistration` if it exists, else `None`.
    #[must_use]
    pub fn get_registration(&self, reference_tag: &Tag) -> Option<&TagRegistration> {
        for registration in &self.registry {
            if registration.tag == *reference_tag { return Some(registration) }
        }
        None
    }

    /// Returns the color of a `Tag`.
    /// If the `Tag` does not exist, a default color is returned.
    #[must_use]
    pub fn get(&self, reference_tag: &Tag) -> MaterialColors {
        if let Some(registration) = self.get_registration(reference_tag) {
            return registration.color()
        }
        MaterialColors::Unavailable
    }

    /// Returns a list of all the `Tag`s that have a given color.
    #[must_use]
    pub fn get_tags_for_color(&self, color: MaterialColors) -> Vec<Tag> {
        let mut tags = Vec::new();
        for registration in &self.registry {
            if registration.color == color { tags.push(registration.tag.clone()) }
        }
        tags
    }
}



/// Holds a registration of a unique `Tag` with a `MaterialColor`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TagRegistration {
    /// The unique `Tag`.
    tag: Tag,
    /// The color of the `Tag`.
    color: MaterialColors,
}
impl TagRegistration {
    // initializing
    /// Creates a new `TagRegistration`.
    #[must_use]
    pub fn new(tag: Tag, color: MaterialColors) -> TagRegistration {
        TagRegistration { tag, color }
    }



    // management
    /// Edits the `Tag` of the `TagRegistration`.
    pub fn edit_tag(&mut self, new_tag: Tag) {
        self.tag = new_tag;
    }

    /// Edits the `color` of the `TagRegistration`.
    pub fn edit_color(&mut self, new_color: MaterialColors) {
        self.color = new_color;
    }



    // data retrieval and parsing
    /// Returns the `color` of the `TagRegistration`.
    #[must_use]
    pub fn color(&self) -> MaterialColors {
        self.color
    }
}