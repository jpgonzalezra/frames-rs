use crate::types::{
    button::FrameButton,
    errors::{Error, ErrorCode, FrameErrors},
};

impl FrameButton {
    const VALID_ACTIONS: [&'static str; 4] = ["post_redirect", "post", "mint", "link"];

    pub fn validate(&self) -> Result<(), FrameErrors> {
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
