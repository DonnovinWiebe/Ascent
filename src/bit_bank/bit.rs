use rust_decimal::Decimal;
use uuid::Uuid;
use crate::vault::transaction::Value;

use crate::vault::transaction::Date;

/// Holds cryptocurrency transaction information inside of a `BitWallet`.
#[derive(Debug, Clone, Copy)]
pub struct Bit {
    /// The id of the `Bit`.
    id: Uuid,
    /// The amount purchased/liquidated.
    amount: Decimal,
    /// The value of a single `Coin` at the time of the transaction.
    coin_value: Value,
    /// The `Date` of the transaction.
    date: Date,
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
    pub fn new(amount: Decimal, coin_value: Value, date: Date) -> Bit {
        Bit { id: Uuid::new_v4(), amount, coin_value, date }
    }



    // basic getters
    /// Gets the `id` of the `Bit`.
    pub fn get_id(&self) -> Uuid { self.id }

    /// Gets the `amount` of the `Bit`.
    pub fn get_amount(&self) -> Decimal { self.amount }

    /// Gets the `coin_value` of the `Bit`.
    pub fn get_coin_value(&self) -> Value { self.coin_value }

    /// Gets the `value` of the `Bit`.
    /// The `value` is the `amount` multiplied by the `coin_value`.
    pub fn get_value(&self) -> Value {
        let currency = self.coin_value.currency();
        let value = self.coin_value.amount() * self.amount;
        Value::from_decimal(value, currency)
    }

    /// Gets the `date` of the `Bit`.
    pub fn get_date(&self) -> Date { self.date }



    // basic editing
    /// Edits the `amount` of the `Bit`.
    pub fn edit_amount(&mut self, new_amount: Decimal) { self.amount = new_amount }

    /// Edits the `coin_value` of the `Bit`.
    pub fn edit_coin_value(&mut self, new_coin_value: Value) { self.coin_value = new_coin_value }

    /// Edits the `date` of the `Bit`.
    pub fn edit_date(&mut self, new_date: Date) { self.date = new_date }
}