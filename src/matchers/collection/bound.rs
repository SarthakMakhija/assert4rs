use crate::matchers::{Matcher, MatcherResult};
use std::fmt::Debug;

pub enum BoundBased<'a, T: PartialOrd + Debug> {
    Upper(&'a T),
    Lower(&'a T),
}

impl<'a, T> BoundBased<'a, T>
where
    T: PartialOrd + Debug,
{
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            BoundBased::Upper(bound) => MatcherResult::formatted(
                collection.iter().all(|source| bound >= &source),
                format!("{:?} should have upper bound {:?}", collection, bound),
                format!("{:?} should not have upper bound {:?}", collection, bound),
            ),
            BoundBased::Lower(bound) => MatcherResult::formatted(
                collection.iter().all(|source| bound <= &source),
                format!("{:?} should have lower bound {:?}", collection, bound),
                format!("{:?} should not have lower bound {:?}", collection, bound),
            ),
        }
    }
}

impl<T: PartialOrd + Debug> Matcher<Vec<T>> for BoundBased<'_, T> {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(&collection)
    }
}

impl<T: PartialOrd + Debug, const N: usize> Matcher<[T; N]> for BoundBased<'_, T> {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd + Debug> Matcher<&[T]> for BoundBased<'_, T> {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(&collection)
    }
}

pub fn have_upper_bound<T: PartialOrd + Debug>(bound: &T) -> BoundBased<'_, T> {
    BoundBased::Upper(bound)
}

pub fn have_lower_bound<T: PartialOrd + Debug>(bound: &T) -> BoundBased<'_, T> {
    BoundBased::Lower(bound)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
    use crate::matchers::collection::bound::{have_lower_bound, have_upper_bound};

    #[test]
    fn should_have_an_upper_bound() {
        let matcher = have_upper_bound(&4);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_an_upper_bound_but_was_not() {
        let matcher = have_upper_bound(&3);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_have_a_lower_bound() {
        let matcher = have_lower_bound(&1);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_a_lower_bound_but_was_not() {
        let matcher = have_lower_bound(&3);
        let collection = vec![1, 2, 3, 4];

        matcher.test(&collection).passed.should_be_true();
    }
}
