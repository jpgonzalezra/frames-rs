use crate::{
    types::{
        errors::{Error, ErrorCode, FrameErrors},
        image::{AspectRatio, FrameImage},
    },
    URL_REGEX,
};

impl FrameImage {
    pub fn validate(&self) -> Result<(), FrameErrors> {
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

        if self.aspect_ratio == AspectRatio::Error {
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
