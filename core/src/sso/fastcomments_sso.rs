use super::{secure_sso_payload::SecureSSOPayload, simple_sso_user_data::SimpleSSOUserData};

pub struct FastCommentsSSO {
    secure_sso_payload: Option<SecureSSOPayload>,
    simple_sso_user_data: Option<SimpleSSOUserData>,
    cached_token: Option<String>,
    pub login_url: Option<String>,
    pub logout_url: Option<String>,
}

trait FastCommentsSSOTrait {
    fn login_callback(&self, username: String);
    fn logout_callback(&self, username: String);
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
        self.reset_token();
    }

    pub fn set_simple_sso_payload(&mut self, simple_soo_user_data: Option<SimpleSSOUserData>) {
        self.simple_sso_user_data = simple_soo_user_data;
        self.reset_token();
    }

    pub fn get_secure_sso_payload(&self) -> &Option<SecureSSOPayload> {
        &self.secure_sso_payload
    }

    pub fn get_simple_sso_payload(&self) -> &Option<SimpleSSOUserData> {
        &self.simple_sso_user_data
    }
}
