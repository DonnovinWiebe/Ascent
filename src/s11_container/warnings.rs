use crate::s21_vault::bank::Bank;

/// Enumerates all possible warning messages.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Warnings {
    MainCurrencyNotUsed,
    InvalidExchangeRates,
    OldExchangeRates,
}
impl Warnings {
    /// Gets the description for the given `Warning`.
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Warnings::MainCurrencyNotUsed => "The main currency was not found in any transaction. Consider changing the main currency in Settings.".to_string(),
            Warnings::InvalidExchangeRates => "Invalid exchange rates were found. Consider setting these rates in Settings.".to_string(),
            Warnings::OldExchangeRates => "Old exchange rates were found. Consider updating these rates in Settings.".to_string(),
        }
    }

    /// Gets the icon name for the given `Warning`.
    #[must_use]
    pub fn icon_name(&self) -> String {
        match self {
            Warnings::MainCurrencyNotUsed => "circle-dollar-sign".to_string(),
            Warnings::InvalidExchangeRates => "arrow-right-arrow-left".to_string(),
            Warnings::OldExchangeRates => "clock".to_string(),
        }
    }
    
    /// Scans for any warnings in the given `Bank`.
    #[must_use]
    pub fn scan(bank: &Bank) -> Vec<Warnings> {
        // the list of warnings
        let mut warnings = Vec::new();


        
        // main currency not used
        let main_currency = bank.currency_exchange.get_main_currency();
        let mut main_currency_used = false;
        for transaction in bank.get_ledger() {
            if transaction.value.currency() == main_currency {
                main_currency_used = true;
                break;
            }
        }
        if !bank.get_ledger().is_empty() && !main_currency_used { warnings.push(Warnings::MainCurrencyNotUsed) }

        // invalid/old exchange rates
        let mut has_invalid_exchange_rates = false;
        let mut has_old_exchange_rates = false;
        for rate in bank.currency_exchange.get_rates() {
            if !rate.is_valid() { has_invalid_exchange_rates = true; }
            if !rate.is_fresh() { has_old_exchange_rates = true; }
        }
        if has_invalid_exchange_rates { warnings.push(Warnings::InvalidExchangeRates) }
        if has_old_exchange_rates { warnings.push(Warnings::OldExchangeRates) }


        
        // returning the warnings
        warnings
    }
}