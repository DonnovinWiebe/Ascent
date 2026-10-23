use std::path::PathBuf;
use crate::{crypto::bit_wallet::BitWallet, data::{paths::path_to_file, tooling::Platforms}, standard::{bank::{CurrencyExchange, ExchangeRate, TagRegistry}, parse::FlowTypes, transaction::{Tag, Transaction}}};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schrod::Schrod::Pass;
use materialui::materials::{MaterialColors, MaterialThemes};
use schrod::Schrod;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data::paths;
/// A cute little helper function to set a default bool as explicitly false for serde deserialization.
fn default_false() -> bool { false }



pub enum SaveDataType {
    Preferences,
    BankData,
    BankDataBackup,
    SyncData,
    AuthData,
}



/// Holds all the saved information from the `App` in various forms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveData {
    Preferences {
        #[serde(default)]
        theme: MaterialThemes,
        #[serde(default = "default_false")]
        developer_mode: bool,
    },
    
    BankData {
        transactions: Vec<Transaction>,
        currency_exchange: CurrencyExchange,
        tag_registry: TagRegistry,
    },

    BankDataBackup {
        transactions: Vec<Transaction>,
        currency_exchange: CurrencyExchange,
        tag_registry: TagRegistry,
    },
    
    SyncData { revision_history: Vec<Uuid> },
    
    AuthData { temp: u32 },
}
impl SaveData {
    /// Used if there is no save data to load.
    #[must_use]
    pub fn empty(data_type: SaveDataType) -> SaveData {
        match data_type {
            SaveDataType::Preferences => SaveData::Preferences {
                theme: MaterialThemes::default(),
                developer_mode: false,
            },
            
            SaveDataType::BankData => SaveData::BankData {
                transactions: Vec::new(),
                currency_exchange: CurrencyExchange::default(),
                tag_registry: TagRegistry::default(),
            },

            SaveDataType::BankDataBackup => SaveData::BankDataBackup {
                transactions: Vec::new(),
                currency_exchange: CurrencyExchange::default(),
                tag_registry: TagRegistry::default(),
            },
            
            SaveDataType::SyncData => SaveData::SyncData { revision_history: Vec::new() },
            
            SaveDataType::AuthData => SaveData::AuthData { temp: 0 },
        }
    }

    /// Serializes the given `SaveData` into a JSON `String`.
    #[must_use]
    fn serialize(&self) -> Schrod<String> {
        // serializing
        let json_result = Schrod::from_result(serde_json::to_string_pretty(self), "Failed to serialize transaction data.", "SaveData::serialize()");
        if json_result.is_fail() {
            return json_result
                .convert("SaveData::serialize()")
                .fail("Failed to save.", "SaveData::serialize()")
        }
        let json = json_result.wont_fail("Past is_fail() guard clause.", "SaveData::serialize()");
        
        // returning the json
        Pass(json)
    }
    
    /// Saves the given save data to a JSON file at the given path.
    #[must_use]
    pub fn save(&self, platform: Platforms) -> Schrod<()> {
        // getting the json
        let json_result = self.serialize();
        if json_result.is_fail() {
            return json_result
                .convert("SaveData::save()")
                .fail("Failed to save.", "SaveData::save()")
        }
        let json = json_result.wont_fail("Past is_fail() guard clause.", "SaveData::save()");
        
        // getting the path
        let path_result = paths::path_to_file(SaveDataType::BankData, platform);
        if path_result.is_fail() {
            return path_result
                .convert("SaveData::save()")
                .fail("Failed to save.", "SaveData::save()")
        }
        let path = path_result.wont_fail("Past is_fail() guard clause.", "SaveData::backup()");

        // writing the file
        let write_result = Schrod::from_result(std::fs::write(path, json), "Failed to write save file.", "SaveData::save()");
        if write_result.is_fail() {
            return write_result
                .convert("SaveData::save()")
                .fail("Failed to save.", "SaveData::save()")
        }
    
        // returning success
        Pass(())
    }
    
    /// Saves the given save data to a JSON file at the given path.
    #[must_use]
    pub fn backup(&self, platform: Platforms) -> Schrod<()> {
        // getting the json
        let json_result = self.serialize();
        if json_result.is_fail() {
            return json_result
                .convert("SaveData::backup()")
                .fail("Failed to create backup.", "SaveData::backup()")
        }
        let json = json_result.wont_fail("Past is_fail() guard clause.", "SaveData::backup()");
        
        // writing the file
        let path_result = paths::path_to_file(SaveDataType::BankDataBackup, platform);
        if path_result.is_fail() {
            return path_result
                .convert("SaveData::backup()")
                .fail("Failed to create backup.", "SaveData::backup()")
        }
        let path = path_result.wont_fail("Past is_fail() guard clause.", "SaveData::backup()");
        let write_result = Schrod::from_result(std::fs::write(path, json), "Failed to write backup file.", "SaveData::backup()");
        if write_result.is_fail() {
            return write_result
                .convert("SaveData::backup()")
                .fail("Failed to create backup.", "SaveData::backup()")
        }
    
        // returning success
        Pass(())
    }
    
