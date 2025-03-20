use std::time;

use fastcomments_core::sso::{fastcomments_sso::FastCommentsSSO, secure_sso_payload::SecureSSOPayload, secure_sso_user_data::SecureSSOUserData};

fn main() {



    // User data
    // This should be done server side, DO NOT DO ON THE CLIENT
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),
        "email".to_string(),
        "username".to_string(),
        "avatar".to_string(),
    );

    // Create the SSO payload
    let timestamp = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let payload = SecureSSOPayload::new(
        serde_json::to_string(&user_data).unwrap(),
        "hash".to_string(),
        timestamp,
    );

    // Create SSO configuration
    let sso = FastCommentsSSO::new(payload);

    // 
}
