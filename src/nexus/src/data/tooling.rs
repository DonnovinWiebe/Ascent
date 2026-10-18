/// Implements `serde::serialize` and `serde::deserialize` for `Coin`.
pub mod coin_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use slip44::Coin;
    use std::convert::TryFrom;

    /// Serializes a `Coin`.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn serialize<S: Serializer>(coin: &Coin, s: S) -> Result<S::Ok, S::Error> {
        coin.id().serialize(s)
    }

    /// Deserializes a `Coin`.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Coin, D::Error> {
        let id = u32::deserialize(d)?;
        Coin::try_from(id).map_err(serde::de::Error::custom)
    }
}

/// Implements `serde::serialize` and `serde::deserialize` for `Value` (`Money`).
pub mod value_serde {
    use rust_decimal::Decimal;
    use rusty_money::iso;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use crate::standard::transaction::Value;

    /// Holds the data contained in a `Value` in a serializable format.
    #[derive(Serialize, Deserialize)]
    struct ValueMiddle {
        amount: Decimal,
        currency_string: String,
    }

    /// Serializes a `Value`.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn serialize<S: Serializer>(value: &Value, s: S) -> Result<S::Ok, S::Error> {
        let amount = *value.amount();
        let currency_string = value.currency().iso_alpha_code.to_string();
        let middle = ValueMiddle { amount, currency_string };
        middle.serialize(s)
    }

    /// Deserializes a `Value`.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Value, D::Error> {
        let middle = ValueMiddle::deserialize(d)?;
        let currency = iso::find(&middle.currency_string)
            .ok_or_else(|| serde::de::Error::custom("unknown currency code"))?;
        Ok(Value::from_decimal(middle.amount, currency))
    }
}