#[macro_use]
extern crate lazy_static;
extern crate regex;

lazy_static! {
    static ref URL_REGEX: Regex =
        Regex::new(r"^https?://(?:www\.)?[\w.-]+\.[a-zA-Z]{2,}(?:[/\?#][^\s]*)?$").unwrap();
}

mod frame;
use regex::Regex;

pub use crate::frame::{AspectRatio, Frame, FrameButton, FrameErrors, FrameImage};

mod address;
pub use crate::address::get_custody_address_by_fid;
