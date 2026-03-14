use crate::vault::result_stack::ResultStack::{Pass, Fail};

/// A custom result type to help track errors through their corresponding call stacks.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultStack<T> {
    Pass(T),
    Fail(FailureStack)
}
impl<T> ResultStack<T> {
    /// Returns a new Fail from a single message.
    pub fn new_fail(message: &str) -> ResultStack<T> {
        Fail(FailureStack::new(message))
    }
    
    /// Returns a new Fail from an existing FailureStack.
    /// This is useful for essentially converting ResultStack T types.
    pub fn new_fail_from_stack(stack: FailureStack) -> ResultStack<T> {
        Fail(stack)
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
    pub fn from_result<E: ToString>(result: Result<T, E>, possible_failure_message: &str) -> ResultStack<T> {
        match result {
            Ok(value) => { Pass(value) }
            Err(err) => {
                let result_stack = ResultStack::new_fail(&err.to_string());
                result_stack.fail(possible_failure_message)
            }
        }
    }

    /// Returns a ResultStack from an Option.
    pub fn from_option(option: Option<T>, possible_failure_message: &str) -> ResultStack<T> {
        match option {
            Some(value) => { Pass(value) }
            None => {
                let result_stack = ResultStack::new_fail("Received a None value.");
                result_stack.fail(possible_failure_message)
            }
        }
    }

    /// Adds another failure message to the Fail FailureStack.
    /// If this is called on a Pass, a new Fail is created.
    pub fn fail(&self, message: &str) -> ResultStack<T> {
        match self {
            Pass(_) => {
                let mut full_messages = vec![message.to_string()];
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
    
    /// Returns the FailureStack from a Fail.
    /// If this is called on a Pass, a new FailureStack is returned.
    pub fn get_stack(&self) -> FailureStack {
        match self {
            Pass(_) => { FailureStack::new("Pass") }
            Fail(stack) => { stack.clone() }
        }
    }
    
    /// Returns the most recent result message or "Unknown failure." if there are no results.
    pub fn most_recent_result(&self) -> String {
        let results = self.results();
        if results.is_empty() { return "Unknown failure.".to_string(); }
        results[0].clone()
    }
    
    /// Forces the ResultStack to yield the Pass value or panic! if it cannot.
    /// This is useful shorthand for testing but is not recommended for production code.
    pub fn unwrap(self) -> T {
        match self {
            Pass(value) => value,
            Fail(_) => panic!("Tried to unwrap a ResultStack::Fail"),
        }
    }
    
    /// This is a more suitable alternative to unwrap() for production code.
    /// If this should panic!, a reason message is printed as to why it should not have failed.
    pub fn wont_fail(self, why_is_it_safe: &str) -> T {
        match self {
            Pass(value) => value,
            Fail(_) => panic!("A ResultStack::Fail was unwrapped when guaranteed to succeed: {}", why_is_it_safe),
        }
    }
    
    /// Returns true if this ResultStack is a Pass.
    pub fn is_pass(&self) -> bool {
        match self {
            Pass(_) => true,
            Fail(_) => false,
        }
    }
    
    /// Returns true if this ResultStack is a Fail.
    pub fn is_fail(&self) -> bool {
        match self {
            Pass(_) => false,
            Fail(_) => true,
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
    fn new(initial_message: &str) -> FailureStack {
        FailureStack { messages: vec![initial_message.to_string()] }
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
    fn continued(&self, new_message: &str) -> FailureStack {
        let mut propagated_messages = self.messages.clone();
        propagated_messages.push(new_message.to_string());
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