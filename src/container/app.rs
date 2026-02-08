use crate::vault::bank::*;

pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    RemovingTransaction,
    Quitting,
}



pub struct App<'bank> {
    pub bank: Bank<'bank>,
}
impl<'bank> App<'bank> {
    pub fn new() -> App<'bank> {
        App { bank: Bank::new() }
    }
}