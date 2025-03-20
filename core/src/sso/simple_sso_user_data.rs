use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SimpleSSOUserData {
    pub username: String,
    pub email: String,
    pub avatar: String,
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
            website_url: None,
        }
    }
}