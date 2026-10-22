/// Lists the various platforms that Ascent can run on.
#[derive(Copy, Clone)]
pub enum Platforms {
    Server,
    Desktop,
    Mobile,
}

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

/// Implements `serde::serialize` and `serde::deserialize` for a collection of `Transaction` parts.
pub mod transaction_parts_serde {
    use rust_decimal::Decimal;
    use rusty_money::iso;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use crate::standard::transaction::{Date, Tag, Value};

    /// Holds the parts data in a serializable format.
    #[derive(Serialize, Deserialize)]
    struct PartsMiddle {
        amount: Decimal,
        currency_string: String,
        date: Date,
        description: String,
        tags: Vec<Tag>,
    }

    /// Serializes `Transaction` parts.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn serialize<S: Serializer>(value: &Value, date: &Date, description: &String, tags: &Vec<Tag>, s: S) -> Result<S::Ok, S::Error> {
        let amount = *value.amount();
        let currency_string = value.currency().iso_alpha_code.to_string();
        let middle = PartsMiddle { amount, currency_string, date: date.clone(), description: description.clone(), tags: tags.clone() };
        middle.serialize(s)
    }

    /// Deserializes `Transaction` parts.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<(Value, Date, String, Vec<Tag>), D::Error> {
        let middle = PartsMiddle::deserialize(d)?;
        let currency = iso::find(&middle.currency_string)
            .ok_or_else(|| serde::de::Error::custom("unknown currency code"))?;
        Ok((Value::from_decimal(middle.amount, currency), middle.date, middle.description, middle.tags))
    }
}

/// Implements `serde::serialize` and `serde::deserialize` for a collection of `Transaction` parts including the id.
pub mod id_transaction_parts_serde {
    use rust_decimal::Decimal;
    use rusty_money::iso;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use uuid::Uuid;
    use crate::standard::transaction::{Date, Tag, Value};

    /// Holds the parts data in a serializable format.
    #[derive(Serialize, Deserialize)]
    struct PartsMiddle {
        id: Uuid,
        amount: Decimal,
        currency_string: String,
        date: Date,
        description: String,
        tags: Vec<Tag>,
    }

    /// Serializes `Transaction` parts including the id.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn serialize<S: Serializer>(id: &Uuid, value: &Value, date: &Date, description: &String, tags: &Vec<Tag>, s: S) -> Result<S::Ok, S::Error> {
        let amount = *value.amount();
        let currency_string = value.currency().iso_alpha_code.to_string();
        let middle = PartsMiddle { id: id.clone(), amount, currency_string, date: date.clone(), description: description.clone(), tags: tags.clone() };
        middle.serialize(s)
    }

    /// Deserializes `Transaction` parts including the id.
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<(Uuid, Value, Date, String, Vec<Tag>), D::Error> {
        let middle = PartsMiddle::deserialize(d)?;
        let currency = iso::find(&middle.currency_string)
            .ok_or_else(|| serde::de::Error::custom("unknown currency code"))?;
        Ok((middle.id, Value::from_decimal(middle.amount, currency), middle.date, middle.description, middle.tags))
    }
}