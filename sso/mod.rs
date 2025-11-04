use hmac::digest::InvalidLength;

pub mod fastcomments_sso;
pub mod secure_sso_payload;
pub mod secure_sso_user_data;
pub mod simple_sso_user_data;
pub mod helpers;

#[derive(Debug)]
pub enum CreateHashError {
    InvalidLength(InvalidLength),
}

impl std::fmt::Display for CreateHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateHashError::InvalidLength(_) => write!(f, "Error initializing HMAC"),
        }
    }
}

impl std::error::Error for CreateHashError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CreateHashError::InvalidLength(e) => Some(e),
        }
    }
}

impl From<InvalidLength> for CreateHashError {
    fn from(err: InvalidLength) -> Self {
        CreateHashError::InvalidLength(err)
    }
}