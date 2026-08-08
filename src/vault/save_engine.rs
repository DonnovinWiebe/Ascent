#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
use std::{io::{Error, ErrorKind, Result}, process::Child};
use std::path::PathBuf;
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
use std::process::Command;
use crate::{bit_vault::bit_wallet::BitWallet, vault::{bank::{CurrencyExchange, TagRegistry}, transaction::{Date, Tag, Transaction, Value}}};
use schrod::Schrod::{Pass, Fail};
use materialui::materials::MaterialThemes;
use rust_decimal::Decimal;
use rusty_money::iso;
use schrod::Schrod;
use serde::{Deserialize, Serialize};

//====================================================================================================//
// STANDARD
//====================================================================================================//
/// Implements `serde::serialize` and `serde::deserialize` for `Coin`.
pub mod coin_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use slip44::Coin;
    use std::convert::TryFrom;

    /// Serializes a `Coin`.
    pub fn serialize<S: Serializer>(coin: &Coin, s: S) -> Result<S::Ok, S::Error> {
        coin.id().serialize(s)
    }

    /// Deserializes a `Coin`.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Coin, D::Error> {
        let id = u32::deserialize(d)?;
        Coin::try_from(id).map_err(serde::de::Error::custom)
    }
}

// This is used in Bit, but could in theory be used for Transactions. I have not attempted
// the change since it could corrupt all existing save data.
/// Implements `serde::serialize` and `serde::deserialize` for `Value` (`Money`).
pub mod value_serde {
    use rust_decimal::Decimal;
    use rusty_money::iso;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use crate::vault::transaction::Value;

    /// Holds the data contained in a `Value` in a serializable format.
    #[derive(Serialize, Deserialize)]
    struct ValueMiddle {
        amount: Decimal,
        currency_string: String,
    }

    /// Serializes a `Value`.
    pub fn serialize<S: Serializer>(value: &Value, s: S) -> Result<S::Ok, S::Error> {
        let amount = *value.amount();
        let currency_string = value.currency().iso_alpha_code.to_string();
        let middle = ValueMiddle { amount, currency_string };
        middle.serialize(s)
    }

    /// Deserializes a `Value`.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Value, D::Error> {
        let middle = ValueMiddle::deserialize(d)?;
        let currency = iso::find(&middle.currency_string)
            .ok_or_else(|| serde::de::Error::custom("unknown currency code"))?;
        Ok(Value::from_decimal(middle.amount, currency))
    }
}



#[derive(Clone)]
pub struct SaveData {
    pub theme: MaterialThemes,
    pub transactions: Vec<Transaction>,
    pub currency_exchange: CurrencyExchange,
    pub tag_registry: TagRegistry,
    pub bit_wallets: Vec<BitWallet>,
}
impl SaveData {
    /// Used if there is no save data to load.
    #[must_use]
    fn empty() -> SaveData {
        SaveData {
            theme: MaterialThemes::Midnight,
            transactions: Vec::new(),
            tag_registry: TagRegistry::default(),
            currency_exchange: CurrencyExchange::default(),
            bit_wallets: Vec::new(),
        }
    }
}

/// Holds the various pieces of data used in `SaveData` in a serializable format.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveDataBundle {
    theme: MaterialThemes,
    transaction_bundles: Vec<TransactionDataBundle>,
    #[serde(default)]
    currency_exchange: CurrencyExchange,
    tag_registry: TagRegistry,
    #[serde(default)]
    bit_wallets: Vec<BitWallet>,
}

/// A serializable bundle of transaction data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionDataBundle {
    value_decimal: Decimal,
    currency_string: String,
    date: Date,
    description: String,
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
    pub fn into_transaction(self) -> Schrod<Transaction> {
        let currency_result = Schrod::from_option(iso::find(&self.currency_string.to_uppercase()), "Failed to convert currency string to Currency.", "TransactionDataBundle::into_transaction()");
        if currency_result.is_fail() {
            return currency_result
                .convert("TransactionDataBundle::into_transaction()")
                .fail("Failed to convert TransactionDataBundle into Transaction.", "TransactionDataBundle::into_transaction()")
        }
        let currency = currency_result.wont_fail("This is past an is_fail() guard clause.", "TransactionDataBundle::into_transaction()");
        
        let value = Value::from_decimal(self.value_decimal, currency);
        Transaction::load_from_parts(value, self.date, self.description, self.tags)
    }
}

