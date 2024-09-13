use super::{ConnectOptions, ConnectType, Endpoint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PhoneConnect {
    pub(crate) endpoint: Vec<PhoneEndpoint>,
    #[serde(flatten)]
    pub connect_options: ConnectOptions,
}

impl ConnectType<PhoneEndpoint> for PhoneConnect {
    fn with_endpoint(&mut self, endpoint: impl FnOnce(&mut PhoneEndpoint)) -> &mut Self {
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
#[serde(rename_all = "camelCase", tag = "type", rename = "phone")]
pub struct PhoneEndpoint {
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_answer: Option<OnAnswer>,
}

impl Endpoint for PhoneEndpoint {}
impl PhoneEndpoint {
    pub fn dtmf_answer(&mut self, dtmf_answer: String) -> &mut Self {
        self.dtmf_answer = Some(dtmf_answer);
        self
    }

    pub fn on_answer(&mut self, on_answer: OnAnswer) -> &mut Self {
        self.on_answer = Some(on_answer);
        self
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnAnswer {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ringback_tone: Option<String>,
}
