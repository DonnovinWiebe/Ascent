use crate::vault::schrod::Schrod::{Pass, Fail};
use std::future::{IntoFuture, ready, Ready};

/// A custom result type to help track errors through their corresponding call stacks.
/// The name comes from Schrödinger's cat, a thought experiment that illustrates the uncertainty of a measurement until examined.
#[derive(Debug, PartialEq)]
pub enum Schrod<T> {
    Pass(T),
    Fail(Trace),
}
impl<T: Clone> IntoFuture for Schrod<T> {
    type Output = Schrod<T>;
    type IntoFuture = Ready<Schrod<T>>;

    fn into_future(self) -> Self::IntoFuture {
        ready(self)
    }
}
impl<T: Clone> Clone for Schrod<T> {
    fn clone(&self) -> Self {
        match self {
            Pass(value) => Pass(value.clone()),
            Fail(trace) => Fail(trace.clone()),
        }
    }
}
impl<T> Schrod<T> {
    /// Returns a new `Fail` from a single message in its `Trace`.
    #[must_use]
    pub fn new_fail(message: &str, function_name: &str) -> Schrod<T> {
        Fail(Trace::new(&format!("{function_name}: {message}")))
    }

    /// Adds another failure message to the `Fail`'s `Trace`.
    /// If this is called on a `Pass`, a new `Fail` is created.
    #[must_use]
    pub fn fail(&self, message: &str, function_name: &str) -> Schrod<T> {
        match self {
            Pass(_) => {
                let mut messages = vec![message.to_string()];
                messages.insert(0, format!("Failed a Pass in {function_name}."));
                Fail(Trace::new_from_list(messages))
            }
            
            Fail(stack) => { Fail(stack.continued(&format!("{function_name}: {message}"))) }
        }
    }
    
    /// Converts a `Fail` to be any type.
    /// This is useful for chaining many different `Fail`s together.
    #[must_use]
    pub fn convert<U>(&self, function_name: &str) -> Schrod<U> {
        match self {
            Pass(_) => Schrod::new_fail("Converted Pass to a Fail.", function_name),
            Fail(stack) => Fail(stack.clone()),
        }
    }

    /// Returns a `Schrod` from a `Result`.
    #[must_use]
    pub fn from_result<E: ToString>(result: Result<T, E>, possible_failure_message: &str, function_name: &str) -> Schrod<T> {
        match result {
            Ok(value) => { Pass(value) }
            Err(err) => {
                let result_stack = Schrod::new_fail(&err.to_string(), function_name);
                result_stack.fail(possible_failure_message, function_name)
            }
        }
    }

    /// Returns a `Schrod` from an `Option`.
    #[must_use]
    pub fn from_option(option: Option<T>, possible_failure_message: &str, function_name: &str) -> Schrod<T> {
        if let Some(value) = option { Pass(value) }
        else {
            let result_stack = Schrod::new_fail("Received a None value.", function_name);
            result_stack.fail(possible_failure_message, function_name)
        }
    }

    /// Tells if any `Schrod` in the given list is a `Fail`.
    #[must_use]
    pub fn contains_fail(schrods: &[Schrod<T>]) -> bool {
        schrods.iter().any(|s| s.is_fail())
    }

    /// Collects all `Fail`s results from the given list and returns a `Fail` with a combined `Trace`.
    #[must_use]
    pub fn collect_and_fail(schrods: &[Schrod<T>], function_name: &str) -> Schrod<T> {
        let failures = schrods.into_iter().filter(|s| s.is_fail()).collect::<Vec<_>>();
        if failures.is_empty() { Schrod::new_fail("Collected Schrods and failed with no failures in list.", function_name) }
        else {
            let mut new_messages = Vec::new();
            for (i, fail) in failures.iter().enumerate() {
                for message in fail.get_trace().messages {
                    new_messages.push(format!("{i}: {message}"));
                }
            }
            
            Fail(Trace::new_from_list(new_messages))
        }
    }

