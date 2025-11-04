use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SecureSSOUserData {
    pub id: String,
    pub email: String,
    pub username: String,
    pub avatar: String,
    pub opted_in_notifications: Option<bool>,
    pub display_label: Option<String>,
    pub display_name: Option<String>,
    pub website_url: Option<String>,
    pub group_ids: Option<Vec<String>>,
    pub is_admin: Option<bool>,
    pub is_moderator: Option<bool>,
    pub is_profile_activity_private: Option<bool>,
}

impl SecureSSOUserData {
    pub fn new(id: String, email: String, username: String, avatar: String) -> SecureSSOUserData {
        SecureSSOUserData {
            id,
            email,
            username,
            avatar,
            opted_in_notifications: None,
            display_label: None,
            display_name: None,
            website_url: None,
            group_ids: None,
            is_admin: None,
            is_moderator: None,
            is_profile_activity_private: None,
        }
    }

    pub fn as_json_base64(&self) -> String {

        let json = serde_json::to_string(self).unwrap();
        
        BASE64_STANDARD.encode(&json)
    }
}