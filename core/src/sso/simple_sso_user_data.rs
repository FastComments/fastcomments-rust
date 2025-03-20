use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SimpleSSOUserData {
    pub username: String,
    pub email: String,
    pub avatar: String,
    pub website_url: String,
}

impl SimpleSSOUserData {
    pub fn new(
        username: String,
        email: String,
        avatar: String,
        website_url: String,
    ) -> SimpleSSOUserData {
        SimpleSSOUserData {
            username,
            email,
            avatar,
            website_url,
        }
    }
}