/// Returns the `Path` to the save location and creates it if it doesn't exist.
#[must_use]
fn save_path() -> Schrod<PathBuf> {
    // executable path
    let exe_path_result = Schrod::from_result(std::env::current_exe(), "Failed to fetch the executable directory.", "save_engine::save_path()");
    if exe_path_result.is_fail() {
        return exe_path_result
            .convert("save_engine::save_path()")
            .fail("Failed to save.", "save_engine::save_path()")
    }
    let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::save_path()");

    // modifies the path when running as a .app bundle in MacOS
    #[cfg(target_os = "macos")]
    {
        if is_running_as_app_bundle(&exe_path) {
            // the os based execution path
            let base_result = Schrod::from_option(dirs::data_dir(), "Failed to fetch the OS data directory.", "save_engine::save_path()");
            if base_result.is_fail() {
                return base_result
                    .convert("save_engine::save_path()")
                    .fail("Failed to save.", "save_engine::save_path()")
            }
            let base = base_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::save_path()");

            // joins the base os path to an app specific folder
            let save_location_path = base.join("Ascent").join("save_data");
            let location_creation_result = Schrod::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create save data location.", "save_engine::save_path()");
            if location_creation_result.is_fail() {
                return location_creation_result
                    .convert("save_engine::save_path()")
                    .fail("Failed to save.", "save_engine::save_path()")
            }

            // returns the final save data path
            return Pass(save_location_path.join("data.json"));
        }
    }
        
    // upstream path
    let upstream_path_result = Schrod::from_option(exe_path.parent(), "Failed to get parent directory of the executable.", "save_engine::save_path()");
    if upstream_path_result.is_fail() {
        return upstream_path_result
            .convert("save_engine::save_path()")
            .fail("Failed to save.", "save_engine::save_path()")
    }
    let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::save_path()");
    
    // save location path
    let save_location_path = upstream_path.join("save_data");
    let location_creation_result = Schrod::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create save data location.", "save_engine::save_path()");
    if location_creation_result.is_fail() {
        return location_creation_result
            .convert("save_engine::save_path()")
            .fail("Failed to save.", "save_engine::save_path()")
    }
    
    // save path
    let save_path = save_location_path.join("data.json");
    
    // returning the save path
    Pass(save_path)
}

