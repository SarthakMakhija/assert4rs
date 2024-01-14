pub mod bool;
pub mod char;
pub mod collection;
pub mod compose;
#[cfg(feature = "date")]
pub mod date;
pub mod equal;
#[cfg(feature = "file")]
pub mod file;
#[cfg(feature = "num")]
pub mod float;
#[cfg(feature = "num")]
pub mod int;
pub mod map;
pub mod option;
pub mod ordered;
pub mod range;
pub mod result;
pub mod string;

/// Should provides a convenient way to express positive assertions within tests, indicating that a value should meet a certain condition.
pub trait Should<T> {
    /// - Takes a matcher as input and performs an assertion against the value itself.
    /// - Panics if the assertion fails, indicating that the value did not match the matcher's expectations.
    fn should(&self, matcher: &dyn Matcher<T>);
}

/// ShouldNot provides a convenient way to express negative assertions within tests, indicating that a value should not meet a certain condition.
pub trait ShouldNot<T> {
    /// - Takes a matcher as input and performs a negated assertion against the value itself.
    /// - Inverts the result of the matcher's test method, ensuring the value does not match.
    /// - Panics if the negated assertion fails, indicating that the value unexpectedly matched the matcher.
    fn should_not(&self, matcher: &dyn Matcher<T>);
}

impl<T> Should<T> for T {
    fn should(&self, matcher: &dyn Matcher<T>) {
        let matcher_result = matcher.test(self);
        if !matcher_result.passed {
            panic!("assertion failed: {}", matcher_result.failure_message);
        }
    }
}

impl<T> ShouldNot<T> for T {
    fn should_not(&self, matcher: &dyn Matcher<T>) {
        let matcher_result = matcher.test(self);
        let passed = !matcher_result.passed;
        if !passed {
            panic!(
                "assertion failed: {}",
                matcher_result.negated_failure_message
            );
        }
    }
}

/// Matcher defines the core functionality of matchers. All the matchers implement Matcher<T> trait.
pub trait Matcher<T> {
    fn test(&self, value: &T) -> MatcherResult;
}

/// BoxWrap provides a boxed method to wrap a Matcher into Box object.
///
/// It is used to compose matchers in [`crate::matchers::compose::Matchers`].
///
/// BoxWrap is implemented for any T: Matcher<M>.
pub trait BoxWrap<W> {
    fn boxed(self) -> Box<dyn Matcher<W>>;
}

impl<M, T: Matcher<M> + 'static> BoxWrap<M> for T {
    fn boxed(self) -> Box<dyn Matcher<M>> {
        Box::new(self)
    }
}

/// MatcherResult defines the result of a matcher execution.
pub struct MatcherResult {
    passed: bool,
    failure_message: String,
    negated_failure_message: String,
}

impl MatcherResult {
    /// new creates a new instance of MatcherResult using failure_message and negated_failure_message of type &'static str.
    pub fn new(
        passed: bool,
        failure_message: &'static str,
        negated_failure_message: &'static str,
    ) -> Self {
        MatcherResult::formatted(
            passed,
            failure_message.to_string(),
            negated_failure_message.to_string(),
        )
    }

    /// formatted creates a new instance of MatcherResult using failure_message and negated_failure_message of type String.
    pub fn formatted(
        passed: bool,
        failure_message: String,
        negated_failure_message: String,
    ) -> Self {
        MatcherResult {
            passed,
            failure_message,
            negated_failure_message,
        }
    }

    /// passed returns true if the result of a matcher execution was successful, false otherwise.
    pub fn passed(&self) -> bool {
        self.passed
    }
}
