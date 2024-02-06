#[derive(Debug, PartialEq)]
pub struct Frame {
    pub version: String,
    pub image: String,
    pub buttons: Vec<Button>,
    pub post_url: String,
    pub input_text: String,
}

#[derive(Debug, PartialEq)]
pub struct Button {
    pub label: String,
    pub action: String,
}

pub fn get_frame(_html: &str) -> Frame {
    Frame {
        version: "vNext".to_string(),
        image: "http://example.com/image.png".to_string(),
        buttons: vec![
            Button { label: "Green".to_string(), action: "post".to_string() },
            Button { label: "Purple".to_string(), action: "post".to_string() },
            Button { label: "Red".to_string(), action: "post".to_string() },
            Button { label: "Blue".to_string(), action: "post".to_string() },
        ],
        post_url: "https://example.com".to_string(),
        input_text: "Enter a message".to_string(),
    }
}
