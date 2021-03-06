pub use regex_static_macro::{lazy_regex, regex, static_regex};

#[doc(hidden)]
pub use once_cell;

#[doc(hidden)]
pub use regex::Regex;

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use regex::Regex;

    use crate as regex_static;

    #[test]
    fn it_works() {
        static _RE: Lazy<Regex> = regex_static::lazy_regex!("^yesss$");
        
        let _regex = regex_static::regex!("^YESSSS (.*)$");
        let _regex = regex_static::lazy_regex!("^YESSSS (.*)$");
        let _regex = regex_static::static_regex!("^YESSSS (.*)$");
    }
}
