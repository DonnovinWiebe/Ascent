use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{iso::Currency, Money};
use crate::vault::transaction::{Transaction};
use rust_decimal_macros::dec;
use crate::vault::bank::Bank;

pub struct CashFlowGrouping {
    transaction_ids: Vec<usize>,
    pub money_list: Vec<Money<'static, Currency>>,
}
impl CashFlowGrouping {
    pub fn new(transaction_ids: Vec<usize>, bank: &Bank) -> CashFlowGrouping {
        let money_list = Self::get_money_list(transaction_ids.clone(), bank);

        CashFlowGrouping {
            transaction_ids,
            money_list,
        }
    }

    fn get_money_list(transaction_ids: Vec<usize>, bank: &Bank) -> Vec<Money<'static, Currency>> {
        // all the transactions grouped into their currencies
        let mut coupled_cash_flow_groups: Vec<(Currency, Vec<usize>)> = Vec::new();

        // collects each transaction value into separate currency groups
        for id in transaction_ids {
            // the current transaction
            let transaction = bank.get(id);
            // checks if the currency has been used already
            let mut is_currency_used = false;

            // adds the transaction to the current group if their currencies are the same
            for group in &mut coupled_cash_flow_groups {
                if transaction.value.currency().clone() == group.0 {
                    is_currency_used = true;
                    group.1.push(id);
                    break;
                }
            }

            // creates a new group if the currency has not been used yet
            if !is_currency_used {
                coupled_cash_flow_groups.push((transaction.value.currency().clone(), vec![id]));
            }
        }

        // collects the coupled cash flow groups into individual money structs
        let cash_flow_groups: Vec<Money<'static, Currency>> = coupled_cash_flow_groups.into_iter().map(|couple| {
            let mut flow: f64 = 0.0;
            for id in &couple.1 {
                flow += bank.get(id.clone()).value.amount().to_f64().expect("Invalid transaction value!");
            }
            Money::from_minor((flow * 100.0) as i64, bank.get(couple.1[0]).value.currency()) // each couple is guaranteed to have at least one transaction
        }).collect();

        // returns the cash flow groups
        cash_flow_groups
    }
}