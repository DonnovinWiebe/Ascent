use std::path::PathBuf;
use crate::{ui::material::AppThemes, vault::{bank::TagRegistry, result_stack::ResultStack, transaction::{Date, Tag, Transaction, Value}}};
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use rust_decimal::Decimal;
use rusty_money::iso;

//====================================================================================================//
// STANDARD
//====================================================================================================//
pub struct SaveData {
    // the theme
    pub theme: AppThemes,
    // the transactions
    pub transactions: Vec<Transaction>,
    // the tag registry
    pub tag_registry: TagRegistry,
}
impl SaveData {
    /// Used if there is no save data to load.
    #[must_use]
    fn empty() -> SaveData {
        SaveData {
            theme: AppThemes::Midnight,
            transactions: Vec::new(),
            tag_registry: TagRegistry::default(),
        }
    }
}

/// Holds the various different collections of save data bundles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveDataBundles {
    // the theme
    pub theme: AppThemes,
    // the transactions
    pub transaction_bundles: Vec<TransactionDataBundle>,
    // the tag registry
    pub tag_registry: TagRegistry,
}

/// A serializable bundle of transaction data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionDataBundle {
    /// The value amount.
    value_decimal: Decimal,
    /// The currency.
    currency_string: String,
    /// The date.
    date: Date,
    /// The description.
    description: String,
    /// The tags.
    tags: Vec<Tag>,
}
impl TransactionDataBundle {
    /// Creates a new `TransactionDataBundle` from a `Transaction`.
    #[must_use]
    pub fn from_transaction(transaction: &Transaction) -> TransactionDataBundle {
        let value_decimal = *transaction.value.amount();
        let currency_string = transaction.value.currency().to_string();
        let date = transaction.date;
        let description = transaction.description.clone();
        let tags = transaction.tags.clone();
        
        TransactionDataBundle {
            value_decimal,
            currency_string,
            date,
            description,
            tags,
        }
    }
    
    /// Creates a new `Transaction` from a `TransactionDataBundle`.
    /// Please note that if this function is used, an id must be filled in later with `set_id()`.
    #[must_use]
    pub fn into_transaction(self) -> ResultStack<Transaction> {
        let currency_result = ResultStack::from_option(iso::find(&self.currency_string.to_uppercase()), "Failed to convert currency string to Currency.");
        if currency_result.is_fail() { return ResultStack::new_fail_from_stack(currency_result.get_stack()).fail("Failed to convert TransactionDataBundle into Transaction.") }
        let currency = currency_result.wont_fail("This is past an is_fail() guard clause.");
        
        let value = Value::from_decimal(self.value_decimal, currency);
        Transaction::load_from_parts(value, self.date, self.description, self.tags)
    }
}

/// Returns the `Path` to the save location and creates it if it doesn't exist.
#[must_use]
fn save_path() -> ResultStack<PathBuf> {
    // executable path
    let exe_path_result = ResultStack::from_result(std::env::current_exe(), "Failed to fetch the executable directory.");
    if exe_path_result.is_fail() { return ResultStack::new_fail_from_stack(exe_path_result.get_stack()).fail("Failed to save."); }
    let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.");
    // upstream path
    let upstream_path_result = ResultStack::from_option(exe_path.parent(), "Failed to get parent directory of the executable.");
    if upstream_path_result.is_fail() { return ResultStack::new_fail_from_stack(upstream_path_result.get_stack()).fail("Failed to save."); }
    let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.");
    // save location path
    let save_location_path = upstream_path.join("save_data");
    let location_creation_result = ResultStack::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create save data location.");
    if location_creation_result.is_fail() { return ResultStack::new_fail_from_stack(location_creation_result.get_stack()).fail("Failed to save."); }
    // save path
    let save_path = save_location_path.join("data.json");
    
    // returning the save path
    Pass(save_path)
}

/// Returns the `Path` to the backup location and creates it if it doesn't exist.
#[must_use]
pub fn backup_path() -> ResultStack<PathBuf> {
    // executable path
    let exe_path_result = ResultStack::from_result(std::env::current_exe(), "Failed to fetch the executable directory.");
    if exe_path_result.is_fail() { return ResultStack::new_fail_from_stack(exe_path_result.get_stack()).fail("Failed to create backup."); }
    let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.");
    // upstream path
    let upstream_path_result = ResultStack::from_option(exe_path.parent(), "Failed to get parent directory of the executable.");
    if upstream_path_result.is_fail() { return ResultStack::new_fail_from_stack(upstream_path_result.get_stack()).fail("Failed to create backup."); }
    let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.");
    // save location path
    let backup_location_path = upstream_path.join("backups");
    let location_creation_result = ResultStack::from_result(std::fs::create_dir_all(backup_location_path.clone()), "Failed to create backup location.");
    if location_creation_result.is_fail() { return ResultStack::new_fail_from_stack(location_creation_result.get_stack()).fail("Failed to create backup."); }
    // save path
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("backup_{}.json", timestamp);
    let export_path = backup_location_path.join(filename);
    
    // returning the save path
    Pass(export_path)
}

