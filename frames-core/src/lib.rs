#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref URL_REGEX: Regex =
        Regex::new(r"^https?://(?:www\.)?[\w.-]+\.[a-zA-Z]{2,}(?:[/\?#][^\s]*)?$").unwrap();
}

pub mod provider;
pub mod types;
pub mod validators;