    /// Returns the `Trace` of the given `Fail`, or a default `Pass` `Trace` if this is called on a `Pass`.
    #[must_use]
    fn get_trace(&self) -> Trace {
        match self {
            Pass(_) => { Trace::new(&format!("Pass: No stack available.")) }
            Fail(trace) => { trace.clone() }
        }
    }
    
    /// Fetches the results gathered by the `Schrod`.
    /// If this is called on a `Fail`, the failures logged along the call stack are returned.
    /// If this is called on a `Pass`, a passing message is returned.
    #[must_use]
    pub fn results(&self) -> Vec<String> {
        match self {
            Pass(_) => { vec!["Pass".to_string()] }
            Fail(stack) => { stack.messages.clone() }
        }
    }
    
    /// Forces the `Schrod` to yield the `Pass` value.
    /// # Panics
    /// If called on a `Fail`, the code panics and a reason message is printed as to why it should not have failed.
    #[must_use]
    pub fn wont_fail(self, why_is_it_safe: &str, function_name: &str) -> T {
        match self {
            Pass(value) => value,
            Fail(_) => panic!("A Schrod::Fail was unwrapped in {function_name} when guaranteed to succeed: {why_is_it_safe}"),
        }
    }
    
    /// Forces the `Schrod` to yield an immutable reference to the `Pass` value.
    /// # Panics
    /// If called on a `Fail`, the code panics and a reason message is printed as to why it should not have failed.
    #[must_use]
    pub fn wont_fail_ref(&self, why_is_it_safe: &str, function_name: &str) -> &T {
        match self {
            Pass(value) => value,
            Fail(_) => panic!("A Schrod::Fail was unwrapped in {function_name} when guaranteed to succeed: {why_is_it_safe}"),
        }
    }
    
    /// Forces the `Schrod` to yield a mutable reference to the `Pass` value.
    /// # Panics
    /// If called on a `Fail`, the code panics and a reason message is printed as to why it should not have failed.
    #[must_use]
    pub fn wont_fail_ref_mut(&mut self, why_is_it_safe: &str, function_name: &str) -> &mut T {
        match self {
            Pass(value) => value,
            Fail(_) => panic!("A Schrod::Fail was unwrapped in {function_name} when guaranteed to succeed: {why_is_it_safe}"),
        }
    }
    
    /// Returns `true` if this `Schrod` is a `Pass`.
    #[must_use]
    pub fn is_pass(&self) -> bool {
        match self {
            Pass(_) => true,
            Fail(_) => false,
        }
    }
    
    /// Returns `true` if this `Schrod` is a `Fail`.
    #[must_use]
    pub fn is_fail(&self) -> bool {
        match self {
            Pass(_) => false,
            Fail(_) => true,
        }
    }
}



/// Used to track errors through a call stack.
#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    messages: Vec<String>,
}
impl Trace {
    /// Creates a new `Trace` object from a single message.
    #[must_use]
    fn new(initial_message: &str) -> Trace {
        Trace { messages: vec![initial_message.to_string()] }
    }

    /// Creates a new `Trace` object from a list of messages.
    #[must_use]
    fn new_from_list(initial_messages: Vec<String>) -> Trace {
        if initial_messages.is_empty() { Trace { messages: vec!["Failed with no failure messages.".to_string()] } }
        else { Trace { messages: initial_messages } }
    }

    /// Returns a new `Trace` object with the given message added to the end.
    #[must_use]
    fn continued(&self, new_message: &str) -> Trace {
        let mut propagated_messages = self.messages.clone();
        propagated_messages.push(new_message.to_string());
        Trace { messages: propagated_messages }
    }

    /// Returns a new `Trace` object with the given messages added to the end.
    #[must_use]
    fn continued_from_list(&self, new_messages: Vec<String>) -> Trace {
        let mut propagated_messages = self.messages.clone();

        if new_messages.is_empty() {
            propagated_messages.push("Failed with no failure messages.".to_string());
        }
        else {
            for message in new_messages { propagated_messages.push(message.clone()); }
        }

        Trace { messages: propagated_messages }
    }
}