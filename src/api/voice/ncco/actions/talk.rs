use super::EventMethod;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Talk {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barge_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "loop")]
    pub loop_times: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_on_completion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_method: Option<EventMethod>,
}

impl Talk {
    pub fn barge_in(&mut self, barge_in: bool) -> &mut Self {
        self.barge_in = Some(barge_in);
        self
    }

    pub fn loop_times(&mut self, loop_times: usize) -> &mut Self {
        self.loop_times = Some(loop_times);
        self
    }

    pub fn level(&mut self, level: i8) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn language(&mut self, language: String) -> &mut Self {
        self.language = Some(language);
        self
    }

    pub fn style(&mut self, style: usize) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn premium(&mut self, premium: bool) -> &mut Self {
        self.premium = Some(premium);
        self
    }

    pub fn event_on_completion(&mut self, event_on_completion: bool) -> &mut Self {
        self.event_on_completion = Some(event_on_completion);
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
}
