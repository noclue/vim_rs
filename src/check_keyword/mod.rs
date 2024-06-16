//! A trait for String-like types to check if a string is a reserved keyword,
//! and convert it to a safe non-keyword if so.
//!
//! Only strict and reserved keywords are checked against; weak keywords are not included.
//!
//! You can add this dependency with:
//!
//! ```toml
//! [dependencies]
//! check_keyword = "0.2"
//! ```
//!
//! # Example
//!
//! ```
//! use check_keyword::CheckKeyword;
//! let keyword = "match";
//!
//! assert!(keyword.is_keyword());
//! assert_eq!(keyword.into_safe(), "r#match");
//! ```
//! 
//! The [CheckKeyword::into_safe] method automatically checks [CheckKeyword::is_keyword] for you.
//! You don't need to call [CheckKeyword::is_keyword]
//! if you don't care whether it was originally a keyword or not.
//! 
//! # Implementations
//! 
//! There is a special implementation of `CheckKeyword<String>` for [&str], and a
//! blanket implementation of `CheckKeyword<T>` where `T: AsRef<str> + From<String>`.
//! 
//! The blanket implementation covers [String], and is only tested for that, but should
//! cover any other String-like type as well. I can try to broaden the definition to fit
//! other types if needed (open an issue).
//! 
//! # Rust Editions
//!
//! By default, the keywords added in Rust Edition 2018 are included in the list of checked keywords.
//! This can be disabled with `default-features = false` in your Cargo.toml.
//!
//! ```toml
//! [dependencies]
//! check_keyword = { version = "0.2", default-features = false }
//! ```
//!
//! Future Rust editions may add new keywords, and this crate will be updated to reflect that.
//! (Or you can create an issue on github if I don't.)
mod impls;

#[macro_use] mod arr_macro;

/// The main trait.
/// 
/// The generic argument `T` is the output type of `into_safe`, and in the blanket implementation
/// is equal to `Self`. I would have used an associated type,
/// but I ran into the good-old "upstream crates may add new impl of trait" error when implementing [str].
pub trait CheckKeyword<T> {
    /// Checks if `self` is a keyword.
    fn is_keyword(&self) -> bool;

    /// Checks if `self` is one of crate, self, super, Self that cannot be treated as raw identifiers.
    fn is_raw_restricted(&self) -> bool;

    /// If its a keyword, add "r#" to the beginning.
    /// 
    /// This function consumes self, so that if it is not a keyword,
    /// it can return quickly without cloning. If you want to keep ownership
    /// of the original data, clone it first.
    fn into_safe(self) -> T;
}

arr!(static KEYWORDS: [&'static str; _] = [

    // STRICT, 2015

    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",

    // STRICT, 2018

    #[cfg(feature = "2018")] "async",
    #[cfg(feature = "2018")] "await",
    #[cfg(feature = "2018")] "dyn",

    // RESERVED, 2015

    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",

    // RESERVED, 2018

    #[cfg(feature = "2018")] "try"

]);

// Keywords that cannot be treated as raw identifiers.
// See https://doc.rust-lang.org/reference/identifiers.html
arr!(static RAW_RESTRICTED: [&'static str; _] = [
    "crate",
    "self",
    "super",
    "Self",
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_length() {
        println!("Number of keywords: {}", KEYWORDS.len());
    }
}