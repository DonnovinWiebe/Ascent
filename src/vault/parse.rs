use crate::vault::bank::{Bank, CurrencyExchange};
use schrod::Schrod;
use schrod::Schrod::Pass;
use crate::vault::transaction::{Id, Value};
use rust_decimal::Decimal;
use rusty_money::iso::Currency;
use serde::{Deserialize, Serialize};
use std::ops::Add;

/// The different ways a `CashFlow` can be displayed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlowTypes {
    /// Collected in separate `Currency` groups.
    Collected,
    /// Unified into a single `Currency` value.
    Unified,
    /// As a time price.
    Time,
}



/// Holds cash flow values for multiple `Currency`s from a single list of `Transaction`s.
#[derive(Debug, Clone, PartialEq)]
pub struct CashFlow {
    /// The list of `Value`s grouped by `Currency`.
    value_flows: Vec<Value>,
    /// The unified value of all `Currency`s in the `value_flows`.
    unified_value_flow: Value,
    /// The overall cash flow represented as a time price.
    time_flow: Decimal,
}
impl CashFlow {
    /// Creates a new `CashFlow` from a list of `Transaction` `Id`s.
    #[must_use]
    pub fn new(bank: &Bank, transaction_ids: &[Id]) -> Schrod<CashFlow> {
        // value flows
        let value_flows_result = CashFlow::get_value_flows(bank, transaction_ids.to_owned());
        if value_flows_result.is_fail() {
            return value_flows_result
                .convert("CashFlow::new()")
                .fail("Failed to create Cash Flow.", "CashFlow::new()");
        }
        let value_flows = value_flows_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::new()");

        // unified value
        let unified_value_flow_result = CashFlow::get_unified_value_flow(bank, &value_flows);
        if unified_value_flow_result.is_fail() {
            return unified_value_flow_result
                .convert("CashFlow::new()")
                .fail("Failed to create Cash Flow.", "CashFlow::new()");
        }
        let unified_value_flow = unified_value_flow_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::new()");

        // time flow
        let time_flow_result = CashFlow::get_time_flow(&unified_value_flow, &bank.currency_exchange);
        if time_flow_result.is_fail() {
            return time_flow_result
                .convert("CashFlow::new()")
                .fail("Failed to create Cash Flow.", "CashFlow::new()");
        }
        let time_flow = time_flow_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::new()");

        Pass(CashFlow {
            value_flows,
            unified_value_flow,
            time_flow,
        })
    }

    /// Gets the `String` representation of the `CashFlow` based on the given flow type.
    #[must_use]
    pub fn display(&self, flow_type: FlowTypes) -> Vec<String> {
        match flow_type {
            FlowTypes::Collected => {
                self.value_flows.iter().map(|vf| format!("{} {}", vf, vf.currency())).collect()
            }
            FlowTypes::Unified => {
                vec![self.unified_value_flow.to_string()]
            }
            FlowTypes::Time => {
                vec![CurrencyExchange::as_time_price_string(self.time())]
            }
        }
    }

    /// Returns the collected value flows of the `CashFlow` as a `Vec<Value>`.
    #[must_use]
    pub fn collected(&self) -> Vec<Value> {
        self.value_flows.clone()
    }

    /// Returns the unified value flow of the `CashFlow` as a `Value`.
    #[must_use]
    pub fn unified(&self) -> Value {
        self.unified_value_flow
    }

    /// Returns the time flow of the `CashFlow`.
    #[must_use]
    pub fn time(&self) -> Decimal {
        self.time_flow
    }

