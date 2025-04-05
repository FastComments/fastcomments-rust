use fastcomments_client::apis::{
    configuration::Configuration,
};
use fastcomments_core::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};
use helpers::comments_params;
mod helpers;

#[tokio::main]
async fn main() {
    let api_key = "Your API key".to_string();

    // User data for SSO
    // This should be done server side, DO NOT DO ON THE CLIENT
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),
        "email".to_string(),
        "username".to_string(),
        "avatar".to_string(),
    );

    // Create the SSO config with a payload
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();

    let tenant_id = "tenant-123".to_string();
    let url_id = "123".to_string();
    let token = sso.create_token().unwrap();

    // Populate this with your site data
    let config = Configuration::new();

    // Try to get comments
    if let Ok(result) = fastcomments_client::apis::public_api::get_comments_public(
        &config,
        comments_params(tenant_id, url_id, Some(token)),
    )
    .await
    {
        // Now we can do something with the comments!
        let _comments = result.comments;
    };
}
