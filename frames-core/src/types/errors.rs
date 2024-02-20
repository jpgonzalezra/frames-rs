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

impl Default for FrameErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for FrameErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(
                f,
                "{} - {} - {}",
                match error.code {
                    ErrorCode::InvalidURL => "The URL provided is invalid.",
                    ErrorCode::InvalidButtonAction => "Invalid button action specified",
                    ErrorCode::FailedToReadResponse => "Failed to read the response text from the URL provided. This may occur due to network issues, server errors, or the response being in an unexpected format.",
                    ErrorCode::FailedToFetchFrameHTML => "Failed to fetch frame HTML.",
                    ErrorCode::MissingTitle => "Please ensure a <title> tag is present within the HTML metadata for proper frame functionality.",
                    ErrorCode::InvalidButtonSequence => "Button indices are not in a consecutive sequence starting from 1.",
                    ErrorCode::InvalidAspectRadio => "Invalid Aspect Radio. (Must be either 1.91:1 or 1:1)"
                },
                error.description,
                error.key.clone().unwrap_or("".to_string())
            )?;
        }
        Ok(())
    }
}
