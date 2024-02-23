use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameActionPayload {
    #[serde(rename = "trustedData")]
    pub trusted_data: PayloadTrustedData,
    #[serde(rename = "untrustedData")]
    pub untrusted_data: PayloadUntrustedData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadTrustedData {
    #[serde(rename = "messageBytes")]
    pub message_bytes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadCastId {
    pub fid: i32,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadUntrustedData {
    pub fid: i32,
    pub url: String,
    #[serde(rename = "messageHash")]
    pub message_hash: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub timestamp: chrono::DateTime<Utc>,
    pub network: i32,
    #[serde(rename = "buttonIndex")]
    pub button_index: i32,
    #[serde(rename = "castId")]
    pub cast_id: PayloadCastId,
    #[serde(rename = "inputText")]
    pub input_text: Option<String>,
}
