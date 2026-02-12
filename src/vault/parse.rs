use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{iso::Currency, Money};
use crate::vault::transaction::Transaction;
use rust_decimal_macros::dec;

pub struct CashFlowGrouping {
    flow_groups: Vec<Money<'static, Currency>>
}
impl CashFlowGrouping {
    pub fn new(transactions: Vec<&Transaction>) -> CashFlowGrouping {
        // all the transactions grouped into their currencies
        let mut coupled_cash_flow_groups: Vec<(Currency, Vec<&Transaction>)> = Vec::new();

        // collects each transaction value into separate currency groups
        for transaction in transactions {
            // checks if the currency has been used already
            let mut is_currency_used = false;

            // adds the transaction to the current group if their currencies are the same
            for group in &mut coupled_cash_flow_groups {
                if transaction.value.currency().clone() == group.0 {
                    is_currency_used = true;
                    group.1.push(transaction);
                    break;
                }
            }

            // creates a new group if the currency has not been used yet
            if !is_currency_used {
                coupled_cash_flow_groups.push((transaction.value.currency().clone(), vec![transaction]));
            }
        }

        // collects the coupled cash flow groups into individual money structs
        let cash_flow_groups: Vec<Money<'static, Currency>> = coupled_cash_flow_groups.into_iter().map(|couple| {
            let mut flow: f64 = 0.0;
            for transaction in &couple.1 {
                flow += transaction.value.amount().to_f64().expect("Invalid transaction value!");
            }
            Money::from_minor((flow * 100.0) as i64, couple.1[0].value.currency()) // each couple is guaranteed to have at least one transaction
        }).collect();

        // returns the cash flow groups
        CashFlowGrouping { flow_groups: cash_flow_groups }
    }
}