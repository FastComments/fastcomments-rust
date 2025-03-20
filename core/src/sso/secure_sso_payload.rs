use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SecureSSOPayload {
    pub user_data_json_base64: String,
    pub verification_hash: String,
    pub timestamp: i64,
}

impl SecureSSOPayload {
    pub fn new(
        user_data_json_base64: String,
        verification_hash: String,
        timestamp: i64,
    ) -> SecureSSOPayload {
        SecureSSOPayload {
            user_data_json_base64,
            verification_hash,
            timestamp,
        }
    }
}