use serde_json::json;
use serde_json::Value;

use super::req::Req;
use super::resp::Resp;

#[derive(Default)]
pub struct Client {
    pub auth_token: Option<String>,
    pub url: Option<String>,
    pub un: Option<String>,
    pw: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn req(&self, method: &str, params: Value) -> Resp {
        Req::new(method.into(), params, self.auth_token.clone())
            .send(&self.url.clone().expect("Client URL is not set"))
    }

    pub fn login(&mut self) -> Resp {
        let mut resp = self.req(
            "user.login",
            json!({
                "user": self.un,
                "password": self.pw,
            }),
        );

        match resp.result() {
            Ok(value) => {
                self.auth_token.replace(value.as_str().unwrap().into());},
            Err(_) => {},
        };

        resp
    }

    pub fn logout(&mut self) -> Resp {
        let mut resp = self.req(
            "user.logout",
            json!([])
        );

        match resp.result() {
            Ok(value) => {
                if value.as_bool().unwrap().into() {
                    self.auth_token = None;
                }
            },
            Err(_) => {},
        };

        resp
    }

    pub fn set_credentials(&mut self, un: &str, pw: &str) {
        self.un.replace(un.into());
        self.pw.replace(pw.into());
    }

    pub fn set_url(&mut self, url: &str) {
        self.url.replace(url.into());
    }

    pub fn is_authorized(&self) -> bool {
        self.auth_token.is_some()
    }
}
