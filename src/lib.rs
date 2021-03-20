//! Compile-time validation of [`regex::Regex`](https://github.com/rust-lang/regex).
//! 
//! ## Examples
//! 
//! ### Lazy regex
//! 
//! Uses `once_cell` to lazily create the regex.
//! 
//! ```rust
//! static RE: Lazy<Regex> = regex_static::lazy_regex!("^yesss$");
//! ```
//! 
//! ### Static regex
//! 
//! Also uses `once_cell`, but works inline (will therefore reuse the same instance of the regex each function call).
//! 
//! ```rust
//! let some_regex = regex_static::static_regex!("^yesss$");
//! ```
//! 
//! ### Ordinary regex
//! 
//! Will create an owned `Regex`, just like calling `Regex::new(...)` but with compile-time validation.
//! 
//! ```rust
//! let ordinary_regex = regex_static::regex!("^yesss$");
//! ```

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