    /// Turns a list of `Transaction`s into a collection of `Value`s, grouped by `Currency`,
    /// that each represent the overall cash flow for the given `Currency`.
    #[must_use]
    fn get_value_flows(bank: &Bank, transaction_ids: Vec<Id>) -> Schrod<Vec<Value>> {
        // the list of all the transactions (by id) grouped by their currencies
        let mut coupled_value_groups: Vec<(Currency, Vec<Id>)> = Vec::new();

        // collects each transaction value into separate value groups
        for id in transaction_ids {
            // the current transaction
            let transaction_result = bank.get(id);
            if transaction_result.is_fail() {
                return transaction_result
                    .convert("CashFlow::get_value_flows()")
                    .fail("Failed to get value flows.", "CashFlow::get_value_flows()")
            }
            let transaction = transaction_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_value_flows()");
            // checks if the currency has been used already
            let mut is_currency_used = false;

            // adds the transaction to the current group if their currencies are the same
            for group in &mut coupled_value_groups {
                if *transaction.value.currency() == group.0 {
                    is_currency_used = true;
                    group.1.push(id);
                    break;
                }
            }

            // creates a new group if the currency has not been used yet
            if !is_currency_used { coupled_value_groups.push((*transaction.value.currency(), vec![id])); }
        }

        // collects the coupled cash flow groups into individual values
        let value_flow_results: Vec<Schrod<Value>> = coupled_value_groups.into_iter().map(|couple| {
            // tracks the flow of each couple
            let mut flow: Decimal = Decimal::ZERO;
            // adds the transaction value to the flow
            for id in &couple.1 {
                let transaction_result = bank.get(*id);
                if transaction_result.is_fail() {
                    return transaction_result
                        .convert("CashFlow::get_value_flows()")
                        .fail("Failed to get value flows.", "CashFlow::get_value_flows()");
                }
                let transaction = transaction_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_value_flows()");
                let value_amount = transaction.value.amount();
                flow = flow.add(value_amount);
            }
            
            // gets the currency from the first transaction in the couple
            let first_transaction_result = bank.get(couple.1[0]);
            if first_transaction_result.is_fail() {
                return first_transaction_result
                    .convert("CashFlow::get_value_flows()")
                    .fail("Failed to get value flows.", "CashFlow::get_value_flows()");
            }
            let last_transaction = first_transaction_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_value_flows()");
            let currency = last_transaction.value.currency();
            Pass(Value::from_decimal(flow, currency))
        }).collect();

        // returns early if any value flow results are fails
        if Schrod::contains_fail(&value_flow_results) {
            return Schrod::collect_and_fail(&value_flow_results, "CashFlow::get_value_flows()")
                .convert("CashFlow::get_value_flows()")
                .fail("Failed to get value flows.", "CashFlow::get_value_flows()");
        }

        // takes the inernal values out of the results
        let value_flows: Vec<Value> = value_flow_results
            .into_iter()
            .map(|vf| vf.wont_fail("This is past a contains_fail() guard clause.", "CashFlow::get_value_flows()"))
            .collect();
        
        // returns the cash flow groups
        Pass(value_flows)
    }

    /// Returns all value flows combined into the same `Currency` based on the `main_currency` in the `CurrencyExchange`.
    #[must_use]
    fn get_unified_value_flow(bank: &Bank, value_flows: &[Value]) -> Schrod<Value> {
        let new_value_results: Vec<_> = value_flows
            .iter()
            .map(|flow| bank.currency_exchange.convert(flow.amount(), flow.currency(), bank.currency_exchange.get_main_currency()))
            .collect();

        if Schrod::contains_fail(&new_value_results) {
            return Schrod::collect_and_fail(&new_value_results, "CashFlow::unified()")
                    .convert("CashFlow::unified()")
                    .fail("Failed to unify values!", "CashFlow::unified()")
        }

        let new_values: Vec<_> = new_value_results
            .into_iter()
            .map(|r| r.wont_fail("This is past a contains_fail() guard clause.", "CashFlow::unified()"))
            .collect();
        let mut unified_value = Decimal::from(0);
        for value in new_values { unified_value += value; }

        Pass(Value::from_decimal(unified_value, bank.currency_exchange.get_main_currency()))
    }

    /// Gets the overall time flow value from a list of `Value`s.
    #[must_use]
    fn get_time_flow(unified_value_flow: &Value, currency_exchange: &CurrencyExchange) -> Schrod<Decimal> {
        currency_exchange.as_time_price(unified_value_flow)
    }
}