use nexus::{data::{paths, save_data::{SaveData, SaveDataType}, tooling::Platforms}, standard::bank::{Bank, CurrencyExchange, TagRegistry}};
use schrod::Schrod::{self, Fail, Pass};

/// Loads `Bank` `SaveData`.
#[must_use]
pub fn load_bank() -> Schrod<Bank> {
    // getting the save data
    let bank_save_data_result = if paths::does_file_exist(SaveDataType::BankData, Platforms::Server) {
        SaveData::load(SaveDataType::BankData, Platforms::Server)
    }
    else {
        Pass(SaveData::empty(SaveDataType::BankData))
    };
    
    if bank_save_data_result.is_fail() {
        return bank_save_data_result
            .convert("init - load_bank()")
            .fail("Failed to load save data.", "init - load_bank()")
    }

    // loading the bank data
    let history;
    let transactions;
    let currency_exchange;
    let tag_registry;
    match bank_save_data_result {
        Pass(bank_data) => {
            match bank_data {
                SaveData::BankData { history: loaded_history, transactions: loaded_transactions, currency_exchange: loaded_currency_exchange, tag_registry: loaded_tag_registry } => {
                    history = loaded_history;
                    transactions = loaded_transactions;
                    currency_exchange = loaded_currency_exchange;
                    tag_registry = loaded_tag_registry;
                }
                _ => {
                    history = Vec::new();
                    transactions = Vec::new();
                    currency_exchange = CurrencyExchange::default();
                    tag_registry = TagRegistry::default();
                }
            }
        }

        Fail(_) => {
            history = Vec::new();
            transactions = Vec::new();
            currency_exchange = CurrencyExchange::default();
            tag_registry = TagRegistry::default();
        }
    }
    
    // creating the bank
    let mut bank = Bank::default();
    bank.init(history, transactions, currency_exchange, tag_registry);
    Pass(bank)
}