use std::collections::HashSet;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use walkdir::WalkDir;

use crate::matchers::{Matcher, MatcherResult};

pub enum FileTypeMatcher {
    File,
    Directory,
    SymbolicLink,
    ZeroSized,
    Readonly,
    Writable,
}

pub enum FilePathMatcher<'a> {
    Absolute,
    Relative,
    Extension(&'a str),
}

pub enum TreeMatcher<'a> {
    Contain(&'a str),
    ContainAll(&'a [&'a str]),
    ContainAny(&'a [&'a str]),
}

impl<T: AsRef<Path> + Debug> Matcher<T> for FileTypeMatcher {
    fn test(&self, value: &T) -> MatcherResult {
        let metadata = fs::metadata(value);
        match &self {
            FileTypeMatcher::File => MatcherResult::formatted(
                value.as_ref().is_file(),
                format!("{:?} should be a file", value),
                format!("{:?} should not be a file", value),
            ),
            FileTypeMatcher::Directory => MatcherResult::formatted(
                value.as_ref().is_dir(),
                format!("{:?} should be a directory", value),
                format!("{:?} should not be a directory", value),
            ),
            FileTypeMatcher::SymbolicLink => MatcherResult::formatted(
                value.as_ref().is_symlink(),
                format!("{:?} should be a symbolic link", value),
                format!("{:?} should not be a symbolic link", value),
            ),
            FileTypeMatcher::ZeroSized => MatcherResult::formatted(
                metadata.is_ok_and(|metadata| metadata.len() == 0),
                format!("{:?} should be zero sized", value),
                format!("{:?} should not be zero sized", value),
            ),
            FileTypeMatcher::Readonly => MatcherResult::formatted(
                metadata.is_ok_and(|metadata| metadata.permissions().readonly()),
                format!("{:?} should be readonly", value),
                format!("{:?} should not be readonly", value),
            ),
            FileTypeMatcher::Writable => MatcherResult::formatted(
                metadata.is_ok_and(|metadata| !metadata.permissions().readonly()),
                format!("{:?} should be writable", value),
                format!("{:?} should not be writable", value),
            ),
        }
    }
}

impl<T: AsRef<Path> + Debug> Matcher<T> for FilePathMatcher<'_> {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            FilePathMatcher::Absolute => MatcherResult::formatted(
                value.as_ref().is_absolute(),
                format!("{:?} should be absolute", value),
                format!("{:?} should not be absolute", value),
            ),
            FilePathMatcher::Relative => MatcherResult::formatted(
                value.as_ref().is_relative(),
                format!("{:?} should be relative", value),
                format!("{:?} should not be relative", value),
            ),
            FilePathMatcher::Extension(extension) => MatcherResult::formatted(
                value
                    .as_ref()
                    .extension()
                    .filter(|source| source == extension)
                    .is_some(),
                format!("{:?} should have extension {:?}", value, extension),
                format!("{:?} should not be have extension {:?}", value, extension),
            ),
        }
    }
}

impl<T: AsRef<Path> + Debug> Matcher<T> for TreeMatcher<'_> {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            TreeMatcher::Contain(name) => {
                for directory_entry in WalkDir::new(value) {
                    if let Ok(entry) = directory_entry {
                        if &entry.file_name() == name {
                            return MatcherResult::formatted(
                                true,
                                format!("{:?} should contain a file name {:?}", value, name),
                                format!("{:?} should not contain a file name {:?}", value, name),
                            );
                        }
                    }
                }
                MatcherResult::formatted(
                    false,
                    format!("{:?} should contain a file name {:?}", value, name),
                    format!("{:?} should not contain a file name {:?}", value, name),
                )
            }
            TreeMatcher::ContainAll(names) => {
                let mut unique_names = names
                    .iter()
                    .map(|name| OsStr::new(name))
                    .collect::<HashSet<_>>();

                for directory_entry in WalkDir::new(value) {
                    if let Ok(entry) = directory_entry {
                        if unique_names.contains(entry.file_name()) {
                            unique_names.remove(entry.file_name());
                        }
                    }
                }
                MatcherResult::formatted(
                    unique_names.len() == 0,
                    format!(
                        "{:?} should contain file names {:?} but was missing {:?}",
                        value, names, unique_names
                    ),
                    format!("{:?} should not contain file names {:?}", value, names),
                )
            }
            TreeMatcher::ContainAny(names) => {
                let mut unique_names = names
                    .iter()
                    .map(|name| OsStr::new(name))
                    .collect::<HashSet<_>>();
                let input_names = unique_names.clone();

                for directory_entry in WalkDir::new(value) {
                    if let Ok(entry) = directory_entry {
                        if unique_names.contains(entry.file_name()) {
                            unique_names.remove(entry.file_name());
                            break;
                        }
                    }
                }
                MatcherResult::formatted(
                    unique_names.len() != input_names.len(),
                    format!("{:?} should contain any of file names {:?}", value, names),
                    format!(
                        "{:?} should not contain any of file names {:?}",
                        value, names
                    ),
                )
            }
        }
    }
}

