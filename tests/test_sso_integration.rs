//! Integration tests for FastComments SSO with real API calls
//!
//! These tests require a running FastComments server or valid API credentials.
//! Run with: FASTCOMMENTS_API_KEY=... FASTCOMMENTS_TENANT_ID=... cargo test

mod common;

use fastcomments_rust::{FastCommentsSSO, SecureSSOUserData, SimpleSSOUserData};
use common::TestEnv;

// Note: Integration tests will be skipped if generated client is not available
// Run update.sh first to generate the API client

#[tokio::test]
async fn test_secure_sso_api_integration() {
    let env = TestEnv::from_env().expect("Test environment not configured");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let user = SecureSSOUserData::new(
        format!("test-user-{}", timestamp),
        format!("test-{}@example.com", timestamp),
        format!("testuser{}", timestamp),
        "https://example.com/avatar.jpg".to_string(),
    );

    let sso = FastCommentsSSO::new_secure(env.api_key.clone(), &user)
        .expect("Failed to create secure SSO");

    let token = sso.create_token().expect("Failed to create token");

    // TODO: Make actual API call once generated client is integrated
    println!("Generated SSO token: {}", token);
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_simple_sso_api_integration() {
    let _env = TestEnv::from_env().expect("Test environment not configured");

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let user = SimpleSSOUserData::new(
        format!("simpleuser{}", timestamp),
        format!("simple-{}@example.com", timestamp),
        "https://example.com/simple-avatar.jpg".to_string(),
    );

    let sso = FastCommentsSSO::new_simple(user);
    let token = sso.create_token().expect("Failed to create token");

    // TODO: Make actual API call once generated client is integrated
    println!("Generated simple SSO token: {}", token);
    assert!(!token.is_empty());
}

#[test]
fn test_env_variables_present() {
    // This test just verifies that environment variables are set correctly
    let result = TestEnv::from_env();
    assert!(
        result.is_ok(),
        "Environment variables not set. Set FASTCOMMENTS_API_KEY and FASTCOMMENTS_TENANT_ID"
    );

    let env = result.unwrap();
    assert!(!env.api_key.is_empty(), "API key should not be empty");
    assert!(!env.tenant_id.is_empty(), "Tenant ID should not be empty");
    assert!(!env.base_url.is_empty(), "Base URL should not be empty");
}
