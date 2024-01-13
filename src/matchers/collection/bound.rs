use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

pub enum BoundMatcher<T: PartialOrd + Debug> {
    Upper(T),
    Lower(T),
}

impl<T> BoundMatcher<T>
    where
        T: PartialOrd + Debug,
{
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            BoundMatcher::Upper(bound) => MatcherResult::formatted(
                collection.iter().all(|source| source <= bound),
                format!("{:?} should have upper bound {:?}", collection, bound),
                format!("{:?} should not have upper bound {:?}", collection, bound),
            ),
            BoundMatcher::Lower(bound) => MatcherResult::formatted(
                collection.iter().all(|source| source >= bound),
                format!("{:?} should have lower bound {:?}", collection, bound),
                format!("{:?} should not have lower bound {:?}", collection, bound),
            ),
        }
    }
}

impl<T: PartialOrd + Debug> Matcher<Vec<T>> for BoundMatcher<T> {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<T: PartialOrd + Debug, const N: usize> Matcher<[T; N]> for BoundMatcher<T> {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd + Debug> Matcher<&[T]> for BoundMatcher<T> {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

pub fn have_upper_bound<T: PartialOrd + Debug>(bound: T) -> BoundMatcher<T> {
    BoundMatcher::Upper(bound)
}

pub fn have_lower_bound<T: PartialOrd + Debug>(bound: T) -> BoundMatcher<T> {
    BoundMatcher::Lower(bound)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::bound::{have_lower_bound, have_upper_bound};

    #[test]
    fn should_have_an_upper_bound() {
        let matcher = have_upper_bound(4);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_an_upper_bound_but_was_not() {
        let matcher = have_upper_bound(3);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_have_a_lower_bound() {
        let matcher = have_lower_bound(1);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_a_lower_bound_but_was_not() {
        let matcher = have_lower_bound(3);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }
}
