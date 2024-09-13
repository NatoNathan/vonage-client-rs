use super::{ConnectOptions, ConnectType, Endpoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SipConnect {
    pub endpoint: Vec<SipEndpoint>,
    #[serde(flatten)]
    pub connect_options: ConnectOptions,
}

impl ConnectType<SipEndpoint> for SipConnect {
    fn with_endpoint(&mut self, endpoint: impl FnOnce(&mut SipEndpoint)) -> &mut Self {
        endpoint(&mut self.endpoint[0]);
        self
    }
    fn with_options(&mut self, options: impl FnOnce(&mut ConnectOptions)) -> &mut Self {
        options(&mut self.connect_options);
        self
    }
    fn options(&mut self) -> &mut ConnectOptions {
        &mut self.connect_options
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", rename = "sip")]
pub struct SipEndpoint {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SipHeaders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_headers: Option<SipStandardHeaders>,
}
impl Endpoint for SipEndpoint {}
impl SipEndpoint {
    pub fn headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn user_to_user(&mut self, user_to_user: String) -> &mut Self {
        self.standard_headers = Some(SipStandardHeaders { user_to_user });
        self
    }
}

pub type SipHeaders = HashMap<String, String>;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct SipStandardHeaders {
    #[serde(rename = "User-to-User")]
    user_to_user: String,
}