/// Checks if the save file exists.
#[must_use]
pub fn does_save_file_exist() -> bool {
    match save_path() {
        Pass(path) => path.exists(),
        Fail(_) => false,
    }
}

/// Serializes the given `SaveData` into a JSON `String`.
#[must_use]
fn get_serialized_save_data(save_data: SaveData) -> ResultStack<String> {
    // converting transactions into bundles
    let transaction_bundles = save_data.transactions
        .iter()
        .map(TransactionDataBundle::from_transaction)
        .collect();
    let bundles = SaveDataBundles { theme: save_data.theme, transaction_bundles, tag_registry: save_data.tag_registry };

    // serializing
    let json_result = ResultStack::from_result(serde_json::to_string_pretty(&bundles), "Failed to serialize transaction data.");
    if json_result.is_fail() { return ResultStack::new_fail_from_stack(json_result.get_stack()).fail("Failed to save."); }
    let json = json_result.wont_fail("Past is_fail() guard clause.");
    
    // returning the json
    Pass(json)
}

/// Saves the given save data to a JSON file at the given path.
#[must_use]
pub fn save(save_data: SaveData) -> ResultStack<()> {
    // getting the json
    let json_result = get_serialized_save_data(save_data);
    if json_result.is_fail() { return ResultStack::new_fail_from_stack(json_result.get_stack()).fail("Failed to save."); }
    let json = json_result.wont_fail("Past is_fail() guard clause.");
    
    // writing the file
    let save_path_result = save_path();
    if save_path_result.is_fail() { return ResultStack::new_fail_from_stack(save_path_result.get_stack()).fail("Failed to save."); }
    let save_path = save_path_result.wont_fail("Past is_fail() guard clause.");
    let write_result = ResultStack::from_result(std::fs::write(save_path, json), "Failed to write save file.");
    if write_result.is_fail() { return ResultStack::new_fail_from_stack(write_result.get_stack()).fail("Failed to save."); }

    // returning success
    Pass(())
}

/// Saves the given save data to a JSON file at the given path.
#[must_use]
pub fn backup(save_data: SaveData) -> ResultStack<()> {
    // getting the json
    let json_result = get_serialized_save_data(save_data);
    if json_result.is_fail() { return ResultStack::new_fail_from_stack(json_result.get_stack()).fail("Failed to create backup."); }
    let json = json_result.wont_fail("Past is_fail() guard clause.");
    
    // writing the file
    let backup_path_result = backup_path();
    if backup_path_result.is_fail() { return ResultStack::new_fail_from_stack(backup_path_result.get_stack()).fail("Failed to create backup."); }
    let backup_path = backup_path_result.wont_fail("Past is_fail() guard clause.");
    let write_result = ResultStack::from_result(std::fs::write(backup_path, json), "Failed to write backup file.");
    if write_result.is_fail() { return ResultStack::new_fail_from_stack(write_result.get_stack()).fail("Failed to create backup."); }

    // returning success
    Pass(())
}

/// Loads save data from a JSON file at the given path.
#[must_use]
pub fn load_from(path: &PathBuf) -> ResultStack<SaveData> {
    // reading the file
    let data_result = ResultStack::from_result(std::fs::read_to_string(path), "Failed to read save file.");
    if data_result.is_fail() { return ResultStack::new_fail_from_stack(data_result.get_stack()).fail("Failed to load save data."); }
    let data = data_result.wont_fail("Past is_fail() guard clause.");

    // deserializing the data into bundles
    let bundles_result: ResultStack<SaveDataBundles> = ResultStack::from_result(serde_json::from_str(&data), "Failed to deserialize save data.");
    if bundles_result.is_fail() { return ResultStack::new_fail_from_stack(bundles_result.get_stack()).fail("Failed to load save data."); }
    let bundles = bundles_result.wont_fail("This is past an is_fail() guard clause.");
    
    // converting transaction bundles into transactions
    let mut transactions = Vec::new();
    for bundle in bundles.transaction_bundles {
        let transaction_result = bundle.into_transaction();
        if transaction_result.is_fail() { return ResultStack::new_fail_from_stack(transaction_result.get_stack()).fail("Failed to load save data."); }
        transactions.push(transaction_result.wont_fail("This is past an is_fail() guard clause."));
    }
    
    // returning the `SaveData`
    Pass(SaveData { theme: bundles.theme, transactions, tag_registry: bundles.tag_registry })
}

