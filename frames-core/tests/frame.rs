#[cfg(test)]
mod tests {
    use frames_core::{get_frame, Button, Frame};

    #[test]
    fn it_parses_frame_html_correctly() {
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
                Button {
                    index: 1,
                    label: "Green".to_string(),
                    action: "post".to_string(),
                    target: None,
                },
                Button {
                    index: 2,
                    label: "Purple".to_string(),
                    action: "post".to_string(),
                    target: None,
                },
                Button {
                    index: 3,
                    label: "Red".to_string(),
                    action: "post".to_string(),
                    target: None,
                },
                Button {
                    index: 4,
                    label: "Blue".to_string(),
                    action: "post".to_string(),
                    target: None,
                },
            ],
            post_url: "https://example.com".to_string(),
            input_text: "Enter a message".to_string(),
        };

        let (frame, errors) = validate_frame_html(html);

        assert_eq!(frame.unwrap(), expected_frame);
        assert_eq!(errors.len(), 0);
    }
}
