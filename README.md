# FastComments Rust SDK

The official Rust SDK for FastComments, a fast and developer-friendly commenting platform. This SDK provides typed API clients and utilities for integrating FastComments into your Rust applications.

## Installation

```bash
cargo add fastcomments-sdk
```

The SDK requires Rust 2021 edition or later.

## Library Contents

The FastComments Rust SDK consists of several modules:

- **Client Module** - Auto-generated API client for FastComments REST APIs
  - Complete type definitions for all API models
  - Both authenticated (`DefaultApi`) and public (`PublicApi`) endpoints
  - Full async/await support with tokio
  - See [client/README.md](client/README.md) for detailed API documentation

- **SSO Module** - Server-side Single Sign-On utilities
  - Secure token generation for user authentication
  - Support for both simple and secure SSO modes
  - HMAC-SHA256 based token signing

- **Core Types** - Shared type definitions and utilities
  - Comment models and metadata structures
  - User and tenant configurations
  - Helper functions for common operations

## Quick Start

### Using the Public API

```rust
use fastcomments_sdk::client::apis::configuration::Configuration;
use fastcomments_sdk::client::apis::public_api;

#[tokio::main]
async fn main() {
    // Create API configuration
    let config = Configuration::new();

    // Fetch comments for a page
    let result = public_api::get_comments_public(
        &config,
        public_api::GetCommentsPublicParams {
            tenant_id: "your-tenant-id".to_string(),
            urlid: Some("page-url-id".to_string()),
            url: None,
            count_only: None,
            skip: None,
            limit: None,
            sort_dir: None,
            page: None,
            sso_hash: None,
            simple_sso_hash: None,
            has_no_comment: None,
            has_comment: None,
            comment_id_filter: None,
            child_ids: None,
            start_date_time: None,
            starts_with: None,
        },
    )
    .await;

    match result {
        Ok(response) => {
            println!("Found {} comments", response.comments.len());
            for comment in response.comments {
                println!("Comment: {:?}", comment);
            }
        }
        Err(e) => eprintln!("Error fetching comments: {:?}", e),
    }
}
```

### Using the Authenticated API

```rust
use fastcomments_sdk::client::apis::configuration::{ApiKey, Configuration};
use fastcomments_sdk::client::apis::default_api;

#[tokio::main]
async fn main() {
    // Create configuration with API key
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: None,
        key: "your-api-key".to_string(),
    });

    // Fetch comments using authenticated API
    let result = default_api::get_comments(
        &config,
        default_api::GetCommentsParams {
            tenant_id: "your-tenant-id".to_string(),
            skip: None,
            limit: None,
            sort_dir: None,
            urlid: Some("page-url-id".to_string()),
            url: None,
            is_spam: None,
            user_id: None,
            all_comments: None,
            for_moderation: None,
            parent_id: None,
            is_flagged: None,
            is_flagged_tag: None,
            is_by_verified: None,
            is_pinned: None,
            asc: None,
            include_imported: None,
            origin: None,
            tags: None,
        },
    )
    .await;

    match result {
        Ok(response) => {
            println!("Total comments: {}", response.count);
            for comment in response.comments {
                println!("Comment ID: {}, Text: {}", comment.id, comment.comment);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Using SSO for Authentication

```rust
use fastcomments_sdk::sso::{
    fastcomments_sso::FastCommentsSSO,
    secure_sso_user_data::SecureSSOUserData,
};

fn main() {
    let api_key = "your-api-key".to_string();

    // Create secure SSO user data (server-side only!)
    let user_data = SecureSSOUserData::new(
        "user-123".to_string(),           // User ID
        "user@example.com".to_string(),   // Email
        "John Doe".to_string(),            // Username
        "https://example.com/avatar.jpg".to_string(), // Avatar URL
    );

    // Generate SSO token
    let sso = FastCommentsSSO::new_secure(api_key, &user_data).unwrap();
    let token = sso.create_token().unwrap();

    println!("SSO Token: {}", token);
    // Pass this token to your frontend for authentication
}
```

## Common Issues

### 401 Unauthorized Errors

If you're getting 401 errors when using the authenticated API:

1. **Check your API key**: Ensure you're using the correct API key from your FastComments dashboard
2. **Verify the tenant ID**: Make sure the tenant ID matches your account
3. **API key format**: The API key should be passed in the Configuration:

```rust
let mut config = Configuration::new();
config.api_key = Some(ApiKey {
    prefix: None,
    key: "YOUR_API_KEY".to_string(),
});
```

### SSO Token Issues

If SSO tokens aren't working:

1. **Use secure mode for production**: Always use `FastCommentsSSO::new_secure()` with your API key for production
2. **Server-side only**: Generate SSO tokens on your server, never expose your API key to clients
3. **Check user data**: Ensure all required fields (id, email, username) are provided

### Async Runtime Errors

The SDK uses tokio for async operations. Make sure to:

1. Add tokio to your dependencies:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

2. Use the tokio runtime:
```rust
#[tokio::main]
async fn main() {
    // Your async code here
}
```

## Notes

### Broadcast IDs

You'll see you're supposed to pass a `broadcastId` in some API calls. When you receive events, you'll get this ID back, so you know to ignore the event if you plan to optimistically apply changes on the client
(which you'll probably want to do since it offers the best experience). Pass a UUID here. The ID should be unique enough to not occur twice in a browser session.

## Support

For issues, questions, or feature requests:

- GitHub Issues: [https://github.com/fastcomments/fastcomments-rust](https://github.com/fastcomments/fastcomments-rust)
- Documentation: [https://docs.fastcomments.com](https://docs.fastcomments.com)
- Support: support@fastcomments.com

## License

MIT - See [LICENSE](LICENSE) file for details.
