use crate::vault::{bank::Bank, parse::CashFlow, result_stack::ResultStack, transaction::{Date, Id, Months, Tag, Transaction}};
use crate::vault::result_stack::ResultStack::{Pass, Fail};

/// Defines how `Transaction`s can be split by time intervals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intervals {
    /// Groups by week.
    Weekly,
    /// Groups by two-week couples.
    BiWeekly,
    /// Groups by month.
    Monthly,
    /// Groups by quarter.
    Quarterly,
    /// Groups by year.
    Yearly,
}



pub struct TrendParse {
    time_lines: Vec<TimeLine>,
    interval: Intervals
}
impl TrendParse {
    pub fn new(bank: &Bank, transactions: &Vec<Transaction>, show_overall_cash_flow: bool, tags: Vec<Tag>, interval: Intervals, last_date: Date, length: usize) -> ResultStack<TrendParse> {
        // the list of time lines
        let mut time_line_results = Vec::new();

        // adding a time line for the overall cash flow
        if show_overall_cash_flow { time_line_results.push(TimeLine::new(bank, transactions, None, interval, last_date, length)) }

        // adding time lines for each tag
        for tag in tags { time_line_results.push(TimeLine::new(bank, transactions, Some(tag), interval, last_date, length)) }

        // filters out any failures
        let mut failures = Vec::new();
        let mut time_lines = Vec::new();
        for result in time_line_results {
            match result {
                Pass(time_line) => time_lines.push(time_line),
                Fail(_) => failures.push(result),
            }
        }

        // returns a fail if there were any failures
        if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to create TrendParse.") }

        // returns the trend parse
        Pass(TrendParse { time_lines, interval })
    }
}



pub struct TimeLine {
    /// Each `TimeLine` has a `Tag` attached. No `Tag` represents the overall `CashFlow`.
    tag: Option<Tag>,
    /// The list of `TimeStamp`s
    time_stamps: Vec<TimeStamp>
}
impl TimeLine {
    fn new(bank: &Bank, transactions: &Vec<Transaction>, trending_tag: Option<Tag>, interval: Intervals, last_date: Date, length: usize) -> ResultStack<TimeLine>{
        // splits the transactions by time group.
        let mut all_time_groups: Vec<Vec<&Transaction>> = Vec::new();
        for transaction in transactions { TimeLine::place_into_time_group(transaction, &mut all_time_groups, interval); }

        // finds the first group to add.
        // it is the "first" because it comes first in the list, not because it is the first chronolgically
        let mut starting_index = 0;
        let mut start_found = false;
        for (i, group) in all_time_groups.iter().enumerate() {
            if TimeLine::contains_date(&group, last_date, interval) {
                start_found = true;
                starting_index = i;
                break;
            }
        }

        if !start_found { return ResultStack::new_fail("Failed to find the starting date in the given Transactions!").fail("Failed to create TrendParse.") }
        
        // takes only the time groups within the given length
        let mut collected_time_groups = Vec::new();
        let mut time_groups_added = 0;
        for i in starting_index..all_time_groups.len() {
            if time_groups_added < length {
                collected_time_groups.push(all_time_groups[i].clone());
                time_groups_added += 1;
            }
        }

        // filters out all the transactions that do not have the tag
        // None results in getting the trend of the overall cash flow
        if let Some(tag) = &trending_tag {
            for time_group in &mut collected_time_groups {
                time_group.retain(|t| t.has_tag(tag));
            }
        }

        // collects the cash flows for the collected time groups
        let cash_flow_results: Vec<ResultStack<CashFlow>> = collected_time_groups.iter().map(|group| CashFlow::new(bank, &Bank::get_ids_from(group), 1.0)).collect();
        let mut cash_flows = Vec::new();
        let mut failures = Vec::new();
        for cash_flow_result in cash_flow_results {
            match cash_flow_result {
                Pass(cash_flow) => cash_flows.push(cash_flow),
                Fail(_) => failures.push(cash_flow_result),
            }
        }

        // returns a failure if any of the cash flows failed to generate
        if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to create TrendParse."); }

        // returns a failure is the length of cash flows and time groups are different
        if collected_time_groups.len() != cash_flows.len() { return ResultStack::new_fail("Generated mismatched TimeGroups and CashFlows while creating a TrendParse!").fail("Failed to create TrendParse.") }

        // creates the timeline from the collected time groups and cash flows
        let mut time_stamps = Vec::new();
        for (i, cash_flow) in cash_flows.into_iter().enumerate() {
            time_stamps.push(TimeStamp { cash_flow: cash_flow, label: TimeStamp::get_label(&collected_time_groups[i], interval) })
        }

        // returns a new TimeLine
        Pass(TimeLine { tag: trending_tag, time_stamps })
    }

