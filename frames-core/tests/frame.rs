#[cfg(test)]
mod tests {
    use frames_core::{AspectRatio, Frame, FrameButton, FrameImage};

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
                aspect_ratio: AspectRatio::OneToOne,
            },
            buttons: vec![
                FrameButton {
                    label: "Green".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    label: "Purple".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
                    label: "Red".to_string(),
                    action: Some("post".to_string()),
                    target: None,
                },
                FrameButton {
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

        assert_eq!(frame, expected_frame);
    }

    fn it_parses_complex_frame_html_correctly() {
        let html = r#"
             <title>PHELMS</title>
             <meta name="description" content="This is a PHelm, a special kind of SVG Helm generated on the fly to get a dynamic base64 meta:image on Frames."/>
             <meta name="fc:frame" content="vNext"/>
             <meta name="fc:frame:image" content="https://pheml.vercel.app/banner.png"/>
             <meta name="fc:frame:post_url" content="https://pheml.vercel.app/api/frame"/>
             <meta name="fc:frame:button:1" content="Reveal my PHELM"/>
             <meta name="fc:frame:button:1:action" content="post"/>
             <meta property="og:title" content="PHELMS"/>
             <meta property="og:description" content="This is a PHelm, a special kind of SVG Helm generated on the fly to get a dynamic base64 meta:image on Frames."/>
             <meta property="og:image" content="https://pheml.vercel.app/banner.png"/>
        "#;
    }
}
