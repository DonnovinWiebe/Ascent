use crate::vault::result_stack::ResultStack::{Pass, Fail};

/// A custom result type to help track errors through their corresponding call stacks.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultStack<T> {
    Pass(T),
    Fail(FailureStack)
}
impl<T> ResultStack<T> {
    /// Returns a new Fail from a single message.
    pub fn new_fail(message: String) -> ResultStack<T> {
        Fail(FailureStack::new(message))
    }
    
    /// Returns a new Fail from a list of messages.
    pub fn new_fail_from_list(messages: Vec<String>) -> ResultStack<T> {
        Fail(FailureStack::new_from_list(messages))
    }
    
    /// Returns a new Fail from an unknown component.
    pub fn new_fail_from_unknown_component(components: &Vec<ResultStack<T>>) -> ResultStack<T> {
        let failure_stacks: Vec<_> = components.iter().filter_map(|component| {
            match component {
                Pass(_) => { None }
                Fail(stack) => { Some(stack.messages.clone()) }
            }
        }).collect();

        let messages = failure_stacks.into_iter().flatten().collect::<Vec<_>>();

        Fail(FailureStack::new_from_list(messages))
    }

    /// Returns a ResultStack from a Result.
    pub fn from_result<E: ToString>(result: Result<T, E>, possible_failure_message: String) -> ResultStack<T> {
        match result {
            Ok(value) => { Pass(value) }
            Err(err) => {
                let result_stack = ResultStack::new_fail(err.to_string());
                result_stack.fail(possible_failure_message);
                result_stack
            }
        }
    }

    /// Returns a ResultStack from an Option.
    pub fn from_option(option: Option<T>, possible_failure_message: String) -> ResultStack<T> {
        match option {
            Some(value) => { Pass(value) }
            None => {
                let result_stack = ResultStack::new_fail("Received a None value.".to_string());
                result_stack.fail(possible_failure_message);
                result_stack
            }
        }
    }

    /// Adds another failure message to the Fail FailureStack.
    /// If this is called on a Pass, a new Fail is created.
    pub fn fail(&self, message: String) -> ResultStack<T> {
        match self {
            Pass(_) => {
                let mut full_messages = vec![message];
                full_messages.insert(0, "Added a failure message to Pass type.".to_string());
                Fail(FailureStack::new_from_list(full_messages))
            }
            Fail(stack) => {
                Fail(stack.continued(message))
            }
        }
    }

    /// Adds a list of failure messages to the Fail FailureStack.
    /// If this is called on a Pass, a new Fail is created.
    pub fn fail_from_list(&self, messages: Vec<String>) -> ResultStack<T> {
        match self {
            Pass(_) => {
                let mut full_messages = messages;
                full_messages.insert(0, "Added a failure message to Pass type.".to_string());
                Fail(FailureStack::new_from_list(full_messages))
            }
            Fail(stack) => {
                Fail(stack.continued_from_list(messages))
            }
        }
    }

    /// Adds a failure for each of the components that failed.
    /// If every component is a Pass, a new Fail is created.
    pub fn fail_from_unknown_component(&self, components: &Vec<ResultStack<T>>) -> ResultStack<T> {
        let failure_stacks: Vec<_> = components.iter().filter_map(|component| {
            match component {
                Pass(_) => { None }
                Fail(stack) => { Some(stack.messages.clone()) }
            }
        }).collect();

        let messages = failure_stacks.into_iter().flatten().collect::<Vec<_>>();

        self.fail_from_list(messages)
    }

    /// Fetches the results gathered by the ResultStack.
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
#[derive(Debug, Clone, PartialEq)]
pub struct FailureStack {
    messages: Vec<String>,
}
impl FailureStack {
    /// Creates a new failure stack object from a single message.
    fn new(initial_message: String) -> FailureStack {
        FailureStack { messages: vec![initial_message] }
    }

    /// Creates a new failure stack object from a list of messages.
    fn new_from_list(initial_messages: Vec<String>) -> FailureStack {
        FailureStack { messages: initial_messages }
    }

    /// Adds a message to the failure stack.
    fn continued(&self, new_message: String) -> FailureStack {
        let mut propagated_messages = self.messages.clone();
        propagated_messages.push(new_message);
        FailureStack { messages: propagated_messages }
    }

    /// Adds a list of messages to the failure stack.
    fn continued_from_list(&self, new_messages: Vec<String>) -> FailureStack {
        let mut propagated_messages = self.messages.clone();
        for message in new_messages {
            propagated_messages.push(message.clone());
        }
        FailureStack { messages: propagated_messages }
    }
}