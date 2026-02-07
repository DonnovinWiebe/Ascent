use crate::vault::bank::*;

pub enum Pages {
    Transactions,
    AddingTransaction,
    EditingTransaction,
    RemovingTransaction,
    Quitting,
}



pub struct App {
    pub bank: Bank,
}
impl App {
    pub fn new() -> App {
        App { bank: Bank::new() }
    }
}