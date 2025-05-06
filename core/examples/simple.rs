use fastcomments_client::apis::configuration::Configuration;
use fastcomments_core::sso::{fastcomments_sso::FastCommentsSSO, simple_sso_user_data::SimpleSSOUserData};
use helpers::comments_params;

mod helpers;

#[tokio::main]
async fn main() {
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

    // Populate this with your site data
    let config = Configuration::new();

    // Try to get comments
    let result = fastcomments_client::apis::public_api::get_comments_public(
        &config,
        comments_params(tenant_id, url_id, Some(token)),
    )
    .await;
    if let Ok(result) = result {
        // Now we can do something with the comments!
        let comments = result.comments;

        println!("Comments: {:#?}", comments);
    } else {
        println!("Failed to get comments {:#?}", result);
    };
}