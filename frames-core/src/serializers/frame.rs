use crate::types::frame::Frame;

use super::HtmlSerializer;

impl HtmlSerializer for Frame {
    fn to_html(&self) -> String {
        let mut html = String::new();
        html += &format!("<title>{}</title>", &self.title);
        html += &format!("<meta name=\"fc:frame\" content=\"{}\" />", &self.version);

        html += &self.image.to_html();

        if let Some(input_text) = &self.input_text {
            html += &format!("<meta name=\"fc:frame:input:text\" content=\"{}\" />", input_text);
        }

        for button in self.buttons.iter() {
            html += &button.to_html()
        }

        if let Some(post_url) = &self.post_url {
            html += &format!("<meta name=\"fc:frame:post_url\" content=\"{}\" />", post_url);
        }

        html.trim().to_string()
    }
}
