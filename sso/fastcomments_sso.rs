use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    helpers::create_verification_hash, secure_sso_payload::SecureSSOPayload,
    secure_sso_user_data::SecureSSOUserData, simple_sso_user_data::SimpleSSOUserData,
    CreateHashError,
};

pub type LoginLogoutCallback = dyn Fn(&str);

pub struct FastCommentsSSO {
    secure_sso_payload: Option<SecureSSOPayload>,
    simple_sso_user_data: Option<SimpleSSOUserData>,
    cached_token: Option<String>,
    pub login_url: Option<String>,
    pub logout_url: Option<String>,
    pub login_callback: Option<Box<LoginLogoutCallback>>,
    pub logout_callback: Option<Box<LoginLogoutCallback>>,
}

impl FastCommentsSSO {
    pub fn new(
        secure_sso_payload: Option<SecureSSOPayload>,
        simple_sso_user_data: Option<SimpleSSOUserData>,
    ) -> FastCommentsSSO {
        FastCommentsSSO {
            secure_sso_payload,
            simple_sso_user_data,
            cached_token: None,
            login_url: None,
            logout_url: None,
            login_callback: None,
            logout_callback: None,
        }
    }

    pub fn new_secure(
        api_key: String,
        secure_sso_user_data: &SecureSSOUserData,
    ) -> Result<Self, CreateHashError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let user_data_string = secure_sso_user_data.as_json_base64();

        // Create verification hash
        let hash = create_verification_hash(&api_key, timestamp, &user_data_string)?;

        let payload = SecureSSOPayload::new(user_data_string, hash, timestamp);

        Ok(FastCommentsSSO::new(Some(payload), None))
    }

    pub fn new_simple(simple_sso_user_data: SimpleSSOUserData) -> Self {
        FastCommentsSSO::new(None, Some(simple_sso_user_data))
    }

    pub fn new_secure_with_urls(
        secure_sso_payload: SecureSSOPayload,
        login_url: String,
        logout_url: String,
    ) -> Self {
        FastCommentsSSO {
            secure_sso_payload: Some(secure_sso_payload),
            simple_sso_user_data: None,
            cached_token: None,
            login_url: Some(login_url),
            logout_url: Some(logout_url),
            login_callback: None,
            logout_callback: None,
        }
    }

    pub fn new_simple_with_callbacks(
        simple_sso_user_data: SimpleSSOUserData,
        login_callback: Box<LoginLogoutCallback>,
        logout_callback: Box<LoginLogoutCallback>,
    ) -> Self {
        FastCommentsSSO {
            secure_sso_payload: None,
            simple_sso_user_data: Some(simple_sso_user_data),
            cached_token: None,
            login_url: None,
            logout_url: None,
            login_callback: Some(login_callback),
            logout_callback: Some(logout_callback),
        }
    }

    pub fn create_token(&self) -> Option<String> {
        if let Some(secure_sso) = &self.secure_sso_payload {
            let json = serde_json::to_string(&secure_sso).ok();
            return json;
        }

        if let Some(simple_sso) = &self.simple_sso_user_data {
            let json = serde_json::to_string(&simple_sso).ok();
            return json;
        }

        None
    }

    pub fn reset_token(&mut self) {
        self.cached_token = None;
    }

    pub fn prepare_to_send(&mut self) -> &Option<String> {
        match &self.cached_token {
            Some(_) => &self.cached_token,
            None => {
                self.cached_token = self.create_token();
                &self.cached_token
            }
        }
    }

    pub fn set_secure_sso_payload(&mut self, secure_sso_payload: Option<SecureSSOPayload>) {
        self.secure_sso_payload = secure_sso_payload;
        self.simple_sso_user_data = None;
        self.reset_token();
    }

    pub fn set_simple_sso_payload(&mut self, simple_soo_user_data: Option<SimpleSSOUserData>) {
        self.simple_sso_user_data = simple_soo_user_data;
        self.secure_sso_payload = None;
        self.reset_token();
    }

    pub fn get_secure_sso_payload(&self) -> &Option<SecureSSOPayload> {
        &self.secure_sso_payload
    }

    pub fn get_simple_sso_payload(&self) -> &Option<SimpleSSOUserData> {
        &self.simple_sso_user_data
    }
}