pub fn be_a_directory() -> FileTypeMatcher {
    FileTypeMatcher::Directory
}

pub fn be_a_file() -> FileTypeMatcher {
    FileTypeMatcher::File
}

pub fn be_a_symbolic_link() -> FileTypeMatcher {
    FileTypeMatcher::SymbolicLink
}

pub fn be_zero_sized() -> FileTypeMatcher {
    FileTypeMatcher::ZeroSized
}

pub fn be_readonly() -> FileTypeMatcher {
    FileTypeMatcher::Readonly
}

pub fn be_writable() -> FileTypeMatcher {
    FileTypeMatcher::Writable
}

pub fn be_absolute() -> FilePathMatcher<'static> {
    FilePathMatcher::Absolute
}

pub fn be_relative() -> FilePathMatcher<'static> {
    FilePathMatcher::Relative
}

pub fn have_extension(extension: &str) -> FilePathMatcher {
    FilePathMatcher::Extension(extension)
}

pub fn contain_file_name(name: &str) -> TreeMatcher {
    TreeMatcher::Contain(name)
}

pub fn contain_all_file_names<'a>(names: &'a [&'a str]) -> TreeMatcher<'a> {
    TreeMatcher::ContainAll(names)
}

pub fn contain_any_file_names<'a>(names: &'a [&'a str]) -> TreeMatcher<'a> {
    TreeMatcher::ContainAny(names)
}

#[cfg(all(test, feature = "file"))]
mod file_type_tests {
    use std::fs::File;
    use std::io::Write;

    use tempdir::TempDir;

    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::file::{
        be_a_directory, be_a_file, be_readonly, be_writable, be_zero_sized,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_be_a_directory() {
        let temporary_directory = TempDir::new(".").unwrap();
        let matcher = be_a_directory();
        matcher.test(&temporary_directory).passed.should_be_true();
    }

    #[test]
    fn should_be_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_a_file();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_zero_sized() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_zero_sized();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_a_zero_sized_file_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let mut file = File::create(file_path.clone()).unwrap();
        writeln!(file, "test content").unwrap();

        let matcher = be_zero_sized();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_readonly() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let file = File::create(file_path.clone()).unwrap();

        let mut readonly_permission = file.metadata().unwrap().permissions();
        readonly_permission.set_readonly(true);

        file.set_permissions(readonly_permission).unwrap();

        let matcher = be_readonly();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_readonly_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_readonly();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_writable() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_writable();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_writable_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let file = File::create(file_path.clone()).unwrap();

        let mut readonly_permission = file.metadata().unwrap().permissions();
        readonly_permission.set_readonly(true);

        file.set_permissions(readonly_permission).unwrap();

        let matcher = be_writable();
        matcher.test(&path).passed.should_be_true();
    }
}

#[cfg(all(test, feature = "file"))]
mod file_path_tests {
    use std::path::Path;

    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::file::{be_absolute, be_relative, have_extension};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_absolute() {
        let path = Path::new("/etc/conf.d");
        let matcher = be_absolute();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_relative() {
        let path = Path::new(".");
        let matcher = be_relative();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_have_extension() {
        let path = Path::new("/etc/sample.txt");
        let matcher = have_extension("txt");
        matcher.test(&path).passed.should_be_true();
    }
}

#[cfg(all(test, feature = "file"))]
mod walk_tree_tests {
    use std::fs::File;

    use tempdir::TempDir;

    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::file::{
        contain_all_file_names, contain_any_file_names, contain_file_name,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_contain_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_file_name(&"junit.txt");
        matcher.test(&directory_path).passed.should_be_true();
    }

    #[test]
    fn should_contain_all_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_all_file_names(&["junit.txt", "assert4rs.txt"]);
        matcher.test(&directory_path).passed.should_be_true();
    }

    #[test]
    fn should_contain_any_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_any_file_names(&["junit.txt", "assert4rs.txt"]);
        matcher.test(&directory_path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_any_files_but_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_any_file_names(&["assert.txt", "assert.txt"]);
        matcher.test(&directory_path).passed.should_be_true();
    }
}
