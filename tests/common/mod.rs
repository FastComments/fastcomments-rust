//! Common test utilities and fixtures

use std::env;

pub struct TestEnv {
    pub api_key: String,
    pub tenant_id: String,
    pub base_url: String,
}

impl TestEnv {
    pub fn from_env() -> Result<Self, String> {
        let api_key = env::var("FASTCOMMENTS_API_KEY")
            .map_err(|_| "FASTCOMMENTS_API_KEY environment variable not set".to_string())?;

        let tenant_id = env::var("FASTCOMMENTS_TENANT_ID")
            .map_err(|_| "FASTCOMMENTS_TENANT_ID environment variable not set".to_string())?;

        let base_url = env::var("FASTCOMMENTS_BASE_URL")
            .unwrap_or_else(|_| "https://fastcomments.com".to_string());

        Ok(TestEnv {
            api_key,
            tenant_id,
            base_url,
        })
    }
}

#[allow(dead_code)]
pub struct TestUserData {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub avatar: String,
}

#[allow(dead_code)]
impl TestUserData {
    pub fn new() -> Self {
        Self {
            user_id: "test-user-123".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            avatar: "https://example.com/avatar.jpg".to_string(),
        }
    }
}

impl Default for TestUserData {
    fn default() -> Self {
        Self::new()
    }
}
