use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AspectRatio {
    None,
    OneToOne,
    OnePointNineToOne,
    Error,
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AspectRatio::None => write!(f, "None"),
            AspectRatio::OneToOne => write!(f, "1:1"),
            AspectRatio::OnePointNineToOne => write!(f, "1.9:1"),
            AspectRatio::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FrameImage {
    pub url: String,
    pub aspect_ratio: AspectRatio,
}