/// Returns the `Path` to the backup location and creates it if it doesn't exist.
#[must_use]
pub fn backup_path() -> Schrod<PathBuf> {
    // executable path
    let exe_path_result = Schrod::from_result(std::env::current_exe(), "Failed to fetch the executable directory.", "save_engine::backup_path()");
    if exe_path_result.is_fail() {
        return exe_path_result
            .convert("save_engine::backup_path()")
            .fail("Failed to create backup.", "save_engine::backup_path()")
    }
    let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::backup_path()");
    
    // modifies the path when running as a .app bundle in MacOS
    #[cfg(target_os = "macos")]
    {
        if is_running_as_app_bundle(&exe_path) {
            // the os based execution path
            let base_result = Schrod::from_option(dirs::data_dir(), "Failed to fetch the OS data directory.", "save_engine::backup_path()");
            if base_result.is_fail() {
                return base_result
                    .convert("save_engine::backup_path()")
                    .fail("Failed to create backup.", "save_engine::backup_path()")
            }
            let base = base_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::backup_path()");

            // joins the base os path to an app specific folder
            let backup_location_path = base.join("Ascent").join("backups");
            let location_creation_result = Schrod::from_result(std::fs::create_dir_all(backup_location_path.clone()), "Failed to create backup location.", "save_engine::backup_path()");
            if location_creation_result.is_fail() {
                return location_creation_result
                    .convert("save_engine::backup_path()")
                    .fail("Failed to create backup.", "save_engine::backup_path()")
            }

            // save path
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
            let filename = format!("backup_{timestamp}.json");
            let export_path = backup_location_path.join(filename);
            
            // returns the final save path
            return Pass(export_path);
        }
    }
    
    // upstream path
    let upstream_path_result = Schrod::from_option(exe_path.parent(), "Failed to get parent directory of the executable.", "save_engine::backup_path()");
    if upstream_path_result.is_fail() {
        return upstream_path_result
            .convert("save_engine::backup_path()")
            .fail("Failed to create backup.", "save_engine::backup_path()")
    }
    let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::backup_path()");
    
    // save location path
    let backup_location_path = upstream_path.join("backups");
    let location_creation_result = Schrod::from_result(std::fs::create_dir_all(backup_location_path.clone()), "Failed to create backup location.", "save_engine::backup_path()");
    if location_creation_result.is_fail() {
        return location_creation_result
            .convert("save_engine::backup_path()")
            .fail("Failed to create backup.", "save_engine::backup_path()");
    }

    // save path
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("backup_{timestamp}.json");
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

/// Checks if the app is running from inside a macOS .app bundle
#[must_use]
#[cfg(target_os = "macos")]
fn is_running_as_app_bundle(exe_path: &std::path::Path) -> bool {
    exe_path.ancestors().any(|p| {
        p.extension().is_some_and(|ext| ext == "app")
    })
}

/// Serializes the given `SaveData` into a JSON `String`.
#[must_use]
fn get_serialized_save_data(save_data: SaveData) -> Schrod<String> {
    // converting transactions into bundles
    let transaction_bundles = save_data.transactions
        .iter()
        .map(TransactionDataBundle::from_transaction)
        .collect();
    
    let bundles = SaveDataBundle {
        theme: save_data.theme,
        transaction_bundles,
        currency_exchange: save_data.currency_exchange,
        tag_registry: save_data.tag_registry,
        bit_wallets: save_data.bit_wallets,
    };

    // serializing
    let json_result = Schrod::from_result(serde_json::to_string_pretty(&bundles), "Failed to serialize transaction data.", "save_engine::get_serialized_save_data()");
    if json_result.is_fail() {
        return json_result
            .convert("save_engine::get_serialized_save_data()")
            .fail("Failed to save.", "save_engine::get_serialized_save_data()")
    }
    let json = json_result.wont_fail("Past is_fail() guard clause.", "save_engine::get_serialized_save_data()");
    
    // returning the json
    Pass(json)
}

/// Saves the given save data to a JSON file at the given path.
#[must_use]
pub fn save(save_data: SaveData) -> Schrod<()> {
    // getting the json
    let json_result = get_serialized_save_data(save_data);
    if json_result.is_fail() {
        return json_result
            .convert("save_engine::save()")
            .fail("Failed to save.", "save_engine::save()")
    }
    let json = json_result.wont_fail("Past is_fail() guard clause.", "save_engine::save()");
    
    // writing the file
    let save_path_result = save_path();
    if save_path_result.is_fail() {
        return save_path_result
            .convert("save_engine::save()")
            .fail("Failed to save.", "save_engine::save()")
    }
    let save_path = save_path_result.wont_fail("Past is_fail() guard clause.", "save_engine::save()");
    let write_result = Schrod::from_result(std::fs::write(save_path, json), "Failed to write save file.", "save_engine::save()");
    if write_result.is_fail() {
        return write_result
            .convert("save_engine::save()")
            .fail("Failed to save.", "save_engine::save()")
    }

    // returning success
    Pass(())
}

/// Saves the given save data to a JSON file at the given path.
#[must_use]
pub fn backup(save_data: SaveData) -> Schrod<()> {
    // getting the json
    let json_result = get_serialized_save_data(save_data);
    if json_result.is_fail() {
        return json_result
            .convert("save_engine::backup()")
            .fail("Failed to create backup.", "save_engine::backup()")
    }
    let json = json_result.wont_fail("Past is_fail() guard clause.", "save_engine::backup()");
    
    // writing the file
    let backup_path_result = backup_path();
    if backup_path_result.is_fail() {
        return backup_path_result
            .convert("save_engine::backup()")
            .fail("Failed to create backup.", "save_engine::backup()")
    }
    let backup_path = backup_path_result.wont_fail("Past is_fail() guard clause.", "save_engine::backup()");
    let write_result = Schrod::from_result(std::fs::write(backup_path, json), "Failed to write backup file.", "save_engine::backup()");
    if write_result.is_fail() {
        return write_result
            .convert("save_engine::backup()")
            .fail("Failed to create backup.", "save_engine::backup()")
    }

    // returning success
    Pass(())
}

/// Loads save data from a JSON file at the given path.
#[must_use]
pub fn load_from(path: &PathBuf) -> Schrod<SaveData> {
    // reading the file
    let data_result = Schrod::from_result(std::fs::read_to_string(path), "Failed to read save file.", "save_engine::load_from()");
    if data_result.is_fail() {
        return data_result
            .convert("save_engine::load_from()")
            .fail("Failed to load save data.", "save_engine::load_from()")
    }
    let data = data_result.wont_fail("Past is_fail() guard clause.", "save_engine::load_from()");

    // deserializing the data into bundles
    let bundle_result: Schrod<SaveDataBundle> = Schrod::from_result(serde_json::from_str(&data), "Failed to deserialize save data.", "save_engine::load_from()");
    if bundle_result.is_fail() {
        return bundle_result
            .convert("save_engine::load_from()")
            .fail("Failed to load save data.", "save_engine::load_from()")
    }
    let bundle = bundle_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::load_from()");
    
    // converting transaction bundles into transactions
    let mut transactions = Vec::new();
    for transaction_bundle in bundle.transaction_bundles {
        let transaction_result = transaction_bundle.into_transaction();
        if transaction_result.is_fail() {
            return transaction_result
                .convert("save_engine::load_from()")
                .fail("Failed to load save data.", "save_engine::load_from()")
        }
        transactions.push(transaction_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::load_from()"));
    }
    
    // returning the `SaveData`
    Pass(SaveData { theme: bundle.theme, transactions, currency_exchange: bundle.currency_exchange, tag_registry: bundle.tag_registry, bit_wallets: bundle.bit_wallets })
}

/// Loads save data from a JSON file from the default `Path`.
#[must_use]
pub fn load() -> Schrod<SaveData> {
    // returning empty save data if there is no save file
    if !does_save_file_exist() { return Pass(SaveData::empty()); }
    
    // reading the file
    let save_path_result = save_path();
    if save_path_result.is_fail() {
        return save_path_result
            .convert("save_engine::load()")
            .fail("Failed to save.", "save_engine::load()")
    }
    let save_path = save_path_result.wont_fail("Past is_fail() guard clause.", "save_engine::load()");
    
    // returning the `SaveData`
    load_from(&save_path)
}

/// Opens a system file explorer at the `App`'s data directory.
#[must_use]
pub fn open_data_location_file_explorer() -> Schrod<()> {
    // gets the save data file path to work with
    let save_path_result = save_path();
    if save_path_result.is_fail() {
        return save_path_result
            .convert("save_engine::open_data_location_window()")
            .fail("Failed to open data location.", "save_engine::open_data_location_window()")
    }
    let save_path = save_path_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::open_data_location_window()");

    // gets the save folder that holds all data
    let save_folder_result = Schrod::from_option(save_path.parent(), "Failed to get parent directory of the save data.", "save_engine::reveal_save_location()");
    if save_folder_result.is_fail() {
        return save_folder_result
            .convert("save_engine::open_data_location_window()")
            .fail("Failed to open data location.", "save_engine::open_data_location_window()")
    }
    let save_folder = save_folder_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::open_data_location_window()");
    
    // gets the parent folder that holds all data
    let home_directory_result = Schrod::from_option(save_folder.parent(), "Failed to get the data directory.", "save_engine::reveal_save_location()");
    if home_directory_result.is_fail() {
        return home_directory_result
            .convert("save_engine::open_data_location_window()")
            .fail("Failed to open data location.", "save_engine::open_data_location_window()")
    }
    let home_directory = home_directory_result.wont_fail("This is past an is_fail() guard clause.", "save_engine::open_data_location_window()");

    // opens the file explorer
    #[cfg(target_os = "macos")]
    let spawn_result = Command::new("open").arg(home_directory).spawn();
    #[cfg(target_os = "linux")]
    let spawn_result = Command::new("xdg-open").arg(home_directory).spawn();
    #[cfg(target_os = "windows")]
    let spawn_result = Command::new("explorer").arg(home_directory).spawn();

    // fails if this is (somehow) running on a different os
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let spawn_result: Resultesult<Child> =
        Err(Error::new(ErrorKind::Unsupported, "Unsupported platform for opening file location"));

    // returns the result
    let result = Schrod::from_result(spawn_result, "Failed to open file explorer window.", "save_engine::open_data_location_window()");
    match result {
        Pass(_) => Pass(()),
        Fail(_) => {
            result
                .convert("save_engine::open_data_location_window()")
                .fail("Failed to open data location.", "save_engine::open_data_location_window()")
        }
    }
}







//====================================================================================================//
// LEGACY
//====================================================================================================//
/// Allows loading of legacy data.
/// In the real world this will not be used by anyone but me since my previous projects are not publicly available.
pub mod legacy {
    use super::{Schrod, Transaction, Decimal, iso, Value, Date, Tag, PathBuf, Pass};
    
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
    
            Transaction::load_from_parts(value, date, self.note, tags)
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
}