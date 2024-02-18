#[cfg(test)]
mod tests {
    use frames_core::{AspectRatio, Error, ErrorCode, Frame, FrameButton, FrameErrors, FrameImage};

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
    fn it_parses_html_from_request_correctly() {
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

    // #[test]
    // fn it_parses_complex_html_from_request_correctly() {
    //     let expected_frame = &mut Frame {
    //         title: "PHELMS".to_string(),
    //         version: "vNext".to_string(),
    //         image: FrameImage {
    //             url: "https://pheml.vercel.app/banner.png".to_string(),
    //             aspect_ratio: AspectRatio::None,
    //         },
    //         buttons: vec![FrameButton {
    //             label: "Reveal my PHELM".to_string(),
    //             action: Some("post".to_string()),
    //             target: "eip155:8453:0xf5a3b6dee033ae5025e4332695931cadeb7f4d2b:1",
    //         }],
    //         post_url: Some("https://pheml.vercel.app/api/frame".to_string()),
    //         input_text: None,
    //     };

    //     let mut frame_container = Frame::new();
    //     let frame = frame_container.from_url("https://mint.farcaster.xyz").unwrap();

    //     frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
    //     expected_frame.buttons.sort_by(|a, b| a.label.cmp(&b.label));
    //     assert_eq!(frame, expected_frame);
    // }

    #[test]
    fn it_parses_button_with_action_correctly() {
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
    fn it_returns_an_frame_image_error() {
        let html = r#"
            <title>Example</title>
            <meta name="fc:frame" content="vNext" />
            <meta name="fc:frame:image" content="htt://example.com/image.png" />
            <meta name="fc:frame:post_url" content="https://example.com" />
            <meta name="fc:frame:input:text" content="Enter a message" />
        "#;

        let mut frame_container = Frame::new();
        let errors = frame_container.from_html(html).err().unwrap();

        let mut expected_errors = FrameErrors::new();
        expected_errors.add_error(Error {
            description: "The URL provided is invalid.".to_string(),
            code: ErrorCode::InvalidURL,
            key: Some("fc:frame:image".to_string()),
        });
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn it_returns_an_button_error() {
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
    fn it_returns_an_button_error_ii() {
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
