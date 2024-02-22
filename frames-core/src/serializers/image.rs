use crate::types::image::{AspectRatio, FrameImage};

use super::HtmlSerializer;

impl HtmlSerializer for FrameImage {
    fn to_html(&self) -> String {
        let mut html = format!("<meta name=\"fc:frame:image\" content=\"{}\" />", self.url);
        if self.aspect_ratio == AspectRatio::OneToOne
            || self.aspect_ratio == AspectRatio::OnePointNineToOne
        {
            html += &format!(
                "<meta name=\"fc:frame:image:aspect_ratio\" content=\"{}\" />",
                self.aspect_ratio.to_string()
            );
        }
        html
    }
}