    /// Checks if a given `Date` fits into a given `TimeGroup` (based on its `Interval`).
    #[must_use]
    fn contains_date(group: &Vec<&Transaction>, date: Date, interval: Intervals) -> bool {
        if group.is_empty() { return false; }
        
        match interval {
            Intervals::Weekly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::BiWeekly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::Monthly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::Quarterly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::Yearly => TimeLine::is_in_same_week(group[0].date, date),
        }
    }
    
    /// Places a `Transaction` into the correct `TimeGroup`.
    fn place_into_time_group<'a>(transaction: &'a Transaction, groups: &mut Vec<Vec<&'a Transaction>>, interval: Intervals) {
        for group in groups.iter_mut() {
            if group.is_empty() { continue; }

            match interval {
                Intervals::Weekly => {
                    if TimeLine::is_in_same_week(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::BiWeekly => {
                    if TimeLine::is_in_same_biweek(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::Monthly => {
                    if TimeLine::is_in_same_month(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::Quarterly => {
                    if TimeLine::is_in_same_quarter(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::Yearly => {
                    if TimeLine::is_in_same_year(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
            }
        }
        
        groups.push(vec![transaction]);
    }

    /// Checks if two `Date`s are in the same week.
    #[must_use]
    fn is_in_same_week(first: Date, second: Date) -> bool {
        TimeLine::get_week_id_for(first) == TimeLine::get_week_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same two-week couple.
    #[must_use]
    fn is_in_same_biweek(first: Date, second: Date) -> bool {
        TimeLine::get_biweek_id_for(first) == TimeLine::get_biweek_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same month.
    #[must_use]
    fn is_in_same_month(first: Date, second: Date) -> bool {
        TimeLine::get_month_id_for(first) == TimeLine::get_month_id_for(second)
    }

    /// Checks if two `Date`s are in the same quarter.
    #[must_use]
    fn is_in_same_quarter(first: Date, second: Date) -> bool {
        TimeLine::get_quarter_id_for(first) == TimeLine::get_quarter_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same quarter.
    #[must_use]
    fn is_in_same_year(first: Date, second: Date) -> bool {
        TimeLine::get_year_id_for(first) == TimeLine::get_year_id_for(second)
    }
        
    /// Gets the week id for a given `Date`.
    #[must_use]
    fn get_week_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let mut distance_into_year: u32 = date.get_day();
        for month in date.get_month().get_previous_months() {
            distance_into_year += month.days_in_month(date.get_year());
        }
        let week = (distance_into_year / 7) + 1;
        
        (year * 100) + week
    }

    /// Gets the two-week couple id for a given `Date`.
    #[must_use]
    fn get_biweek_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let mut distance_into_year: u32 = date.get_day();
        for month in date.get_month().get_previous_months() {
            distance_into_year += month.days_in_month(date.get_year());
        }
        let biweek = (distance_into_year / 14) + 1;
        
        (year * 100) + biweek
    }
    
    /// Gets the month id for a given `Date`.
    #[must_use]
    fn get_month_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let month = date.get_month().as_value();
        (year * 100) + month
    }

    /// Gets the quarter id for a given `Date`.
    #[must_use]
    fn get_quarter_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let quarter = match date.get_month() {
            Months::January | Months::February | Months::March => 1,
            Months::April | Months::May | Months::June => 2,
            Months::July | Months::August | Months::September => 3,
            Months::October | Months::November | Months::December => 4,
        };
        
        (year * 10) + quarter
    }
    
    /// Gets the year id for a given `Date`.
    #[must_use]
    fn get_year_id_for(date: Date) -> u32 {
        date.get_year()
    }
}



pub struct TimeStamp {
    /// Shows if money was earned or spent during a time period.
    cash_flow: CashFlow,
    /// The time period of the `TimeStamp`. (January, Q1 2026, etc.)
    label: String,
}
impl TimeStamp {
    fn get_label(time_group: &Vec<&Transaction>, interval: Intervals) -> String {
        if time_group.is_empty() { "No data".to_string() }
        else {
            let date = time_group[0].date;
            match interval {
                Intervals::Weekly => { format!("Week {}", TimeLine::get_week_id_for(date)) }
                Intervals::BiWeekly => { format!("Bi-Week {}", TimeLine::get_biweek_id_for(date)) }
                Intervals::Monthly => { format!("{}, {}", date.get_month().display(), date.get_year()) }
                Intervals::Quarterly => {
                    let quarter = match date.get_month() {
                        Months::January | Months::February | Months::March => 1,
                        Months::April | Months::May | Months::June => 2,
                        Months::July | Months::August | Months::September => 3,
                        Months::October | Months::November | Months::December => 4,
                    };
                    format!("Q{}", quarter)
                }
                Intervals::Yearly => { format!("{}", TimeLine::get_year_id_for(date)) }
            }
        }
    }
}