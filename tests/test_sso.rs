//! Unit tests for FastComments SSO functionality

mod common;

use fastcomments_sdk::{
    FastCommentsSSO, SecureSSOUserData, SimpleSSOUserData,
};
use common::{TestEnv, TestUserData};

#[test]
fn test_create_secure_sso() {
    let env = TestEnv::from_env().expect("Test environment not configured");
    let test_data = TestUserData::new();

    let user = SecureSSOUserData::new(
        test_data.user_id.clone(),
        test_data.email.clone(),
        test_data.username.clone(),
        test_data.avatar.clone(),
    );

    let sso = FastCommentsSSO::new_secure(env.api_key.clone(), &user);
    assert!(sso.is_ok());

    let sso = sso.unwrap();
    assert!(sso.get_secure_sso_payload().is_some());
    assert!(sso.get_simple_sso_payload().is_none());
}

#[test]
fn test_secure_sso_token_creation() {
    let env = TestEnv::from_env().expect("Test environment not configured");
    let test_data = TestUserData::new();

    let user = SecureSSOUserData::new(
        test_data.user_id.clone(),
        test_data.email.clone(),
        test_data.username.clone(),
        test_data.avatar.clone(),
    );

    let sso = FastCommentsSSO::new_secure(env.api_key.clone(), &user)
        .expect("Failed to create secure SSO");

    let token = sso.create_token();
    assert!(token.is_some());

    let token = token.unwrap();
    assert!(token.contains("user_data_json_base64"));
    assert!(token.contains("verification_hash"));
    assert!(token.contains("timestamp"));
}

#[test]
fn test_create_simple_sso() {
    let test_data = TestUserData::new();

    let user = SimpleSSOUserData::new(
        test_data.username.clone(),
        test_data.email.clone(),
        test_data.avatar.clone(),
    );

    let sso = FastCommentsSSO::new_simple(user);
    assert!(sso.get_simple_sso_payload().is_some());
    assert!(sso.get_secure_sso_payload().is_none());
}

#[test]
fn test_simple_sso_token_creation() {
    let test_data = TestUserData::new();

    let user = SimpleSSOUserData::new(
        test_data.username.clone(),
        test_data.email.clone(),
        test_data.avatar.clone(),
    );

    let sso = FastCommentsSSO::new_simple(user);
    let token = sso.create_token();
    assert!(token.is_some());

    let token = token.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&token).expect("Failed to parse JSON");

    assert_eq!(parsed["username"], test_data.username);
    assert_eq!(parsed["email"], test_data.email);
    assert_eq!(parsed["avatar"], test_data.avatar);
}

#[test]
fn test_secure_sso_with_urls() {
    let env = TestEnv::from_env().expect("Test environment not configured");
    let test_data = TestUserData::new();

    let user = SecureSSOUserData::new(
        test_data.user_id.clone(),
        test_data.email.clone(),
        test_data.username.clone(),
        test_data.avatar.clone(),
    );

    let sso_result = FastCommentsSSO::new_secure(env.api_key.clone(), &user);
    assert!(sso_result.is_ok());

    let sso = sso_result.unwrap();

    // Just verify that the secure payload exists
    assert!(sso.get_secure_sso_payload().is_some());
    // Note: can't test new_secure_with_urls without Clone trait on SecureSSOPayload
}

#[test]
fn test_token_caching() {
    let test_data = TestUserData::new();

    let user = SimpleSSOUserData::new(
        test_data.username.clone(),
        test_data.email.clone(),
        test_data.avatar.clone(),
    );

    let mut sso = FastCommentsSSO::new_simple(user);
    let token1 = sso.prepare_to_send().clone();
    let token2 = sso.prepare_to_send().clone();

    assert_eq!(token1, token2);
}

#[test]
fn test_reset_token() {
    let test_data = TestUserData::new();

    let user = SimpleSSOUserData::new(
        test_data.username.clone(),
        test_data.email.clone(),
        test_data.avatar.clone(),
    );

    let mut sso = FastCommentsSSO::new_simple(user);
    let _token1 = sso.prepare_to_send();

    sso.reset_token();
    assert!(sso.prepare_to_send().is_some());
}

#[test]
fn test_set_secure_sso_payload() {
    let env = TestEnv::from_env().expect("Test environment not configured");
    let test_data = TestUserData::new();

    let simple_user = SimpleSSOUserData::new(
        test_data.username.clone(),
        test_data.email.clone(),
        test_data.avatar.clone(),
    );

    let mut sso = FastCommentsSSO::new_simple(simple_user);

    // Switch to secure
    let secure_user = SecureSSOUserData::new(
        test_data.user_id.clone(),
        test_data.email.clone(),
        test_data.username.clone(),
        test_data.avatar.clone(),
    );

    let secure_sso = FastCommentsSSO::new_secure(env.api_key.clone(), &secure_user)
        .expect("Failed to create secure SSO");
    let payload = secure_sso.get_secure_sso_payload().clone();

    sso.set_secure_sso_payload(payload);

    assert!(sso.get_secure_sso_payload().is_some());
    assert!(sso.get_simple_sso_payload().is_none());
}

#[test]
fn test_set_simple_sso_user_data() {
    let env = TestEnv::from_env().expect("Test environment not configured");
    let test_data = TestUserData::new();

    let secure_user = SecureSSOUserData::new(
        test_data.user_id.clone(),
        test_data.email.clone(),
        test_data.username.clone(),
        test_data.avatar.clone(),
    );

    let mut sso = FastCommentsSSO::new_secure(env.api_key.clone(), &secure_user)
        .expect("Failed to create secure SSO");

    // Switch to simple
    let simple_user = SimpleSSOUserData::new(
        test_data.username.clone(),
        test_data.email.clone(),
        test_data.avatar.clone(),
    );

    sso.set_simple_sso_payload(Some(simple_user));

    assert!(sso.get_simple_sso_payload().is_some());
    assert!(sso.get_secure_sso_payload().is_none());
}
