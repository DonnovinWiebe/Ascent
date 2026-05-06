use crate::vault::{bank::Bank, result_stack::ResultStack, transaction::{Date, Id, Months, Tag}};
use crate::vault::result_stack::ResultStack::{Pass};

/// A custom type that helps to clarify how the some groups of `Transaction`s are meant to denote groups sepparated by time intervals.
pub type TimeGroup = Vec<Id>;



/// Defines how `Transaction`s can be split by time intervals.
pub enum Intervals {
    /// Groups by week.
    Weekly(Date, usize),
    /// Groups by two-week couples.
    BiWeekly(Date, usize),
    /// Groups by month.
    Monthly(Date, usize),
    /// Groups by quarter.
    Quarterly(Date, usize),
    /// Groups by year.
    Yearly(Date, usize),
}



/// A custom object that displays financial trends over time based on a given time `Interval`.
pub struct TrendSetter {
    show_cash_flow: bool,
    tags: Vec<Tag>,
    interval: Intervals,
}
impl TrendSetter {
    /// Splits a list of `Transaction`s (`Id`s) by a given `Interval`.
    #[must_use]
    fn split_by_interval(interval: Intervals) -> ResultStack<Vec<TimeGroup>> {
        match interval {
            Intervals::Weekly(last_date, distance) => {
                
                Pass(Vec::new())
            }
            
            Intervals::BiWeekly(last_date, distance) => {
                
                Pass(Vec::new())
            }
            
            Intervals::Monthly(last_date, distance) => {
                
                Pass(Vec::new())
            }
            
            Intervals::Quarterly(last_date, distance) => {
                
                Pass(Vec::new())
            }
            
            Intervals::Yearly(last_date, distance) => {
                
                Pass(Vec::new())
            }
        }
    }

    /// Checks if two `Transaction`s are in the same quarter.
    #[must_use]
    fn is_in_same_quarter(bank: &Bank, first_id: Id, second_id: Id) -> ResultStack<bool> {
        let first_quarter_id_result = TrendSetter::get_quarter_id_for(bank, first_id);
        if first_quarter_id_result.is_fail() { return ResultStack::new_fail_from_stack(first_quarter_id_result.get_stack()).fail("Failed to check if two Transactions are in the same quarter."); }
        let first_quarter_id = first_quarter_id_result.wont_fail("This is past an is_fail() guard clause.");
        let second_quarter_id_result = TrendSetter::get_quarter_id_for(bank, second_id);
        if second_quarter_id_result.is_fail() { return ResultStack::new_fail_from_stack(second_quarter_id_result.get_stack()).fail("Failed to check if two Transactions are in the same quarter."); }
        let second_quarter_id = second_quarter_id_result.wont_fail("This is past an is_fail() guard clause.");
        
        Pass(first_quarter_id == second_quarter_id)
    }

    /// Gets the quarter a given `Transaction` is in.
    #[must_use]
    fn get_quarter_id_for(bank: &Bank, id: Id) -> ResultStack<u32> {
        let transaction_result = bank.get(id);
        if transaction_result.is_fail() { return ResultStack::new_fail_from_stack(transaction_result.get_stack()).fail("Failed to get the quarter id from the given Transaction!"); }
        let transaction = transaction_result.wont_fail("This is past an is_fail() guard clause.");
        
        let year = transaction.date.get_year();
        let quarter = match transaction.date.get_month() {
            Months::January | Months::February | Months::March => 1,
            Months::April | Months::May | Months::June => 2,
            Months::July | Months::August | Months::September => 3,
            Months::October | Months::November | Months::December => 4,
        };
        
        Pass((year * 10) + quarter)
    }

    /// Checks if two `Transaction`s are in the same week.
    #[must_use]
    fn is_in_same_week(bank: &Bank, first_id: Id, second_id: Id) -> ResultStack<bool> {
        let first_week_id_result = TrendSetter::get_week_id_for(bank, first_id);
        if first_week_id_result.is_fail() { return ResultStack::new_fail_from_stack(first_week_id_result.get_stack()).fail("Failed to check if two Transactions are in the same week."); }
        let first_week_id = first_week_id_result.wont_fail("This is past an is_fail() guard clause.");
        let second_week_id_result = TrendSetter::get_week_id_for(bank, second_id);
        if second_week_id_result.is_fail() { return ResultStack::new_fail_from_stack(second_week_id_result.get_stack()).fail("Failed to check if two Transactions are in the same week."); }
        let second_week_id = second_week_id_result.wont_fail("This is past an is_fail() guard clause.");
        
        Pass(first_week_id == second_week_id)
    }

    /// Gets the week a given `Transaction` is in.
    #[must_use]
    fn get_week_id_for(bank: &Bank, id: Id) -> ResultStack<u32> {
        let transaction_result = bank.get(id);
        if transaction_result.is_fail() { return ResultStack::new_fail_from_stack(transaction_result.get_stack()).fail("Failed to get the week id from the given Transaction!"); }
        let transaction = transaction_result.wont_fail("This is past an is_fail() guard clause.");

        let mut distance_into_year: u32 = transaction.date.get_day();
        for month in transaction.date.get_month().get_previous_months() {
            distance_into_year += month.days_in_month(transaction.date.get_year());
        }
        
        Pass((distance_into_year / 7) + 1)
    }

    /// Checks if two `Transaction`s are in the same two-week couple.
    #[must_use]
    fn is_in_same_biweek(bank: &Bank, first_id: Id, second_id: Id) -> ResultStack<bool> {
        let first_biweek_id_result = TrendSetter::get_biweek_id_for(bank, first_id);
        if first_biweek_id_result.is_fail() { return ResultStack::new_fail_from_stack(first_biweek_id_result.get_stack()).fail("Failed to check if two Transactions are in the same biweek."); }
        let first_biweek_id = first_biweek_id_result.wont_fail("This is past an is_fail() guard clause.");
        let second_biweek_id_result = TrendSetter::get_biweek_id_for(bank, second_id);
        if second_biweek_id_result.is_fail() { return ResultStack::new_fail_from_stack(second_biweek_id_result.get_stack()).fail("Failed to check if two Transactions are in the same biweek."); }
        let second_biweek_id = second_biweek_id_result.wont_fail("This is past an is_fail() guard clause.");
        
        Pass(first_biweek_id == second_biweek_id)
    }

    /// Gets the two-week couple a given `Transaction` is in.
    #[must_use]
    fn get_biweek_id_for(bank: &Bank, id: Id) -> ResultStack<u32> {
        let week_id_result = TrendSetter::get_week_id_for(bank, id);
        if week_id_result.is_fail() { return ResultStack::new_fail_from_stack(week_id_result.get_stack()).fail("Failed to get the biweek id from the given Transaction!"); }
        let week_id = week_id_result.wont_fail("This is past an is_fail() guard clause.");
        
        Pass(week_id / 2)
    }
}