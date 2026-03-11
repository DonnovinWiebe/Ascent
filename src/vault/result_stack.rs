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

    /// Returns a new Fail from a list of unknown failures.
    pub fn new_fail_from_unknown_failure(possible_failures: Vec<Option<Vec<String>>>) -> ResultStack<T> {
        let mut messages = Vec::new();

        for possible_failure in possible_failures {
            if let Some(possible_failures) = possible_failure {
                messages.extend(possible_failures);
            }
        }

        Fail(FailureStack::new_from_list(messages))
    }
    
    /// Returns a ResultStack without the generic type parameter.
    /// This is useful for the failure branch of a function that returns an empty Fail(()).
    pub fn empty_type(&self) -> ResultStack<()> {
        match self {
            Pass(_) => Pass(()),
            Fail(stack) => Fail(stack.clone()),
        }
    }

    /// Gets a list of possible failure messages.
    /// If it is a Failure, Some(messages) is returned.
    /// If it is a Pass, None is returned.
    pub fn get_possible_failures(&self) -> Option<Vec<String>> {
        match self {
            Pass(_) => { None }
            Fail(stack) => { Some(stack.messages.clone()) }
        }
    }

    /// Returns a ResultStack from a Result.
    pub fn from_result<E: ToString>(result: Result<T, E>, possible_failure_message: String) -> ResultStack<T> {
        match result {
            Ok(value) => { Pass(value) }
            Err(err) => {
                let result_stack = ResultStack::new_fail(err.to_string());
                result_stack.fail(possible_failure_message)
            }
        }
    }

    /// Returns a ResultStack from an Option.
    pub fn from_option(option: Option<T>, possible_failure_message: String) -> ResultStack<T> {
        match option {
            Some(value) => { Pass(value) }
            None => {
                let result_stack = ResultStack::new_fail("Received a None value.".to_string());
                result_stack.fail(possible_failure_message)
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
    pub fn fail_from_unknown_fail(&self, possible_failures: Vec<Option<Vec<String>>>) -> ResultStack<T> {
        let mut messages = Vec::new();

        for possible_failure in possible_failures {
            if let Some(possible_failures) = possible_failure {
                messages.extend(possible_failures);
            }
        }

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
        if initial_messages.is_empty() {
            FailureStack { messages: vec!["Failed with no failure messages.".to_string()] }
        }
        else {
            FailureStack { messages: initial_messages }
        }
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

        if new_messages.is_empty() {
            propagated_messages.push("Failed with no failure messages.".to_string());
        }
        else {
            for message in new_messages {
                propagated_messages.push(message.clone());
            }
        }

        FailureStack { messages: propagated_messages }
    }
}