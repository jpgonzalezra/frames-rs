use crate::types::{
    button::FrameButton,
    image::{AspectRatio, FrameImage},
};

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub title: String,
    pub version: String,
    pub image: FrameImage,
    pub post_url: Option<String>,
    pub buttons: Vec<FrameButton>,
    pub input_text: Option<String>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            title: String::new(),
            version: String::new(),
            image: FrameImage { url: String::new(), aspect_ratio: AspectRatio::None },
            post_url: None,
            buttons: Vec::new(),
            input_text: None,
        }
    }
}
