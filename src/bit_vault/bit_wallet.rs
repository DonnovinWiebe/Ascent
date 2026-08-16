use std::str::FromStr;
use rust_decimal::Decimal;
use rusty_money::iso;
use schrod::Schrod;
use serde::{Deserialize, Serialize};
use slip44::Coin;
use Schrod::Pass;
use uuid::Uuid;
use crate::{bit_vault::bit::{Bit, BitTypes}, vault::transaction::{Date, Value}};
use crate::container::save_engine::coin_serde;

/// Holds a collection of cryptocurrency transactions (`Bit`s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitWallet {
    /// The id of the `BitWallet`.
    id: Uuid,
    /// The name of this particular `BitWallet`.
    name: String,
    /// The cryptocurrency that the `BitWallet` holds.
    #[serde(with = "coin_serde")]
    coin: Coin,
    /// The list of individual `Bit`s.
    ledger: Vec<Bit>,
}
impl BitWallet {
    // initializing
    /// Creates a new `BitWallet`.
    /// This is intended for internal use inside the `BitWallet`.
    fn new_from_parts(name: &str, coin: Coin) -> BitWallet {
        BitWallet { id: Uuid::new_v4(), name: name.to_string(), coin, ledger: Vec::new() }
    }

    /// Creates a new `BitWallet`.
    /// This is intended to be used when a new `BitWallet` is created from within the `App`.
    #[must_use]
    pub fn new_from_raw_parts(name: &str, coin_string: &str) -> Schrod<BitWallet> {
        // getting the coin
        let coin_result = Schrod::from_result(Coin::from_str(&coin_string.to_uppercase()), "Failed to get Coin from coin_string!", "BitWallet::new_from_raw_parts");
        if coin_result.is_fail() {
            return coin_result
                .convert("BitWallet::new_from_raw_parts")
                .fail("Failed to create BitWallet from raw parts.", "BitWallet::new_from_raw_parts")
        }
        let coin = coin_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::new_from_raw_parts");

        // returning the result
        if BitWallet::are_raw_parts_valid(name, coin_string) { Pass(BitWallet::new_from_parts(name, coin)) }
        else { Schrod::new_fail("Failed to create BitWallet from raw parts.", "BitWallet::new_from_raw_parts") }
    }



    // validation
    /// Checks if a `BitWallet` can be created from the given raw parts.
    #[must_use]
    pub fn are_raw_parts_valid(name: &str, coin_string: &str) -> bool {
        BitWallet::is_name_valid(name) && BitWallet::can_parse_as_coin(coin_string)
    }

    /// Checks if a given `String` is a valid `name`.
    #[must_use]
    pub fn is_name_valid(name: &str) -> bool {
        !name.trim().is_empty()
    }
    
    /// Checks if a given `String` can be parsed into a `Coin`.
    #[must_use]
    pub fn can_parse_as_coin(coin_string: &str) -> bool {
        Coin::from_str(&coin_string.to_uppercase()).is_ok()
    }



    // management
    /// Edits the `name` of the `BitWallet`.
    pub fn edit_name(&mut self, new_name: &str) -> Schrod<()> {
        if BitWallet::is_name_valid(new_name) {
            self.name = new_name.to_string();
            Pass(())
        }
        else {
            Schrod::new_fail("Invalid name!", "BitWallet::edit_name()")
                .fail("Failed to edit name.", "BitWallet::edit_name()")
        }
    }

    /// Edits the `coin` of the `BitWallet`.
    #[must_use]
    pub fn edit_coin(&mut self, new_coin_string: &str) -> Schrod<()> {
        let coin_result = Schrod::from_result(Coin::from_str(&new_coin_string.to_uppercase()), "Failed to convert new_coin_string to Coin!", "BitWallet::edit_coin()");
        if coin_result.is_fail() {
            return coin_result
                .convert("BitWallet::edit_coin()")
                .fail("Failed to edit coin.", "BitWallet::edit_coin()")
        }
        self.coin = coin_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::edit_coin()");
        Pass(())
    }
    
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
    fn add_bit_from_parts(&mut self, amount: Decimal, coin_value: Value, date: Date, bit_type: BitTypes) {
        self.ledger.push(Bit::new(amount, coin_value, date, bit_type));
        self.sort_ledger();
    }

    /// Creates a new `Bit` from raw data parts.
    /// This is intended to be used when a new `Bit` is created from within the `App`.
    #[must_use]
    pub fn add_bit_from_raw_parts(&mut self, amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date, bit_type: BitTypes) -> Schrod<()> {
        // This mirrors the checks in Bit::are_raw_parts_valid(). I may be able to save code
        // instead of reimplementing this 3 times, but at least for now it's ok with me.
        
        // the amount
        let amount_result = Schrod::from_result(Decimal::from_str_exact(amount_string), "Failed to convert amount_string to Decimal!", "BitWallet::add_bit_from_raw_parts()");
        if amount_result.is_fail() {
            return amount_result
                .convert("BitWallet::add_bit_from_raw_parts()")
                .fail("Failed to add Bit from raw parts.", "BitWallet::add_bit_from_raw_parts()")
        }
        let amount = amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::add_bit_from_raw_parts()");
        if amount <= Decimal::ZERO {
            return Schrod::new_fail("Amount cannot be less than zero!", "BitWallet::add_bit_from_raw_parts()")
                .fail("Failed to add Bit from raw parts.", "BitWallet::add_bit_from_raw_parts()")
        }

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
        self.add_bit_from_parts(amount, coin_value, date, bit_type);
        Pass(())
    }

