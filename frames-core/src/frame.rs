extern crate scraper;

use scraper::{ElementRef, Html, Selector};

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
    pub index: usize,
    pub label: String,
    pub action: String,
    pub target: Option<String>,
}

pub fn validate_frame_html(html_string: &str) -> (Option<Frame>, Vec<String>) {
    let document = Html::parse_document(html_string);
    let mut errors: Vec<String> = vec![];

    let version_result =
        get_content_from_meta(&document, "meta[property='fc:frame'], meta[name='fc:frame']");

    // TODO: check bytes
    let post_url_result = get_content_from_meta(
        &document,
        "meta[property='fc:frame:post_url'], meta[name='fc:frame:post_url']",
    );

    // TODO: validate images
    let image_result = get_content_from_meta(
        &document,
        "meta[property='fc:frame:image'], meta[name='fc:frame:image']",
    );

    // TODO: validate images
    let input_text_result = get_content_from_meta(
        &document,
        "meta[property='fc:frame:input:text'], meta[name='fc:frame:input:text']",
    );

    let button_selector_result = Selector::parse(r#"meta[name^="fc:frame:button:"]"#);

    let button_actions = match button_selector_result {
        Ok(selector) => {
            let mut button_actions: Vec<Button> = Vec::new();
            for element in document.select(&selector) {
                if let Some(button) = parse_button_element(element) {
                    button_actions.push(button);
                }
            }
            button_actions
        }
        Err(_) => {
            errors.push(format!("Buttons parser error"));
            Vec::new()
        }
    };

    let mut collect_error = |result: Result<String, String>| -> Option<String> {
        match result {
            Ok(value) => Some(value),
            Err(e) => {
                errors.push(e);
                None
            }
        }
    };

    let version = collect_error(version_result);
    let post_url = collect_error(post_url_result);
    let image = collect_error(image_result);
    let input_text = collect_error(input_text_result);

    let frame = if errors.is_empty() {
        Some(Frame {
            version: version.unwrap(),
            image: image.unwrap(),
            buttons: button_actions,
            post_url: post_url.unwrap(),
            input_text: input_text.unwrap(),
        })
    } else {
        None
    };

    (frame, errors)
}

// Privates
fn get_content_from_meta(document: &Html, selector_str: &str) -> Result<String, String> {
    Selector::parse(selector_str)
        .map_err(|_| format!("Selector parser error: {}", selector_str))
        .and_then(|selector| {
            document
                .select(&selector)
                .next()
                .and_then(|e| e.value().attr("content"))
                .map(String::from)
                .ok_or_else(|| format!("No content found for selector: {}", selector_str))
        })
}

fn parse_button_element(element: ElementRef) -> Option<Button> {
    let name = element.value().attr("name")?;
    let content = element.value().attr("content")?;

    let parts: Vec<&str> = name.split(':').collect();
    if parts.len() < 4 {
        return None;
    }
    let index_str = parts[3];
    let index: usize = index_str.parse().ok()?;

    // FIXME: action button
    Some(Button { index, label: content.to_string(), action: "post".to_string(), target: None })
}
