use crate::types::button::FrameButton;

use super::HtmlSerializer;

impl HtmlSerializer for FrameButton {
    fn to_html(&self) -> String {
        let mut html =
            format!("<meta name=\"fc:frame:button:{}\" content=\"{}\" />", self.id, self.label);

        if let Some(action) = &self.action {
            html += &format!(
                "<meta name=\"fc:frame:button:{}:action\" content=\"{}\" />",
                self.id, action
            );
        }

        if let Some(target) = &self.target {
            html += &format!(
                "<meta name=\"fc:frame:button:{}:target\" content=\"{}\" />",
                self.id, target
            );
        }

        html
    }
}
