
///! Codify the rust convention on names from https://github.com/rust-lang/rust/blob/0bb6fec1c9aa484a7ec987a9e8ffca2eb647b0b3/src/doc/style-guide/src/advice.md
///! Also apply safe rust naming conventions 
///! We use:
///! - check_keyword to check if the name is a keyword and make it safe
///! - convert_case to convert the name as needed

use convert_case::{Case, Casing};
use check_keyword::CheckKeyword;

pub fn to_type_name(name: &str) -> String {
    name.to_case(Case::Pascal).into_safe()
}

pub fn to_enum_variant(name: &str) -> String {
    name.to_case(Case::Pascal).into_safe()
}

pub fn to_field_name(name: &str) -> String {
    name.to_case(Case::Snake).into_safe()
}

pub fn getter_name(name: &str) -> String {
    format!("get_{}", name.to_case(Case::Snake))
}

pub fn any_into_name(name: &str) -> String {
    format!("any_into_{}", name.to_case(Case::Snake))
}


pub fn to_fn_name(name: &str) -> String {
    name.to_case(Case::Snake).into_safe()
}
