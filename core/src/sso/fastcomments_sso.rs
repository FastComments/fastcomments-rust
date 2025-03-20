use super::{secure_sso_payload::SecureSSOPayload, simple_sso_user_data::SimpleSSOUserData};

pub struct FastCommentsSSO {
    secure_sso_payload: Option<SecureSSOPayload>,
    simple_sso_user_data: Option<SimpleSSOUserData>,
    cached_token: Option<String>,
    pub login_url: String,
    pub logout_url: String,
}

trait FastCommentsSSOTrait {
    fn login_callback(&self, username: String);
    fn logout_callback(&self, username: String);
}

impl FastCommentsSSO {
    pub fn new(
        secure_sso_payload: Option<SecureSSOPayload>,
        simple_soo_user_data: Option<SimpleSSOUserData>,
        cached_token: Option<String>,
        login_url: String,
        logout_url: String,
    ) -> FastCommentsSSO {
        FastCommentsSSO {
            secure_sso_payload,
            simple_sso_user_data: simple_soo_user_data,
            cached_token,
            login_url,
            logout_url,
        }
    }

    pub fn secure_sso(&mut self, secure_sso_payload: Option<SecureSSOPayload>) {
        self.secure_sso_payload = secure_sso_payload;
        self.simple_sso_user_data = None
    }

    pub fn simple_sso(&mut self, simple_soo_user_data: Option<SimpleSSOUserData>) {
        self.secure_sso_payload = None;
        self.simple_sso_user_data = simple_soo_user_data;
    }

    pub fn create_token(&self) -> Option<String> {
        if let Some(secure_sso) = &self.secure_sso_payload {
            let json = serde_json::to_string(&secure_sso).ok();
            return json
        }

        if let Some(simple_sso) = &self.simple_sso_user_data {
            let json = serde_json::to_string(&simple_sso).ok();
            return json
        }

        None
    }

    pub fn set_secure_sso_payload(&mut self, secure_sso_payload: Option<SecureSSOPayload>) {
        self.secure_sso_payload = secure_sso_payload;
        self.reset_token();
    }

    pub fn set_simple_sso_payload(&mut self, simple_soo_user_data: Option<SimpleSSOUserData>) {
        self.simple_sso_user_data = simple_soo_user_data;
        self.reset_token();
    }

    pub fn reset_token(&mut self) {
        self.cached_token = None;
    }
}