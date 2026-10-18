use std::path::PathBuf;
use crate::{crypto::bit_wallet::BitWallet, data::tooling::Platforms, standard::{bank::{CurrencyExchange, TagRegistry}, transaction::Transaction}};
use schrod::Schrod::Pass;
use materialui::materials::MaterialThemes;
use schrod::Schrod;
use serde::{Deserialize, Serialize};
use crate::data::paths;
/// A cute little helper function to set a default bool as explicitly false for serde deserialization.
fn default_false() -> bool { false }



/// Holds all the saved information from the `App`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    #[serde(default)]
    pub theme: MaterialThemes,
    #[serde(default = "default_false")]
    pub developer_mode: bool,
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
            theme: MaterialThemes::Peach,
            developer_mode: false,
            transactions: Vec::new(),
            tag_registry: TagRegistry::default(),
            currency_exchange: CurrencyExchange::default(),
            bit_wallets: Vec::new(),
        }
    }

    /// Serializes the given `SaveData` into a JSON `String`.
    #[must_use]
    fn serialize(&self) -> Schrod<String> {
        // serializing
        let json_result = Schrod::from_result(serde_json::to_string_pretty(self), "Failed to serialize transaction data.", "SaveData::get_serialized_save_data()");
        if json_result.is_fail() {
            return json_result
                .convert("SaveData::get_serialized_save_data()")
                .fail("Failed to save.", "SaveData::get_serialized_save_data()")
        }
        let json = json_result.wont_fail("Past is_fail() guard clause.", "SaveData::get_serialized_save_data()");
        
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
        
        // writing the file
        let path_result = paths::save_path(platform);
        if path_result.is_fail() {
            return path_result
                .convert("SaveData::save()")
                .fail("Failed to save.", "SaveData::save()")
        }
        let path = path_result.wont_fail("Past is_fail() guard clause.", "SaveData::backup()");
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
        let path_result = paths::backup_path(platform);
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
    
        // deserializing the data into bundles
        let save_data_result: Schrod<SaveData> = Schrod::from_result(serde_json::from_str(&data), "Failed to deserialize save data.", "SaveData::load_from()");
        if save_data_result.is_fail() {
            return save_data_result
                .convert("SaveData::load_from()")
                .fail("Failed to load save data.", "SaveData::load_from()")
        }
        let save_data = save_data_result.wont_fail("This is past an is_fail() guard clause.", "SaveData::load_from()");
        
        // returning the `SaveData`
        Pass(SaveData { theme: save_data.theme, developer_mode: save_data.developer_mode, transactions: save_data.transactions, currency_exchange: save_data.currency_exchange, tag_registry: save_data.tag_registry, bit_wallets: save_data.bit_wallets })
    }
    
    /// Loads save data from a JSON file from the default `Path`.
    #[must_use]
    pub fn load(platform: Platforms) -> Schrod<SaveData> {
        // returning empty save data if there is no save file
        if !paths::does_save_file_exist(platform) { return Pass(SaveData::empty()); }
        
        // reading the file
        let save_path_result = paths::save_path(platform);
        if save_path_result.is_fail() {
            return save_path_result
                .convert("SaveData::load()")
                .fail("Failed to save.", "SaveData::load()")
        }
        let save_path = save_path_result.wont_fail("Past is_fail() guard clause.", "SaveData::load()");
        
        // returning the `SaveData`
        SaveData::load_from(&save_path)
    }
}


/*
/// Tracks individual actions that build `SaveData` over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Action {
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
*/