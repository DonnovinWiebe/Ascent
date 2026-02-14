use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{iso::Currency, Money};
use crate::vault::transaction::{Id, Transaction, Value};
use rust_decimal_macros::dec;
use crate::vault::bank::Bank;

/// Holds cash flow values for multiple currencies from a single list of transactions.
pub struct CashFlow {
    /// The list of transaction id's used.
    transaction_ids: Vec<Id>,
    /// The list of values grouped by currency.
    pub value_flows: Vec<Value>,
    /// The overall cash flow represented as a time price.
    pub time_flow: Vec<f64>,
}
impl CashFlow {
    /// Creates a new cash flows object from a list of transaction id's.
    pub fn new(transaction_ids: Vec<Id>, bank: &Bank) -> CashFlow {
        let value_flows = Self::get_value_flows(transaction_ids.clone(), bank);
        let time_flow = vec![Self::get_time_flow(&value_flows)];

        CashFlow {
            transaction_ids,
            value_flows,
            time_flow,
        }
    }

    /// Turns a list of transactions into a collection of values, grouped by currency, that each represent the overall cash flow for the given currency.
    fn get_value_flows(transaction_ids: Vec<Id>, bank: &Bank) -> Vec<Value> {
        // the list of all the transactions (by id) grouped into their currencies
        let mut coupled_value_groups: Vec<(Currency, Vec<Id>)> = Vec::new();

        // collects each transaction value into separate value groups
        for id in transaction_ids {
            // the current transaction
            let transaction = bank.get(id);
            // checks if the currency has been used already
            let mut is_currency_used = false;

            // adds the transaction to the current group if their currencies are the same
            for group in &mut coupled_value_groups {
                if transaction.value.currency().clone() == group.0 {
                    is_currency_used = true;
                    group.1.push(id);
                    break;
                }
            }

            // creates a new group if the currency has not been used yet
            if !is_currency_used {
                coupled_value_groups.push((transaction.value.currency().clone(), vec![id]));
            }
        }

        // collects the coupled cash flow groups into individual values
        let value_flows: Vec<Value> = coupled_value_groups.into_iter().map(|couple| {
            let mut flow: f64 = 0.0;
            for id in &couple.1 {
                flow += bank.get(id.clone()).value.amount().to_f64().expect("Invalid transaction value!");
            }
            Value::from_minor((flow * 100.0) as i64, bank.get(couple.1[0]).value.currency()) // each couple is guaranteed to have at least one transaction
        }).collect();

        // returns the cash flow groups
        value_flows
    }

    /// Gets the overall time flow value from a list of values.
    fn get_time_flow(value_flows: &Vec<Value>) -> f64 {
        let mut time_flow = 0.0;
        for value_flow in value_flows {
            time_flow += value_flow.amount().to_f64().expect("Invalid transaction value!"); //todo convert for currency
        }
        time_flow
    }
}