    /// Edits a `Bit` with raw parts.
    #[must_use]
    pub fn edit_bit_with_raw_parts(&mut self, id: Uuid, amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date, bit_type: BitTypes) -> Schrod<()> {
        // This mirrors the checks in Bit::are_raw_parts_valid(). I may be able to save code
        // instead of reimplementing this 3 times, but at least for now it's ok with me.
        
        // the amount
        let amount_result = Schrod::from_result(Decimal::from_str_exact(amount_string), "Failed to convert amount_string to Decimal!", "BitWallet::edit_bit_with_raw_parts()");
        if amount_result.is_fail() {
            return amount_result
                .convert("BitWallet::edit_bit_with_raw_parts()")
                .fail("Failed to edit Bit with raw parts.", "BitWallet::edit_bit_with_raw_parts()")
        }
        let amount = amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::edit_bit_with_raw_parts()");
        if amount <= Decimal::ZERO {
            return Schrod::new_fail("Amount cannot be zero!", "BitWallet::edit_bit_with_raw_parts()")
                .fail("Failed to add Bit from raw parts.", "BitWallet::edit_bit_with_raw_parts()")
        }

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
        bit.edit_bit_type(bit_type);
        self.sort_ledger();
        Pass(())
    }

    /// Calculates and adds a fee from raw parts.
    fn calculate_fee_from_raw_parts(&mut self, starting_balance_string: &str,  ending_balance_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date) -> Schrod<()> {
        // This mirrors the checks in Bit::are_raw_fee_parts_valid(). I may be able to save code
        // instead of reimplementing this 3 times, but at least for now it's ok with me.
        
        // the starting balance
        let starting_balance_result = Schrod::from_result(Decimal::from_str_exact(starting_balance_string), "Failed to convert starting_balance_string to Decimal!", "BitWallet::calculate_fee_from_raw_parts()");
        if starting_balance_result.is_fail() {
            return starting_balance_result
                .convert("BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }
        let starting_balance = starting_balance_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::calculate_fee_from_raw_parts()");
        if starting_balance <= Decimal::ZERO {
            return Schrod::new_fail("Amount cannot be less than zero!", "BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }
        
        // the ending balance
        let ending_balance_result = Schrod::from_result(Decimal::from_str_exact(ending_balance_string), "Failed to convert starting_balance_string to Decimal!", "BitWallet::calculate_fee_from_raw_parts()");
        if ending_balance_result.is_fail() {
            return ending_balance_result
                .convert("BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }
        let ending_balance = ending_balance_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::calculate_fee_from_raw_parts()");
        if ending_balance <= Decimal::ZERO {
            return Schrod::new_fail("Amount cannot be less than zero!", "BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }

        // the fee amount
        let amount = starting_balance - ending_balance;
        if ending_balance <= Decimal::ZERO {
            return Schrod::new_fail("Gas cost cannot be less than zero!", "BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }

        // the coin value
        let coin_value_amount_result = Schrod::from_result(Decimal::from_str(coin_value_amount_string), "Failed to convert coin_value_amount_string to Decimal.", "BitWallet::calculate_fee_from_raw_parts()");
        if coin_value_amount_result.is_fail() {
            return coin_value_amount_result
                .convert("BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }
        let coin_value_amount = coin_value_amount_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::calculate_fee_from_raw_parts()");
        let coin_value_currency_result = Schrod::from_option(iso::find(&coin_value_currency_string.to_uppercase()), "Failed to convert coin_value_currency_string to a Coin.", "BitWallet::calculate_fee_from_raw_parts()");
        if coin_value_currency_result.is_fail() {
            return coin_value_currency_result
                .convert("BitWallet::calculate_fee_from_raw_parts()")
                .fail("Failed to add fee from raw parts.", "BitWallet::calculate_fee_from_raw_parts()")
        }
        let coin_value_currency = coin_value_currency_result.wont_fail("This is past an is_fail() guard clause.", "BitWallet::calculate_fee_from_raw_parts()");
        let coin_value = Value::from_decimal(coin_value_amount, coin_value_currency);
        
        // adding the fee
        self.add_bit_from_parts(amount, coin_value, date, BitTypes::Fee);
        Pass(())
    }
    
    /// Removes a `Bit` from the `ledger`.
    pub fn remove_bit(&mut self, id: Uuid) {
        self.ledger.retain(|bit| bit.get_id() != id);
    }

    
    
    // data retrieval and parsing
    /// Gets the `id` of the `BitWallet`.
    #[must_use]
    pub fn get_id(&self) -> Uuid { self.id }
    
    /// Returns an immutable reference to the `ledger`.
    #[must_use]
    pub fn get_ledger(&self) -> &Vec<Bit> { &self.ledger }

    /// Returns a mutable reference to the `ledger`.
    #[must_use]
    pub fn get_ledger_mut(&mut self) -> &mut Vec<Bit> { &mut self.ledger }

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

    /// Gets the `id`s from a list of `Bit`s.
    #[must_use]
    pub fn get_bit_ids_from(bits: &Vec<&Bit>) -> Vec<Uuid> {
        bits.iter().map(|b| b.get_id()).collect()
    }
    
    /// Gets the `Date` of the latest `Bit` in the ledger.
    /// If the `ledger` is empty, this returns the default `Date`.
    #[must_use]
    pub fn get_latest_date(&self) -> Date {
        self.ledger.first().map(|b| b.get_date()).unwrap_or_default()
    }
}