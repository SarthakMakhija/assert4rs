use crate::matchers::bound::{have_lower_bound, have_upper_bound};
use crate::matchers::Should;
use crate::panicking::{assert_failed_binary, AssertKind};

pub trait Bounds<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    fn should_have_upper_bound(&self, element: &T) -> &Self;
    fn should_have_lower_bound(&self, element: &T) -> &Self;
}

impl<T> Bounds<T> for Vec<T>
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_have_upper_bound(&self, element: &T) -> &Self {
        (self as &[T]).should_have_upper_bound(element);
        self
    }

    fn should_have_lower_bound(&self, element: &T) -> &Self {
        (self as &[T]).should_have_lower_bound(element);
        self
    }
}

impl<T, const N: usize> Bounds<T> for [T; N]
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_have_upper_bound(&self, element: &T) -> &Self {
        (self as &[T]).should_have_upper_bound(element);
        self
    }

    fn should_have_lower_bound(&self, element: &T) -> &Self {
        (self as &[T]).should_have_lower_bound(element);
        self
    }
}

impl<T> Bounds<T> for [T]
where
    T: std::fmt::Debug,
    T: PartialOrd,
{
    fn should_have_upper_bound(&self, element: &T) -> &Self {
        if !self.should(&have_upper_bound(element)) {
            assert_failed_binary(AssertKind::UpperBound, &self, &element)
        }
        self
    }

    fn should_have_lower_bound(&self, element: &T) -> &Self {
        if !self.should(&have_lower_bound(element)) {
            assert_failed_binary(AssertKind::LowerBound, &self, &element)
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::collection::bound::Bounds;

    #[test]
    fn should_have_an_upper_bound() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_upper_bound(&4);
    }

    #[test]
    #[should_panic]
    fn should_have_an_upper_bound_but_was_not() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_upper_bound(&1);
    }

    #[test]
    fn should_have_a_lower_bound() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_lower_bound(&1);
    }

    #[test]
    #[should_panic]
    fn should_have_a_lower_bound_but_was_not() {
        let collection = vec![1, 2, 3, 4];
        collection.should_have_lower_bound(&3);
    }
}
