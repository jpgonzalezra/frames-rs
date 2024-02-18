use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::URL_REGEX;

#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    InvalidURL,
    InvalidButtonAction,
    FailedToReadResponse,
    FailedToFetchFrameHTML,
    MissingTitle,
    InvalidButtonSequence,
    InvalidAspectRadio,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub description: String,
    pub code: ErrorCode,
    pub key: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct FrameErrors {
    pub errors: Vec<Error>,
}

impl FrameErrors {
    pub fn new() -> Self {
        FrameErrors { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn add_errors(&mut self, errors: Vec<Error>) {
        for error in errors {
            self.errors.push(error);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl std::fmt::Display for FrameErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(
                f,
                "{} - {} - {}",
                error.key.clone().unwrap_or("".to_string()),
                match error.code {
                    ErrorCode::InvalidURL => "The URL provided is invalid.",
                    ErrorCode::InvalidButtonAction => "Invalid button action specified",
                    ErrorCode::FailedToReadResponse => "Failed to read the response text from the URL provided. This may occur due to network issues, server errors, or the response being in an unexpected format.",
                    ErrorCode::FailedToFetchFrameHTML => "Failed to fetch frame HTML.",
                    ErrorCode::MissingTitle => "Please ensure a <title> tag is present within the HTML metadata for proper frame functionality.",
                    ErrorCode::InvalidButtonSequence => "Button indices are not in a consecutive sequence starting from 1.", 
                    ErrorCode::InvalidAspectRadio => "Invalid Aspect Radio. (Must be either 1.91:1 or 1:1)"
                },
                error.description
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum AspectRatio {
    OneToOne,
    OnePointNineToOne,
    None,
}

#[derive(Debug, PartialEq)]
pub struct FrameImage {
    pub url: String,
    pub aspect_ratio: AspectRatio,
}

impl FrameImage {
    fn validate(&self) -> Result<(), FrameErrors> {
        let mut errors = FrameErrors::new();

        // validate image (jpg, png, gif)

        // validate image url
        if !URL_REGEX.is_match(&self.url) {
            let error = Error {
                description: "The URL provided is invalid.".to_string(),
                code: ErrorCode::InvalidURL,
                key: Some("fc:frame:image".to_string()),
            };
            errors.add_error(error);
        }

        if self.aspect_ratio == AspectRatio::None {
            let error = Error {
                description: "Invalid image aspect ratio.".to_string(),
                code: ErrorCode::InvalidAspectRadio,
                key: Some("fc:frame:image:aspect_ratio".to_string()),
            };
            errors.add_error(error);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct FrameButton {
    pub id: usize,
    pub label: String,
    pub action: Option<String>,
    pub target: Option<String>,
}

impl FrameButton {
    const VALID_ACTIONS: [&'static str; 4] = ["post_redirect", "post", "mint", "link"];

    fn validate(&self) -> Result<(), FrameErrors> {
        let mut errors = FrameErrors::new();

        match &self.action {
            Some(action) if !Self::VALID_ACTIONS.contains(&action.as_str()) => {
                let error = Error {
                    code: ErrorCode::InvalidButtonAction,
                    description: "Invalid button action specified".to_string(),
                    key: Some(format!("fc:frame:button:{}:action", self.id)),
                };
                errors.add_error(error);
            }
            _ => {}
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub title: String,
    pub version: String,
    pub image: FrameImage,
    pub post_url: Option<String>,
    pub buttons: Vec<FrameButton>,
    pub input_text: Option<String>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            title: String::new(),
            version: String::new(),
            image: FrameImage { url: String::new(), aspect_ratio: AspectRatio::None },
            post_url: None,
            buttons: Vec::new(),
            input_text: None,
        }
    }

    fn validate(&self) -> Result<(), FrameErrors> {
        let mut errors = FrameErrors::new();

        match self.image.validate() {
            Ok(_) => (),
            Err(e) => errors.add_errors(e.errors),
        }

        for button in &self.buttons {
            match button.validate() {
                Ok(_) => (),
                Err(e) => errors.add_errors(e.errors),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    pub fn from_url(&mut self, url: &str) -> Result<&mut Self, FrameErrors> {
        let response = reqwest::blocking::get(url);
        match response {
            Ok(body) => {
                let text = body.text();
                match text {
                    Ok(html) => self.from_html(&html),
                    Err(_) => {
                        let mut errors = FrameErrors::new();
                        let error = Error {
                            description: "Failed to read the response text from the URL provided. This may occur due to network issues, server errors, or the response being in an unexpected format."
                            .to_string(),
                            code: ErrorCode::FailedToReadResponse,
                            key: None,
                        };
                        errors.add_error(error);
                        return Err(errors);
                    }
                }
            }
            Err(_) => {
                let mut errors = FrameErrors::new();
                let error = Error {
                    description: "Failed to fetch frame HTML.".to_string(),
                    code: ErrorCode::FailedToFetchFrameHTML,
                    key: None,
                };
                errors.add_error(error);
                return Err(errors);
            }
        }
    }

    pub fn from_html(&mut self, html: &str) -> Result<&mut Self, FrameErrors> {
        let document = Html::parse_document(html);
        let mut errors = FrameErrors::new();

        let title_selector = Selector::parse("title").unwrap();
        if let Some(title_element) = document.select(&title_selector).next() {
            let title_text = title_element.text().collect::<Vec<_>>().join("");
            self.title = title_text
        } else {
            let error = Error {
                description: "Please ensure a <title> tag is present within the HTML metadata for proper frame functionality..".to_string(),
                code: ErrorCode::MissingTitle,
                key: None,
            };
            errors.add_error(error);
        }

        let selector = Selector::parse("meta").unwrap();
        let mut temp_buttons: HashMap<usize, FrameButton> = HashMap::new();
        for element in document.select(&selector) {
            if let Some(name) = element.value().attr("name") {
                if let Some(_content) = element.value().attr("content") {
                    let content = _content.to_string();
                    match name {
                        "fc:frame" => self.version = content,
                        "fc:frame:image" => self.image.url = content,
                        "fc:frame:image:aspect_ratio" => {
                            self.image.aspect_ratio = match _content {
                                "1.91:1" => AspectRatio::OnePointNineToOne,
                                "1:1" => AspectRatio::OneToOne,
                                _ => AspectRatio::None,
                            }
                        }
                        "fc:frame:post_url" => self.post_url = Some(content),
                        "fc:frame:input:text" => self.input_text = Some(content),
                        name if name.starts_with("fc:frame:button:") => {
                            let parts: Vec<&str> = name.split(":").collect();
                            if let Ok(idx) = parts[3].parse::<usize>() {
                                match parts.get(4) {
                                    Some(&"action") => {
                                        if let Some(button) = temp_buttons.get_mut(&idx) {
                                            button.action = Some(content);
                                        } else {
                                            let button = FrameButton {
                                                id: idx,
                                                label: content.clone(),
                                                action: Some(content),
                                                target: None,
                                            };
                                            temp_buttons.insert(idx, button);
                                        }
                                    }
                                    _ => {
                                        let button = FrameButton {
                                            id: idx,
                                            label: content,
                                            action: Some("post".to_string()),
                                            target: None,
                                        };
                                        temp_buttons.insert(idx, button);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        match self.add_buttons_if_apply(temp_buttons) {
            Ok(buttons) => self.buttons.extend(buttons),
            Err(errs) => errors.add_errors(errs.errors),
        };

        match self.validate() {
            Ok(_) => (),
            Err(e) => {
                errors.add_errors(e.errors);
                return Err(errors);
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(self)
    }

    fn add_buttons_if_apply(
        &mut self,
        temp_buttons: HashMap<usize, FrameButton>,
    ) -> Result<Vec<FrameButton>, FrameErrors> {
        let mut errors = FrameErrors::new();
        let mut buttons: Vec<FrameButton> = Vec::new();

        let mut indices: Vec<usize> = temp_buttons.keys().cloned().collect();
        indices.sort();

        let valid_sequence = if indices.len() == 0 || indices.len() == 1 {
            true
        } else {
            indices[0] == 1 && indices.windows(2).all(|w| w[0] + 1 == w[1])
        };

        if valid_sequence {
            buttons.extend(temp_buttons.into_values());
        } else {
            let error = Error {
                description: "Button indices are not in a consecutive sequence starting from 1."
                    .to_string(),
                code: ErrorCode::InvalidButtonSequence,
                key: Some("fc:frame:buttons".to_string()),
            };
            errors.add_error(error);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(buttons)
    }
}
