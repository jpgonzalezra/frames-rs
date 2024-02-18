#[macro_use]
extern crate lazy_static;
extern crate regex;

lazy_static! {
    static ref URL_REGEX: Regex =
        Regex::new(r"^https?://(?:www\.)?[\w.-]+\.[a-zA-Z]{2,}(?:[/\?#][^\s]*)?$").unwrap();
}

pub mod address;
pub mod types;

use regex::Regex;

pub use crate::address::get_custody_address_by_fid;