    /// Loads save data from a JSON file at the given path.
    #[must_use]
    pub fn load_from(path: &PathBuf) -> Schrod<SaveData> {
        // reading the file
        let data_result = Schrod::from_result(std::fs::read_to_string(path), "Failed to read save file.", "SaveData::load_from()");
        if data_result.is_fail() {
            return data_result
                .convert("SaveData::load_from()")
                .fail("Failed to load save data.", "SaveData::load_from()")
        }
        let data = data_result.wont_fail("Past is_fail() guard clause.", "SaveData::load_from()");
    
        // deserializing the data
        let save_data_result: Schrod<SaveData> = Schrod::from_result(serde_json::from_str(&data), "Failed to deserialize save data.", "SaveData::load_from()");
        if save_data_result.is_fail() {
            return save_data_result
                .convert("SaveData::load_from()")
                .fail("Failed to load save data.", "SaveData::load_from()")
        }
        save_data_result
    }
    
    /// Loads save data from a JSON file from a default `Path`.
    #[must_use]
    pub fn load(save_data_type: SaveDataType, platform: Platforms) -> Schrod<SaveData> {
        // getting the path
        let path_result = paths::path_to_file(save_data_type, platform);
        if path_result.is_fail() {
            return path_result
                .convert("SaveData::load()")
                .fail("Failed to load save data.", "SaveData::load()")
        }
        let path = path_result.wont_fail("Past is_fail() guard clause.", "SaveData::load()");
        
        // reading the file
        let data_result = Schrod::from_result(std::fs::read_to_string(path), "Failed to read save file.", "SaveData::load()");
        if data_result.is_fail() {
            return data_result
                .convert("SaveData::load()")
                .fail("Failed to load save data.", "SaveData::load()")
        }
        let data = data_result.wont_fail("Past is_fail() guard clause.", "SaveData::load()");
    
        // deserializing the data
        let save_data_result: Schrod<SaveData> = Schrod::from_result(serde_json::from_str(&data), "Failed to deserialize save data.", "SaveData::load()");
        if save_data_result.is_fail() {
            return save_data_result
                .convert("SaveData::load()")
                .fail("Failed to load save data.", "SaveData::load()")
        }
        save_data_result
    }
}



/// Tracks individual actions that build `SaveData` over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    /// Changes the theme.
    ChangeTheme { id: Uuid, time: DateTime<Utc>, new_theme: MaterialThemes },

    /// Changes the developer mode setting.
    ChangeDeveloperMode { id: Uuid, time: DateTime<Utc>, new_setting: bool },

    
    
    /// Adds a `Transaction`.
    AddTransaction { id: Uuid, time: DateTime<Utc>, new_transaction: Transaction },
    
    /// Edits a `Transaction`.
    EditTransaction { id: Uuid, time: DateTime<Utc>, edited_transaction: Transaction },

    /// Removes a `Transaction`.
    RemoveTransaction { id: Uuid, time: DateTime<Utc>, transaction_id: Uuid },


    
    /// Gets the main `Currency` of the `CurrencyExchange`.
    SetMainCurrency { id: Uuid, time: DateTime<Utc>, new_main_currency_string: String },
    
    /// Sets the time price from a string.
    SetTimePrice { id: Uuid, time: DateTime<Utc>, new_time_price: Decimal },
    
    /// Sets the flow type.
    SetFlowType { id: Uuid, time: DateTime<Utc>, new_flow_type: FlowTypes },
    
    /// Sets an `ExchangeRate`.
    SetExchangeRate { id: Uuid, time: DateTime<Utc>, new_exchange_rate: ExchangeRate },
    
    /// Updates all `ExchangeRate`s for the `Currency`s used by the `Bank` (sourced from a duplicate `ledger`).
    RefreshCurrencyExchange { id: Uuid, time: DateTime<Utc> },


    
    /// Sets a `TagRegistration`.
    /// If the `Tag` does not exist in the `registry`, a new `TagRegistration` is created.
    /// If the `Tag` does exist in the `registry`, the existing `TagRegistration` is edited.
    SetTagRegistration { id: Uuid, time: DateTime<Utc>, reference_tag: Tag, color: MaterialColors },
    
    /// Edits an existing `Tag` in the `registry`.
    ChangeTagRegistration { id: Uuid, time: DateTime<Utc>, old_reference_tag: Tag, new_reference_tag: Tag },
    
    /// Removes a `Tag` from the `registry`.
    RemoveTagRegistration { id: Uuid, time: DateTime<Utc>, reference_tag: Tag },
    
    /// Updates the `registry` to match the given `Tag`s, removing unnecessary `TagRegistration`s and adding unregistered `Tag`s.
    UpdateTagRegistry { id: Uuid, time: DateTime<Utc>, },
}