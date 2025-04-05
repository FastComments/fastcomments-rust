use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SimpleSSOUserData {
    /// If you don't set id, it defaults to their email.
    pub id: Option<String>,
    /// Their locale, for email notifications etc, in the format "en_us".
    pub locale: Option<String>,
    /// Set a non-unique name (since username must be unique within your tenant).
    pub display_name: Option<String>,
    /// Show a nice label with their comments, like "VIP User".
    pub display_label: Option<String>,
    /// Defaults to true when null.
    pub is_profile_activity_private: Option<bool>,
    /// This must be unique when paired with an email.
    pub username: String,
    /// The user's email.
    pub email: String,
    /// The user's avatar.
    pub avatar: String,
    /// The user's website, blog, or personal account page to show with their comments.
    pub website_url: Option<String>,
}

impl SimpleSSOUserData {
    pub fn new(
        username: String,
        email: String,
        avatar: String,
    ) -> SimpleSSOUserData {
        SimpleSSOUserData {
            username,
            email,
            avatar,
            ..Default::default()
        }
    }
}