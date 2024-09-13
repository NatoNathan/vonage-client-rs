mod app;
mod phone;
mod sip;
mod vbc;
mod websocket;

use super::{EventMethod, EventType};
use serde::{Deserialize, Serialize};

pub use app::*;
pub use phone::*;
pub use sip::*;
pub use vbc::*;
pub use websocket::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Connect {
    Phone(PhoneConnect),
    App(AppConnect),
    Websocket(WebsocketConnect),
    Sip(SipConnect),
    Vbc(VbcConnect),
}

pub trait Endpoint {}

pub trait ConnectType<E: Endpoint> {
    fn with_endpoint(&mut self, endpoint: impl FnOnce(&mut E)) -> &mut Self;
    fn with_options(&mut self, options: impl FnOnce(&mut ConnectOptions)) -> &mut Self;
    fn options(&mut self) -> &mut ConnectOptions;
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_from_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_detection: Option<MachineDetection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_machine_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_method: Option<EventMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ringback_tone: Option<String>,
}

impl ConnectOptions {
    pub fn from(&mut self, from: String) -> &mut Self {
        self.from = Some(from);
        self
    }

    pub fn random_from_number(&mut self, random_from_number: bool) -> &mut Self {
        self.random_from_number = Some(random_from_number);
        self
    }

    pub fn event_type(&mut self, event_type: EventType) -> &mut Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn machine_detection(&mut self, machine_detection: MachineDetection) -> &mut Self {
        self.machine_detection = Some(machine_detection);
        self
    }

    pub fn advanced_machine_detection(&mut self, advanced_machine_detection: bool) -> &mut Self {
        self.advanced_machine_detection = Some(advanced_machine_detection);
        self
    }

    pub fn event_url(&mut self, event_url: String) -> &mut Self {
        self.event_url = Some(event_url);
        self
    }

    pub fn event_method(&mut self, event_method: EventMethod) -> &mut Self {
        self.event_method = Some(event_method);
        self
    }

    pub fn ringback_tone(&mut self, ringback_tone: String) -> &mut Self {
        self.ringback_tone = Some(ringback_tone);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MachineDetection {
    Continue,
    Hangup,
}
