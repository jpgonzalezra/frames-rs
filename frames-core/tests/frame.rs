#[cfg(test)]
mod tests {
    use frames_core::{get_frame, Button, Frame};

    #[test]
    fn it_parses_frame_correctly() {
        let html = "
            <meta name=\"fc:frame\" content=\"vNext\" />
            <meta name=\"fc:frame:image\" content=\"http://example.com/image.png\" />
            <meta name=\"fc:frame:button:1\" content=\"Green\" />
            <meta name=\"fc:frame:button:2\" content=\"Purple\" />
            <meta name=\"fc:frame:button:3\" content=\"Red\" />
            <meta name=\"fc:frame:button:4\" content=\"Blue\" />
            <meta name=\"fc:frame:post_url\" content=\"https://example.com\" />
            <meta name=\"fc:frame:input:text\" content=\"Enter a message\" />
        ";

        let expected_frame = Frame {
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
        };

        assert_eq!(get_frame(html), expected_frame);
    }
}
