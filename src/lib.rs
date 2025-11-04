//! FastComments Rust SDK
//!
//! This SDK provides access to the FastComments API and includes SSO functionality.

// Re-export SSO functionality
#[path = "../sso/mod.rs"]
pub mod sso;

// Re-export generated API client
#[path = "../client/src/lib.rs"]
pub mod client;

// Re-export for convenience
pub use sso::fastcomments_sso::FastCommentsSSO;
pub use sso::secure_sso_payload::SecureSSOPayload;
pub use sso::secure_sso_user_data::SecureSSOUserData;
pub use sso::simple_sso_user_data::SimpleSSOUserData;
