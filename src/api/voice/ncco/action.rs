use serde::{Deserialize, Serialize};

use super::actions::{Connect, Conversation, Talk};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum Action {
    Talk(Talk),
    Connect(Connect),
    Conversation(Conversation),
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventMethod {
    GET,
    POST,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Synchronous,
}
