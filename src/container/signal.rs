use crate::container::app::*;
use crate::vault::transaction::*;

pub enum Signal {
    StartAddingTransaction,
    StartEditingTransaction(usize), // id
    StartRemovingTransaction(usize), // id
    Cancel(Pages), // the page to return to
    AddTransaction(Value, Date, Tag, Vec<Tag>), // value, date, description, tags
    EditTransaction(Value, Date, Tag, Vec<Tag>), // value, date, description, tags
}