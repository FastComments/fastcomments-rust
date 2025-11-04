use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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