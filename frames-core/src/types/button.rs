#[derive(Debug, PartialEq)]
pub struct FrameButton {
    pub id: usize,
    pub label: String,
    pub action: Option<String>,
    pub target: Option<String>,
}
