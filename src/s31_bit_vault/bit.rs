use std::str::FromStr;
use rust_decimal::Decimal;
use rusty_money::iso;
use schrod::Schrod;
use serde::Deserialize;
use serde::Serialize;
use slip44::Coin;
use uuid::Uuid;
use crate::s21_vault::transaction::Transaction;
use crate::s21_vault::transaction::Value;
use crate::s21_vault::transaction::Date;
use crate::s11_container::save_engine::value_serde;

/// The types of tranactions that a `Bit` can represent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BitTypes {
    /// Buying crypto.
    Aquisition,
    /// Selling crypto.
    Liquidation,
    /// Additional costs for working with crypto (such as transfering and whatnot).
    Fee,
}



/// Holds cryptocurrency transaction information inside of a `BitWallet`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bit {
    /// The id of the `Bit`.
    id: Uuid,
    /// The amount purchased/liquidated.
    /// This is always positive.
    amount: Decimal,
    /// The value of a single `Coin` at the time of the transaction.
    #[serde(with = "value_serde")]
    coin_value: Value,
    /// The `Date` of the transaction.
    date: Date,
    /// The type of tranactions that the `Bit` represents.
    bit_type: BitTypes,
}
impl PartialEq for Bit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Bit {
    // initializing
    /// Creates a new `Bit`.
    #[must_use]
    pub fn new(amount: Decimal, coin_value: Value, date: Date, bit_type: BitTypes) -> Bit {
        Bit { id: Uuid::new_v4(), amount, coin_value, date, bit_type }
    }



    // validation
    /// Checks if a `Bit` can be created from the given raw parts.
    #[must_use]
    pub fn are_raw_parts_valid(amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str) -> bool {
        // checking the amount
        let amount_result = Schrod::from_result(Decimal::from_str_exact(amount_string), "Failed to convert amount_string to Decimal.", "Bit::are_raw_parts_valid()");
        if amount_result.is_fail() { return false }
        let amount = amount_result.wont_fail("This is past an is_fail() guard clause.", "Bit::are_raw_parts_valid()");
        if amount <= Decimal::ZERO { return false }

        // checking the coin value
        let is_coin_value_amount_string_valid = Transaction::can_parse_to_decimal(coin_value_amount_string);
        let is_coin_value_currency_string_valid = Transaction::can_parse_to_currency(coin_value_currency_string);
        is_coin_value_amount_string_valid && is_coin_value_currency_string_valid
    }
    
    /// Checks if a fee can be created from the given raw parts.
    #[must_use]
    pub fn are_raw_fee_parts_valid(starting_balance_string: &str, ending_balance_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str) -> bool {
        // checking the starting balance
        let starting_balance_result = Schrod::from_result(Decimal::from_str_exact(starting_balance_string), "Failed to convert amount_string to Decimal.", "Bit::are_raw_parts_valid()");
        if starting_balance_result.is_fail() { return false }
        let starting_balance = starting_balance_result.wont_fail("This is past an is_fail() guard clause.", "Bit::are_raw_parts_valid()");
        if starting_balance <= Decimal::ZERO { return false }
        
        // checking the ending balance
        let ending_balance_string = Schrod::from_result(Decimal::from_str_exact(starting_balance_string), "Failed to convert amount_string to Decimal.", "Bit::are_raw_parts_valid()");
        if ending_balance_string.is_fail() { return false }
        let ending_balance = ending_balance_string.wont_fail("This is past an is_fail() guard clause.", "Bit::are_raw_parts_valid()");
        if ending_balance <= Decimal::ZERO { return false }

        // checking the amount
        if starting_balance - ending_balance <= Decimal::ZERO { return false }

        // checking the coin value
        let is_coin_value_amount_string_valid = Transaction::can_parse_to_decimal(coin_value_amount_string);
        let is_coin_value_currency_string_valid = Transaction::can_parse_to_currency(coin_value_currency_string);
        is_coin_value_amount_string_valid && is_coin_value_currency_string_valid
    }



    // basic getters
    /// Gets the `id` of the `Bit`.
    #[must_use]
    pub fn get_id(&self) -> Uuid { self.id }

    /// Gets the `amount` of the `Bit`.
    #[must_use]
    pub fn get_amount(&self) -> Decimal { self.amount }

    /// Gets the `coin_value` of the `Bit`.
    #[must_use]
    pub fn get_coin_value(&self) -> Value { self.coin_value }

    /// Gets the `value` of the `Bit`.
    /// The `value` is the `amount` multiplied by the `coin_value`.
    #[must_use]
    pub fn get_value(&self) -> Value {
        let currency = self.coin_value.currency();
        let value = self.coin_value.amount() * self.amount;
        Value::from_decimal(value, currency)
    }

    /// Gets the `date` of the `Bit`.
    #[must_use]
    pub fn get_date(&self) -> Date { self.date }

    /// Gets the `bit_type` of the `Bit`.
    #[must_use]
    pub fn get_bit_type(&self) -> BitTypes { self.bit_type }



    // basic editing
    /// Edits the `amount` of the `Bit`.
    pub fn edit_amount(&mut self, new_amount: Decimal) { self.amount = new_amount }

    /// Edits the `coin_value` of the `Bit`.
    pub fn edit_coin_value(&mut self, new_coin_value: Value) { self.coin_value = new_coin_value }

    /// Edits the `date` of the `Bit`.
    pub fn edit_date(&mut self, new_date: Date) { self.date = new_date }

    /// Edits the `bit_type` of the `Bit`.
    pub fn edit_bit_type(&mut self, new_bit_type: BitTypes) { self.bit_type = new_bit_type }
}