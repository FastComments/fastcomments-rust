use std::time;

use fastcomments_core::sso::{fastcomments_sso::FastCommentsSSO, secure_sso_payload::SecureSSOPayload, secure_sso_user_data::SecureSSOUserData};

fn main() {

    // User data for SSO
    // This should be done server side, DO NOT DO ON THE CLIENT
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),
        "email".to_string(),
        "username".to_string(),
        "avatar".to_string(),
    );

    let timestamp = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;

    // Create the SSO payload
    let secure_payload = SecureSSOPayload::new(
        serde_json::to_string(&user_data).unwrap(),
        "hash".to_string(),
        timestamp,
    );

    // Create SSO configuration
    let sso = FastCommentsSSO::new(Some(secure_payload), None);

    let tenant_id = "tenant-123".to_string();
    let url_id = "123".to_string();
    let token = sso.create_token().unwrap();

    // Use comments API
    publicApi.getComments(tenant_id, url_id, token);
}
