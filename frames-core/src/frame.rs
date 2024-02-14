use scraper::{Html, Selector};

extern crate scraper;

// use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct FrameErrors {
    pub errors: Vec<String>,
}

impl FrameErrors {
    fn new() -> Self {
        FrameErrors { errors: Vec::new() }
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    fn add_errors(&mut self, errors: Vec<String>) {
        for error in errors {
            self.errors.push(error);
        }
    }

    fn is_empty(&self) -> bool {
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

        // validate size (< 10 MB)
        // validate image (jpg, png, gif)

        // validate aspect_ratio
        match self.aspect_ratio {
            AspectRatio::OneToOne | AspectRatio::OnePointNineToOne => Ok(()),
            _ => {
                errors.add_error("Frame image ratio error".to_string());
                Err(errors)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FrameButton {
    pub label: String,
    pub action: Option<String>,
    pub target: Option<String>,
}

impl FrameButton {
    fn validate(&self) -> Result<(), FrameErrors> {
        let mut errors = FrameErrors::new();

        if self.label.len() > 256 {
            errors.add_error("Label exceeds 256 bytes".to_string());
        }

        // more validations

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub version: String,
    pub image: FrameImage,
    pub post_url: Option<String>,
    pub buttons: Vec<FrameButton>,
    pub input_text: Option<String>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
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

    pub fn from_html(&mut self, html: &str) -> Result<&mut Self, FrameErrors> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("meta").unwrap();

        for element in document.select(&selector) {
            if let Some(name) = element.value().attr("name") {
                if let Some(content) = element.value().attr("content") {
                    match name {
                        "fc:frame" => self.version = content.to_string(),
                        "fc:frame:image" => self.image.url = content.to_string(),
                        "fc:frame:post_url" => self.post_url = Some(content.to_string()),
                        "fc:frame:input:text" => self.input_text = Some(content.to_string()),
                        name if name.starts_with("fc:frame:button:") => {
                            let idx = name
                                .split(":")
                                .nth(3)
                                .unwrap_or_default()
                                .parse::<usize>()
                                .unwrap_or_default();
                            if idx > 0 && idx <= 4 {
                                let button = FrameButton {
                                    label: content.to_string(),
                                    action: None,
                                    target: None,
                                };
                                self.buttons.push(button);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        self.validate()?;
        Ok(self)
    }
}
