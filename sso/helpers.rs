use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::CreateHashError;

type HmacSha256 = Hmac<Sha256>;

pub fn create_verification_hash(
    api_key: &str,
    timestamp: u64,
    user_data_json_base64: &str,
) -> Result<String, CreateHashError> {
    let mut mac = HmacSha256::new_from_slice(api_key.as_bytes())?;
    
    let message_str = format!("{}{}", timestamp, user_data_json_base64);
    mac.update(message_str.as_bytes());
    
    let bytes = mac.finalize().into_bytes();
    
    Ok(get_bytes_as_hex(&bytes))
}

fn get_bytes_as_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}