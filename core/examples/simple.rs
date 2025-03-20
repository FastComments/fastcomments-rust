use fastcomments_core::sso::{fastcomments_sso::FastCommentsSSO, simple_sso_user_data::SimpleSSOUserData};



fn main() {
    // Create user data (usually client side, less secure)
    let user_data = SimpleSSOUserData::new(
        "username".to_string(),
        "email".to_string(),
        "avatar".to_string(),
    );

    // Create SSO configuration
    let sso = FastCommentsSSO::new(None, Some(user_data));

    let tenant_id = "tenant-123".to_string();
    let url_id = "123".to_string();
    let token = sso.create_token().unwrap();

    // Use comments API
    publicApi.getComments(tenant_id, url_id, token);
}