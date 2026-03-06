use crate::vault::result::Result::{Pass, Fail};

/// A custom Result type to help track errors through their corresponding call stacks.
pub enum Result<T> {
    Pass(T),
    Fail(FailureStack)
}
impl<T> Result<T> {
    /// Adds another failure message to the Fail FailureStack.
    /// If this is called on a Pass, a new Fail is created.
    pub fn fail(&self, message: String) -> Result<T> {
        match self {
            Pass(_) => {
                Fail(FailureStack::new("Added a failure message to Pass type.".to_string()))
            }
            Fail(stack) => {
                Fail(stack.continued(message))
            }
        }
    }

    /// Fetches the results gathered by the Result type.
    /// If this is called on a Fail, the failures logged along the call stack are returned.
    /// If this is called on a Pass, a passing message is returned.
    pub fn results(&self) -> Vec<String> {
        match self {
            Pass(_) => { vec!["Pass".to_string()] }
            Fail(stack) => { stack.messages.clone() }
        }
    }
}



/// Used to track errors through a call stack.
pub struct FailureStack {
    messages: Vec<String>,
}
impl FailureStack {
    /// Creates a new failure stack object.
    fn new(initial_message: String) -> FailureStack {
        FailureStack { messages: vec![initial_message] }
    }

    /// Adds a message to the failure stack.
    fn continued(&self, new_message: String) -> FailureStack {
        let mut propagated_messages = self.messages.clone();
        propagated_messages.push(new_message);
        FailureStack { messages: propagated_messages }
    }
}