/// Loads save data from a JSON file from the default `Path`.
#[must_use]
pub fn load() -> ResultStack<SaveData> {
    // returning empty save data if there is no save file
    if !does_save_file_exist() { return Pass(SaveData::empty()); }
    
    // reading the file
    let save_path_result = save_path();
    if save_path_result.is_fail() { return ResultStack::new_fail_from_stack(save_path_result.get_stack()).fail("Failed to save."); }
    let save_path = save_path_result.wont_fail("Past is_fail() guard clause.");
    
    // returning the `SaveData`
    load_from(&save_path)
}







//====================================================================================================//
// LEGACY
//====================================================================================================//
/// Allows loading of legacy data.
/// In the real world this will not be used by anyone but me since my previous projects are not publicly available.
pub mod legacy {
    use super::{ResultStack, Transaction, Decimal, iso, Value, Date, Tag, PathBuf, Pass};
    
    /// A serializable bundle of transaction data used for loading legacy transaction data.
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct LegacyTransactionDataBundle {
        /// The tags.
        #[serde(rename = "tagLine")]
        tag_line: String,
        /// The value amount.
        value: f64,
        /// The currency.
        #[serde(rename = "currencyString")]
        currency_string: String,
        /// The date (in YYYYMMDD format)
        date: u32,
        /// The description.
        note: String,
    }
    impl LegacyTransactionDataBundle {
        /// Creates a new `Transaction` from a `LegacyTransactionDataBundle`.
        /// Please note that if this function is used, an id must be filled in later with `set_id()`.
        #[must_use]
        fn into_transaction(self) -> ResultStack<Transaction> {
            // value amount
            let value_amount_result = ResultStack::from_result(Decimal::try_from(self.value), "Failed to convert f64 value to Decimal.");
            if value_amount_result.is_fail() { return ResultStack::new_fail_from_stack(value_amount_result.get_stack()).fail("Failed to convert LegacyTransactionDataBundle into Transaction.") }
            let value_amount = value_amount_result.wont_fail("This is past an is_fail() guard clause.");
            
            // currency
            let currency_result = ResultStack::from_option(iso::find(&self.currency_string.to_uppercase()), "Failed to convert currency string to Currency.");
            if currency_result.is_fail() { return ResultStack::new_fail_from_stack(currency_result.get_stack()).fail("Failed to convert LegacyTransactionDataBundle into Transaction.") }
            let currency = currency_result.wont_fail("This is past an is_fail() guard clause.");
            
            // value
            let value = Value::from_decimal(value_amount, currency);
    
            // date
            let new_date = Date::from_value(self.date);
            if new_date.is_fail() { return ResultStack::new_fail_from_stack(new_date.get_stack()).fail("Failed to convert LegacyTransactionDataBundle into Transaction.") }
            let date = new_date.wont_fail("This is past an is_fail() guard clause.");
    
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
                
                .map(|tm| { Tag::new(tm).wont_fail("This is past an is_fail() filter.") })
                
                .collect();
    
            Transaction::load_from_parts(value, date, self.note, tags)
        }
    }
    
    /// Loads legacy `Transaction`s from a JSON file at the given `Path`.
    #[must_use]
    pub fn load_legacy_from(path: &PathBuf) -> ResultStack<Vec<Transaction>> {
        // reading the file
        let data_result = ResultStack::from_result(std::fs::read_to_string(path), "Failed to read legacy save file.");
        if data_result.is_fail() { return ResultStack::new_fail_from_stack(data_result.get_stack()).fail("Failed to load legacy transactions."); }
        let data = data_result.wont_fail("Past is_fail() guard clause.");
    
        // deserializing the data into bundles
        let bundles_result: ResultStack<Vec<LegacyTransactionDataBundle>> = ResultStack::from_result(serde_json::from_str(&data), "Failed to deserialize legacy transaction data.");
        if bundles_result.is_fail() { return ResultStack::new_fail_from_stack(bundles_result.get_stack()).fail("Failed to load legacy transactions."); }
        let bundles = bundles_result.wont_fail("This is past an is_fail() guard clause.");
        
        // converting bundles into transactions
        let mut transactions = Vec::new();
        for bundle in bundles {
            let transaction_result = bundle.into_transaction();
            if transaction_result.is_fail() { return ResultStack::new_fail_from_stack(transaction_result.get_stack()).fail("Failed to load legacy transactions."); }
            transactions.push(transaction_result.wont_fail("This is past an is_fail() guard clause."));
        }
        
        // returning the transactions
        Pass(transactions)
    }
}