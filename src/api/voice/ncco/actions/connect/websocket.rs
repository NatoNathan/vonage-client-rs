use super::{ConnectOptions, ConnectType, Endpoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WebsocketConnect {
    pub endpoint: Vec<WebsocketEndpoint>,
    #[serde(flatten)]
    pub connect_options: ConnectOptions,
}

impl ConnectType<WebsocketEndpoint> for WebsocketConnect {
    fn with_endpoint(&mut self, endpoint: impl FnOnce(&mut WebsocketEndpoint)) -> &mut Self {
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
#[serde(rename_all = "camelCase", tag = "type", rename = "websocket")]
pub struct WebsocketEndpoint {
    pub uri: String,
    pub content_type: AudioFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<WebsocketHeaders>,
}
impl Endpoint for WebsocketEndpoint {}
impl WebsocketEndpoint {
    pub fn headers(&mut self, headers: WebsocketHeaders) -> &mut Self {
        self.headers = Some(headers);
        self
    }
}

pub type WebsocketHeaders = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AudioFormat {
    #[serde(rename = "audio/l16;rate=16000")]
    L16_16K,
    #[serde(rename = "audio/l16;rate=8000")]
    L16_8K,
}

impl Default for AudioFormat {
    fn default() -> Self {
        AudioFormat::L16_16K
    }
}
