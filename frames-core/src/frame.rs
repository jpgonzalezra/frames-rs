use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::URL_REGEX;

#[derive(Debug, PartialEq)]
pub struct FrameErrors {
    pub errors: Vec<String>,
}

impl FrameErrors {
    pub fn new() -> Self {
        FrameErrors { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_errors(&mut self, errors: Vec<String>) {
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
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum AspectRatio {
    OneToOne,
    OnePointNineToOne,
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
            errors.add_error("Invalid URL".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        // validate aspect_ratio
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct FrameButton {
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
                errors.add_error("Invalid button action specified".to_string());
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
            image: FrameImage { url: String::new(), aspect_ratio: AspectRatio::OneToOne },
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
                        errors.add_error("Failed to read response text".to_string());
                        return Err(errors);
                    }
                }
            }
            Err(_) => {
                let mut errors = FrameErrors::new();
                errors.add_error("Invalid frame html".to_string());
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
            errors.add_error("The title is mandatory".to_string())
        }

        let selector = Selector::parse("meta").unwrap();
        let mut temp_buttons: HashMap<usize, FrameButton> = HashMap::new();
        for element in document.select(&selector) {
            if let Some(name) = element.value().attr("name") {
                if let Some(content) = element.value().attr("content") {
                    match name {
                        "fc:frame" => self.version = content.to_string(),
                        "fc:frame:image" => self.image.url = content.to_string(),
                        "fc:frame:post_url" => self.post_url = Some(content.to_string()),
                        "fc:frame:input:text" => self.input_text = Some(content.to_string()),
                        name if name.starts_with("fc:frame:button:") => {
                            let parts: Vec<&str> = name.split(":").collect();
                            if let Ok(idx) = parts[3].parse::<usize>() {
                                match parts.get(4) {
                                    Some(&"action") => {
                                        if let Some(button) = temp_buttons.get_mut(&idx) {
                                            button.action = Some(content.to_string());
                                        } else {
                                            let button = FrameButton {
                                                label: content.to_string(),
                                                action: Some(content.to_string()),
                                                target: None,
                                            };
                                            temp_buttons.insert(idx, button);
                                        }
                                    }
                                    _ => {
                                        let button = FrameButton {
                                            label: content.to_string(),
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
        self.buttons.extend(temp_buttons.into_values());

        match self.validate() {
            Ok(_) => (),
            Err(e) => {
                errors.add_errors(e.errors);
                return Err(errors);
            }
        }
        Ok(self)
    }
}
