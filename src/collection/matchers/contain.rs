use crate::collection::panicking::{assert_failed_binary, AssertKind};

pub(crate) trait Contains<U> where U: Eq + std::fmt::Debug {
    fn should_contain(&self, element: &U) -> &Self;
    fn should_not_contain(&self, element: &U) -> &Self;
}

impl<T> Contains<T> for Vec<T>
    where T: std::fmt::Debug,
          T: Eq {
    fn should_contain(&self, element: &T) -> &Self {
        let contains = self.iter().any(|source| source == element);
        if !contains {
            assert_failed_binary(AssertKind::Contains, self, element);
        }
        self
    }

    fn should_not_contain(&self, element: &T) -> &Self {
        let contains = self.iter().any(|source| source == element);
        if contains {
            assert_failed_binary(AssertKind::NotContains, &self, element);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::matchers::contain::Contains;

    #[test]
    fn should_contain() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_contain(&"assert4j");
    }

    #[test]
    #[should_panic]
    fn should_contain_but_was_not_contained() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_contain(&"catch");
    }

    #[test]
    fn should_not_contain() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_not_contain(&"catch");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_but_was_contained() {
        let collection = vec!["junit", "assert4j", "catch2"];
        collection.should_not_contain(&"catch2");
    }
}