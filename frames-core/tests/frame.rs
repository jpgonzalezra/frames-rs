#[cfg(test)]
mod tests {
    use frames_core::serializers::HtmlSerializer;
    use frames_core::types::button::FrameButton;
    use frames_core::types::errors::{Error, ErrorCode, FrameErrors};
    use frames_core::types::frame::Frame;
    use frames_core::types::image::{AspectRatio, FrameImage};

    #[test]
    fn it_parses_frame_html_correctly() {
        let html = r#"
            <title>Example</title>
            <meta name="fc:frame" content="vNext" />
            <meta name="fc:frame:image" content="http://example.com/image.png" />
            <meta name="fc:frame:button:1" content="Green" />
            <meta name="fc:frame:button:2" content="Purple" />
            <meta name="fc:frame:button:3" content="Red" />
            <meta name="fc:frame:button:4" content="Blue" />
            <meta name="fc:frame:post_url" content="https://example.com" />
            <meta name="fc:frame:input:text" content="Enter a message" />
        "#;

        let expected_frame = &mut Frame {
            title: "Example".to_string(),
            version: "vNext".to_string(),
            image: FrameImage {
                url: "http://example.com/image.png".to_string(),
                aspect_ratio: AspectRatio::None,
            },
            buttons: vec![
                FrameButton {
                    id: 1,
                    label: "Green".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 2,
                    label: "Purple".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 3,
                    label: "Red".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 4,
                    label: "Blue".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
            ],
            post_url: Some("https://example.com".to_string()),
            input_text: Some("Enter a message".to_string()),
        };

        let mut frame_container = Frame::new();
        let frame = frame_container.from_html(html).unwrap();

        frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        expected_frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(frame, expected_frame);
    }

    #[test]
    fn frame_to_html_test() {
        let expected_frame = Frame {
            title: "Example".to_string(),
            version: "vNext".to_string(),
            image: FrameImage {
                url: "http://example.com/image.png".to_string(),
                aspect_ratio: AspectRatio::None,
            },
            buttons: vec![
                FrameButton {
                    id: 1,
                    label: "Green".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 2,
                    label: "Purple".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 3,
                    label: "Red".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 4,
                    label: "Blue".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
            ],
            post_url: Some("https://example.com".to_string()),
            input_text: Some("Enter a message".to_string()),
        };

        let html_result = expected_frame.to_html();

        let expected_html = r#"<title>Example</title><meta name="fc:frame" content="vNext" /><meta name="fc:frame:image" content="http://example.com/image.png" /><meta name="fc:frame:input:text" content="Enter a message" /><meta name="fc:frame:button:1" content="Green" /><meta name="fc:frame:button:2" content="Purple" /><meta name="fc:frame:button:3" content="Red" /><meta name="fc:frame:button:4" content="Blue" /><meta name="fc:frame:post_url" content="https://example.com" />"#;
        assert_eq!(html_result, expected_html);
    }

    #[test]
    fn it_parses_frame_html_from_request_correctly() {
        let expected_frame = &mut Frame {
            title: "PHELMS".to_string(),
            version: "vNext".to_string(),
            image: FrameImage {
                url: "https://pheml.vercel.app/banner.png".to_string(),
                aspect_ratio: AspectRatio::None,
            },
            buttons: vec![FrameButton {
                id: 1,
                label: "Reveal my PHELM".to_string(),
                action: Some("post".to_string()),
                target: None,
            }],
            post_url: Some("https://pheml.vercel.app/api/frame".to_string()),
            input_text: None,
        };

        let mut frame_container = Frame::new();
        let frame = frame_container.from_url("https://pheml.vercel.app").unwrap();

        frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        expected_frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(frame, expected_frame);
    }

    #[test]
    fn it_parses_frame_with_action_button_correctly() {
        let html = r#"
             <title>PHELMS</title>
             <meta name="fc:frame" content="vNext"/>
             <meta name="fc:frame:image" content="https://pheml.vercel.app/banner.png"/>
             <meta name="fc:frame:post_url" content="https://pheml.vercel.app/api/frame"/>
             <meta name="fc:frame:button:1:action" content="post"/>
             <meta name="fc:frame:button:1" content="Reveal my PHELM"/>
             <meta name="fc:frame:button:2" content="Reveal my PHELM2"/>
             <meta name="fc:frame:button:2:action" content="post"/>
        "#;

        let expected_frame = &mut Frame {
            title: "PHELMS".to_string(),
            version: "vNext".to_string(),
            image: FrameImage {
                url: "https://pheml.vercel.app/banner.png".to_string(),
                aspect_ratio: AspectRatio::None,
            },
            buttons: vec![
                FrameButton {
                    id: 1,
                    label: "Reveal my PHELM".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    id: 2,
                    label: "Reveal my PHELM2".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
            ],
            post_url: Some("https://pheml.vercel.app/api/frame".to_string()),
            input_text: None,
        };

        let mut frame_container = Frame::new();
        let frame = frame_container.from_html(html).unwrap();

        frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        expected_frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(frame, expected_frame);
    }

    #[test]
    fn it_parses_frame_with_image_1_1_aspect_ratio_correctly() {
        let html = r#"
             <title>PHELMS</title>
             <meta name="fc:frame" content="vNext"/>
             <meta name="fc:frame:image" content="https://pheml.vercel.app/banner.png"/>
             <meta name="fc:frame:image:aspect_ratio" content="1:1"/>
             <meta name="fc:frame:post_url" content="https://pheml.vercel.app/api/frame"/>
             <meta name="fc:frame:button:1" content="Reveal my PHELM"/>
        "#;

        let expected_frame = &mut Frame {
            title: "PHELMS".to_string(),
            version: "vNext".to_string(),
            image: FrameImage {
                url: "https://pheml.vercel.app/banner.png".to_string(),
                aspect_ratio: AspectRatio::OneToOne,
            },
            buttons: vec![FrameButton {
                id: 1,
                label: "Reveal my PHELM".to_string(),
                action: Some("post".to_string()),
                target: None,
            }],
            post_url: Some("https://pheml.vercel.app/api/frame".to_string()),
            input_text: None,
        };

        let mut frame_container = Frame::new();
        let frame = frame_container.from_html(html).unwrap();

        frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        expected_frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(frame, expected_frame);
    }

    #[test]
    fn it_returns_an_error_for_invalid_frame_image_url() {
        let html = r#"
            <title>Example</title>
            <meta name="fc:frame" content="vNext" />
            <meta name="fc:frame:image" content="htt://example.com/image.png" />
            <meta name="fc:frame:post_url" content="https://example.com" />
            <meta name="fc:frame:input:text" content="Enter a message" />
        "#;

        let mut frame_container = Frame::new();
        let result = frame_container.from_html(html);

        assert!(result.is_err(), "Expected an error due to invalid URL in image metadata");

        let errors = result.err().unwrap();
        let expected_error = Error {
            description: "The URL provided is invalid.".to_string(),
            code: ErrorCode::InvalidURL,
            key: Some("fc:frame:image".to_string()),
        };

        assert!(
            errors.errors.contains(&expected_error),
            "Expected errors to include an invalid URL error"
        );
    }

    #[test]
    fn it_returns_an_error_for_invalid_button_action() {
        let html = r#"
             <title>PHELMS</title>
             <meta name="fc:frame" content="vNext"/>
             <meta name="fc:frame:image" content="https://pheml.vercel.app/banner.png"/>
             <meta name="fc:frame:post_url" content="https://pheml.vercel.app/api/frame"/>
             <meta name="fc:frame:button:1" content="Reveal my PHELM"/>
             <meta name="fc:frame:button:1:action" content="invalid_content"/>
        "#;

        let mut frame_container = Frame::new();
        let errors = frame_container.from_html(html).err().unwrap();

        let mut expected_errors = FrameErrors::new();
        expected_errors.add_error(Error {
            code: ErrorCode::InvalidButtonAction,
            description: "Invalid button action specified".to_string(),
            key: Some("fc:frame:button:1:action".to_string()),
        });
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn it_returns_an_for_invalid_index_sequence() {
        let html = r#"
             <title>PHELMS</title>
             <meta name="fc:frame" content="vNext"/>
             <meta name="fc:frame:image" content="https://pheml.vercel.app/banner.png"/>
             <meta name="fc:frame:post_url" content="https://pheml.vercel.app/api/frame"/>
             <meta name="fc:frame:button:1" content="Reveal my PHELM"/>
             <meta name="fc:frame:button:1:action" content="post"/>
             <meta name="fc:frame:button:3" content="Reveal my PHELM2"/>
             <meta name="fc:frame:button:3:action" content="post"/>
        "#;

        let mut frame_container = Frame::new();
        let errors = frame_container.from_html(html).err().unwrap();

        let mut expected_errors = FrameErrors::new();
        expected_errors.add_error(Error {
            description: "Button indices are not in a consecutive sequence starting from 1."
                .to_string(),
            code: ErrorCode::InvalidButtonSequence,
            key: Some("fc:frame:buttons".to_string()),
        });
        assert_eq!(errors, expected_errors);
    }
}
