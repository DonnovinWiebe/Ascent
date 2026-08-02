use std::str::FromStr;

use rust_decimal::Decimal;
use rusty_money::iso;
use schrod::Schrod;
use slip44::Coin;
use Schrod::Pass;
use uuid::Uuid;

use crate::{bit_bank::bit::Bit, vault::transaction::{Date, Value}};

/// Holds a collection of cryptocurrency transactions (`Bit`s).
pub struct BitWallet {
    /// The cryptocurrency that the `BitWallet` holds.
    coin: Coin,
    /// The list of individual `Bit`s.
    ledger: Vec<Bit>,
}
impl BitWallet {
    // initializing
    /// Creates a new `BitWallet`.
    pub fn new(coin: Coin) -> BitWallet {
        BitWallet { coin, ledger: Vec::new() }
    }



    // management
    /// Gets a copy of the `ledger`.
    /// Please note that modifying these `Bit`s has no effect on the `BitWallet`'s internal `ledger`.
    #[must_use]
    pub fn get_ledger_copy(&self) -> Vec<Bit> {
        self.ledger.clone()
    }

    /// Sorts the `ledger` by `Date`.
    fn sort_ledger(&mut self) {
        self.ledger.sort_by(|a, b| b.get_date().as_value().cmp(&a.get_date().as_value()));
    }

    /// Adds a new `Bit` from concrete values.
    /// This is intended for internal use inside the `BitWallet`.
    fn add_bit_from_parts(&mut self, amount: Decimal, coin_value: Value, date: Date) {
        self.ledger.push(Bit::new(amount, coin_value, date));
        self.sort_ledger();
    }

    /// Creates a new `Bit` from raw data parts.
    /// This is intended to be used when a new `Bit` is created from within the `App`.
    #[must_use]
    pub fn add_bit_from_raw_parts(&mut self, amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date) -> Schrod<()> {
        // the amount
        let amount_result = Schrod::from_result(Decimal::from_str_exact(amount_string), "Failed to convert amount_string to Decimal!", "BitWallet::add_bit_from_raw_parts()");
        if amount_result.is_fail() {
            return amount_result
                .convert("BitWallet::add_bit_from_raw_parts()")
                .fail("Failed to add Bit from raw parts.", "BitWallet::add_bit_from_raw_parts()")
        }
        let amount = amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::add_bit_from_raw_parts()");

        // the coin value
        let coin_value_amount_result = Schrod::from_result(Decimal::from_str(coin_value_amount_string), "Failed to convert coin_value_amount_string to Decimal.", "BitWallet::add_bit_from_raw_parts()");
        if coin_value_amount_result.is_fail() {
            return coin_value_amount_result
                .convert("BitWallet::add_bit_from_raw_parts()")
                .fail("Failed to add Bit from raw parts.", "BitWallet::add_bit_from_raw_parts()")
        }
        let coin_value_amount = coin_value_amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::add_bit_from_raw_parts()");
        let coin_value_currency_result = Schrod::from_option(iso::find(&coin_value_currency_string.to_uppercase()), "Failed to convert coin_value_currency_string to a Coin.", "BitWallet::add_bit_from_raw_parts()");
        if coin_value_currency_result.is_fail() {
            return coin_value_currency_result
                .convert("BitWallet::add_bit_from_raw_parts()")
                .fail("Failed to add Bit from raw parts.", "BitWallet::add_bit_from_raw_parts()")
        }
        let coin_value_currency = coin_value_currency_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::add_bit_from_raw_parts()");
        let coin_value = Value::from_decimal(coin_value_amount, coin_value_currency);

        // adding the bit
        self.add_bit_from_parts(amount, coin_value, date);
        Pass(())
    }

