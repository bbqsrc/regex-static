pub use regex_static_macro::{lazy_regex, regex};

#[doc(hidden)]
pub use once_cell;

#[doc(hidden)]
pub use regex::Regex;

#[cfg(test)]
mod tests {
    use crate as regex_static;

    #[test]
    fn it_works() {
        let _regex = regex_static::regex!("^YESSSS (.*)$");
        let _regex = regex_static::lazy_regex!("^YESSSS (.*)$");
    }
}