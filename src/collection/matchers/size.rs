use crate::collection::panicking::{assert_failed_binary, AssertKind};

pub(crate) trait Size {
    fn should_have_size(&self, size: usize) -> &Self;
    fn should_not_have_size(&self, size: usize) -> &Self;
    fn should_have_at_least_size(&self, size: usize) -> &Self;
    fn should_have_at_most_size(&self, size: usize) -> &Self;
    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self;
}

impl<T> Size for Vec<T>
    where T: std::fmt::Debug {
    fn should_have_size(&self, size: usize) -> &Self {
        if self.len() != size {
            assert_failed_binary(AssertKind::EqualSize, self, &size);
        }
        self
    }

    fn should_not_have_size(&self, size: usize) -> &Self {
        if self.len() == size {
            assert_failed_binary(AssertKind::NotEqualSize, self, &size);
        }
        self
    }

    fn should_have_at_least_size(&self, size: usize) -> &Self {
        if !(self.len() >= size) {
            assert_failed_binary(AssertKind::AtleastSize, self, &size);
        }
        self
    }

    fn should_have_at_most_size(&self, size: usize) -> &Self {
        if !(self.len() <= size) {
            assert_failed_binary(AssertKind::AtmostSize, self, &size);
        }
        self
    }

    fn should_be_same_size_as<U>(&self, other: &[U]) -> &Self {
        if self.len() != other.len() {
            assert_failed_binary(AssertKind::EqualSize, self, &other.len());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::matchers::size::Size;

    #[test]
    fn should_have_size() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size(2);
    }

    #[test]
    #[should_panic]
    fn should_have_size_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_have_size(3);
    }

    #[test]
    fn should_not_have_size() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size(3);
    }

    #[test]
    #[should_panic]
    fn should_not_have_size_but_was() {
        let collection = vec!["junit", "testify"];
        collection.should_not_have_size(2);
    }

    #[test]
    fn should_have_at_least_size() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_least_size(3);
    }

    #[test]
    #[should_panic]
    fn should_have_at_least_size_but_was_not() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_least_size(4);
    }

    #[test]
    fn should_have_at_most_size() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_most_size(3);
    }

    #[test]
    #[should_panic]
    fn should_have_at_most_size_but_was_not() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_have_at_most_size(2);
    }

    #[test]
    fn should_be_same_size_as_other() {
        let collection = vec!["junit", "testify", "catch2"];
        collection.should_be_same_size_as(&[1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn should_be_same_size_as_other_but_was_not() {
        let collection = vec!["junit", "testify"];
        collection.should_be_same_size_as(&[1, 2, 3]);
    }
}