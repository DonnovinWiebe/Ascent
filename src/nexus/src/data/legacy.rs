/// Allows loading of legacy data.
/// In the real world this will not be used by anyone but me since my previous projects are not publicly available.
use std::path::PathBuf;
use rust_decimal::Decimal;
use rusty_money::iso;
use schrod::Schrod::{self, Pass};
use uuid::Uuid;
use crate::standard::transaction::{Date, Tag, Transaction, Value};

/// A serializable bundle of transaction data used for loading legacy transaction data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct LegacyTransactionDataBundle {
    #[serde(rename = "tagLine")]
    tag_line: String,
    value: f64,
    #[serde(rename = "currencyString")]
    currency_string: String,
    date: u32,
    note: String,
}
impl LegacyTransactionDataBundle {
    /// Creates a new `Transaction` from a `LegacyTransactionDataBundle`.
    /// Please note that if this function is used, an id must be filled in later with `set_id()`.
    #[must_use]
    fn into_transaction(self) -> Schrod<Transaction> {
        // value amount
        let value_amount_result = Schrod::from_result(Decimal::try_from(self.value), "Failed to convert f64 value to Decimal.", "LegacyTransactionDataBundle::into_transaction()");
        if value_amount_result.is_fail() {
            return value_amount_result
                .convert("LegacyTransactionDataBundle::into_transaction()")
                .fail("Failed to convert LegacyTransactionDataBundle into Transaction.", "LegacyTransactionDataBundle::into_transaction()")
        }
        let value_amount = value_amount_result.wont_fail("This is past an is_fail() guard clause.", "LegacyTransactionDataBundle::into_transaction()");
        
        // currency
        let currency_result = Schrod::from_option(iso::find(&self.currency_string.to_uppercase()), "Failed to convert currency string to Currency.", "LegacyTransactionDataBundle::into_transaction()");
        if currency_result.is_fail() {
            return currency_result
                .convert("LegacyTransactionDataBundle::into_transaction()")
                .fail("Failed to convert LegacyTransactionDataBundle into Transaction.", "LegacyTransactionDataBundle::into_transaction()")
        }
        let currency = currency_result.wont_fail("This is past an is_fail() guard clause.", "LegacyTransactionDataBundle::into_transaction()");
        
        // value
        let value = Value::from_decimal(value_amount, currency);

        // date
        let new_date = Date::from_value(self.date);
        if new_date.is_fail() {
            return new_date
                .convert("LegacyTransactionDataBundle::into_transaction()")
                .fail("Failed to convert LegacyTransactionDataBundle into Transaction.", "LegacyTransactionDataBundle::into_transaction()")
        }
        let date = new_date.wont_fail("This is past an is_fail() guard clause.", "LegacyTransactionDataBundle::into_transaction()");

        // tags
        let mut tag_conversion_failures = Vec::new();
        
        let tags: Vec<Tag> = self.tag_line
            .split('|')
            
            .filter(|tf| {
                let tag_result = Tag::new(tf);
                if tag_result.is_fail() {
                    tag_conversion_failures.push(tag_result);
                    false
                }
                else { true }
            })
            
            .map(|tm| { Tag::new(tm).wont_fail("This is past an is_fail() filter.", "LegacyTransactionDataBundle::into_transaction()") })
            
            .collect();

        Transaction::load_from_parts(Uuid::new_v4(), value, date, self.note, tags)
    }
}

/// Loads legacy `Transaction`s from a JSON file at the given `Path`.
#[must_use]
pub fn load_legacy_from(path: &PathBuf) -> Schrod<Vec<Transaction>> {
    // reading the file
    let data_result = Schrod::from_result(std::fs::read_to_string(path), "Failed to read legacy save file.", "LegacyTransactionDataBundle::load_legacy_from()");
    if data_result.is_fail() {
        return data_result
            .convert("LegacyTransactionDataBundle::load_legacy_from()")
            .fail("Failed to load legacy transactions.", "LegacyTransactionDataBundle::load_legacy_from()")
    }
    let data = data_result.wont_fail("Past is_fail() guard clause.", "LegacyTransactionDataBundle::load_legacy_from()");

    // deserializing the data into bundles
    let bundles_result: Schrod<Vec<LegacyTransactionDataBundle>> = Schrod::from_result(serde_json::from_str(&data), "Failed to deserialize legacy transaction data.", "LegacyTransactionDataBundle::load_legacy_from()");
    if bundles_result.is_fail() {
        return bundles_result
            .convert("LegacyTransactionDataBundle::load_legacy_from()")
            .fail("Failed to load legacy transactions.", "LegacyTransactionDataBundle::load_legacy_from()")
    }
    let bundles = bundles_result.wont_fail("This is past an is_fail() guard clause.", "LegacyTransactionDataBundle::load_legacy_from()");
    
    // converting bundles into transactions
    let mut transactions = Vec::new();
    for bundle in bundles {
        let transaction_result = bundle.into_transaction();
        if transaction_result.is_fail() {
            return transaction_result
                .convert("LegacyTransactionDataBundle::load_legacy_from()")
                .fail("Failed to load legacy transactions.", "LegacyTransactionDataBundle::load_legacy_from()")
        }
        transactions.push(transaction_result.wont_fail("This is past an is_fail() guard clause.", "LegacyTransactionDataBundle::load_legacy_from()"));
    }
    
    // returning the transactions
    Pass(transactions)
}