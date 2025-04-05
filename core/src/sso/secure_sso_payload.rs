use hmac::{digest::InvalidLength, Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize)]
pub struct SecureSSOPayload {
    pub user_data_json_base64: String,
    pub verification_hash: String,
    pub timestamp: u64,
}

impl SecureSSOPayload {
    pub fn new(
        user_data_json_base64: String,
        verification_hash: String,
        timestamp: u64,
    ) -> SecureSSOPayload {
        SecureSSOPayload {
            user_data_json_base64,
            verification_hash,
            timestamp,
        }
    }
}