#[derive(Debug, PartialEq)]
pub enum AspectRatio {
    None,
    OneToOne,
    OnePointNineToOne,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct FrameImage {
    pub url: String,
    pub aspect_ratio: AspectRatio,
}