    /// Edits a `Bit` with raw parts.
    #[must_use]
    pub fn edit_bit_with_raw_parts(&mut self, id: Uuid, amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date) -> Schrod<()> {
        // the amount
        let amount_result = Schrod::from_result(Decimal::from_str_exact(amount_string), "Failed to convert amount_string to Decimal!", "BitWallet::edit_bit_with_raw_parts()");
        if amount_result.is_fail() {
            return amount_result
                .convert("BitWallet::edit_bit_with_raw_parts()")
                .fail("Failed to edit Bit with raw parts.", "BitWallet::edit_bit_with_raw_parts()")
        }
        let amount = amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::edit_bit_with_raw_parts()");

        // the coin value
        let coin_value_amount_result = Schrod::from_result(Decimal::from_str(coin_value_amount_string), "Failed to convert coin_value_amount_string to Decimal.", "BitWallet::edit_bit_with_raw_parts()");
        if coin_value_amount_result.is_fail() {
            return coin_value_amount_result
                .convert("BitWallet::edit_bit_with_raw_parts()")
                .fail("Failed to edit Bit with raw parts.", "BitWallet::edit_bit_with_raw_parts()")
        }
        let coin_value_amount = coin_value_amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::edit_bit_with_raw_parts()");
        let coin_value_currency_result = Schrod::from_option(iso::find(&coin_value_currency_string.to_uppercase()), "Failed to convert coin_value_currency_string to a Coin.", "BitWallet::edit_bit_with_raw_parts()");
        if coin_value_currency_result.is_fail() {
            return coin_value_currency_result
                .convert("BitWallet::edit_bit_with_raw_parts()")
                .fail("Failed to edit Bit with raw parts.", "BitWallet::edit_bit_with_raw_parts()")
        }
        let coin_value_currency = coin_value_currency_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::edit_bit_with_raw_parts()");
        let coin_value = Value::from_decimal(coin_value_amount, coin_value_currency);

        // getting the bit
        let bit_result = self.get_mut(id);
        if bit_result.is_fail() {
            return bit_result
                .convert("BitWallet::edit_bit_with_raw_parts()")
                .fail("Failed to edit Bit with raw parts.", "BitWallet::edit_bit_with_raw_parts()")
        }
        let bit = bit_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::edit_bit_with_raw_parts()");

        // editing the bit
        bit.edit_amount(amount);
        bit.edit_coin_value(coin_value);
        bit.edit_date(date);
        self.sort_ledger();
        Pass(())
    }

    /// Removes a `Bit` from the `ledger`.
    #[must_use]
    pub fn remove_bit(&mut self, id: Uuid) -> Schrod<()> {
        for i in 0..self.ledger.len() {
            let bit = &mut self.ledger[i];
            if bit.get_id() == id {
                return Pass(())
            }
        }
        
        Schrod::new_fail("Bit could not be found!", "BitWallet::remove_bit()")
            .fail("Failed to remove Bit.", "BitWallet::remove_bit()")
    }

    
    
    // data retrieval and parsing
    /// Returns a mutable reference to the `ledger`.
    #[must_use]
    pub fn get_ledger_mut(&mut self) -> &mut Vec<Bit> { &mut self.ledger }

    /// Returns an immutable reference to the `ledger`.
    #[must_use]
    pub fn get_ledger(&self) -> &Vec<Bit> { &self.ledger }

    /// Returns an immutable reference to a `Bit`.
    #[must_use]
    pub fn get(&self, id: Uuid) -> Schrod<&Bit> {
        for bit in &self.ledger {
            if bit.get_id() == id {
                return Pass(bit);
            }
        }
        
        Schrod::new_fail("Bit could not be found!", "BitWallet::get()")
    }

    /// Returns a mutable reference to a `Bit`.
    #[must_use]
    pub fn get_mut(&mut self, id: Uuid) -> Schrod<&mut Bit> {
        for bit in &mut self.ledger {
            if bit.get_id() == id {
                return Pass(bit);
            }
        }
        
        Schrod::new_fail("Bit could not be found!", "BitWallet::get_mut()")
    }

    /// Gets the `Id`s from a list of `Bit`s.
    #[must_use]
    pub fn get_ids_from(bits: &Vec<&Bit>) -> Vec<Uuid> {
        bits.iter().map(|b| b.get_id()).collect()
    }
    
    /// Gets the `Date` of the latest `Bit` in the ledger.
    /// If the `ledger` is empty, this returns the default `Date`.
    #[must_use]
    pub fn get_latest_date(&self) -> Date {
        self.ledger.first().map(|b| b.get_date()).unwrap_or_default()
    }
    
}